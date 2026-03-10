//! Event dispatch: closures stored by target ID, wired via ObjC trampolines.
//!
//! The pattern:
//! 1. User creates a `Button` with `.on_click(|| { ... })`
//! 2. We box the closure and insert it into a global map keyed by a unique ID
//! 3. We create (once) an ObjC class "MalusTrampoline" with a method `handleAction:`
//! 4. We allocate an instance of MalusTrampoline, store the callback ID as an ivar
//! 5. We set it as the NSButton's target, `handleAction:` as the action
//! 6. When the button is clicked, ObjC runtime calls handleAction: on our
//!    trampoline, which looks up the closure in the map and invokes it.

use crate::runtime::*;
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

// ============================================================================
// Global callback storage
// ============================================================================

type ActionCallback = Box<dyn Fn() + Send + 'static>;
type ChangeCallback = Box<dyn Fn(String) + Send + 'static>;

enum Callback {
    Action(ActionCallback),
    Change(ChangeCallback),
}

static NEXT_ID: AtomicUsize = AtomicUsize::new(1);

fn next_callback_id() -> usize {
    NEXT_ID.fetch_add(1, Ordering::Relaxed)
}

static CALLBACKS: std::sync::OnceLock<Mutex<HashMap<usize, Callback>>> =
    std::sync::OnceLock::new();

fn callbacks() -> &'static Mutex<HashMap<usize, Callback>> {
    CALLBACKS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub(crate) fn store_action(cb: impl Fn() + Send + 'static) -> usize {
    let id = next_callback_id();
    callbacks()
        .lock()
        .unwrap()
        .insert(id, Callback::Action(Box::new(cb)));
    id
}

pub(crate) fn store_change(cb: impl Fn(String) + Send + 'static) -> usize {
    let id = next_callback_id();
    callbacks()
        .lock()
        .unwrap()
        .insert(id, Callback::Change(Box::new(cb)));
    id
}

fn invoke_action(id: usize) {
    let map = callbacks().lock().unwrap();
    if let Some(Callback::Action(f)) = map.get(&id) {
        f();
    }
}

fn invoke_change(id: usize, value: String) {
    let map = callbacks().lock().unwrap();
    if let Some(Callback::Change(f)) = map.get(&id) {
        f(value);
    }
}

// ============================================================================
// ObjC trampoline class — registered once, instances created per-target
// ============================================================================

static TRAMPOLINE_REGISTERED: std::sync::Once = std::sync::Once::new();

/// Register the MalusTrampoline class if not already done.
fn ensure_trampoline_class() {
    TRAMPOLINE_REGISTERED.call_once(|| {
        unsafe {
            let cls = create_class("MalusTrampoline", "NSObject");

            // Add ivar: `callback_id` (usize, stored as pointer-sized int)
            let ok = objc_sys::class_addIvar(
                cls,
                c"callback_id".as_ptr(),
                std::mem::size_of::<usize>(),
                std::mem::align_of::<usize>().trailing_zeros() as u8,
                c"Q".as_ptr(), // unsigned long long encoding
            );
            assert!(ok != no(), "Failed to add callback_id ivar");

            // `handleAction:` — target-action callback
            add_method(
                cls,
                sel!("handleAction:"),
                std::mem::transmute::<
                    unsafe extern "C" fn(Id, Sel, Id),
                    unsafe extern "C" fn(),
                >(trampoline_handle_action),
                "v@:@",
            );

            // `controlTextDidChange:` — NSTextField notifications
            add_method(
                cls,
                sel!("controlTextDidChange:"),
                std::mem::transmute::<
                    unsafe extern "C" fn(Id, Sel, Id),
                    unsafe extern "C" fn(),
                >(trampoline_text_did_change),
                "v@:@",
            );

            register_class(cls);
        }
    });
}

/// C entry point for target-action (NSButton clicks, etc.)
unsafe extern "C" fn trampoline_handle_action(this: Id, _sel: Sel, _sender: Id) {
    let cb_id = get_callback_id(this);
    invoke_action(cb_id);
}

/// C entry point for NSTextField text-change notifications
unsafe extern "C" fn trampoline_text_did_change(this: Id, _sel: Sel, notification: Id) {
    let cb_id = get_callback_id(this);

    // notification.object → the NSTextField
    let text_field: Id = msg_send!(
        notification,
        sel!("object"),
        fn(Id, Sel) -> Id
    );

    // [textField stringValue]
    let ns_str: Id = msg_send!(
        text_field,
        sel!("stringValue"),
        fn(Id, Sel) -> Id
    );

    let value = from_nsstring(ns_str);
    invoke_change(cb_id, value);
}

// ============================================================================
// Ivar access helpers
// ============================================================================

unsafe fn get_callback_id(obj: Id) -> usize {
    let ivar = objc_sys::class_getInstanceVariable(
        objc_sys::object_getClass(obj) as *mut _,
        c"callback_id".as_ptr(),
    );
    let offset = objc_sys::ivar_getOffset(ivar);
    let ptr = (obj as *const u8).offset(offset) as *const usize;
    *ptr
}

unsafe fn set_callback_id(obj: Id, id: usize) {
    let ivar = objc_sys::class_getInstanceVariable(
        objc_sys::object_getClass(obj) as *mut _,
        c"callback_id".as_ptr(),
    );
    let offset = objc_sys::ivar_getOffset(ivar);
    let ptr = (obj as *mut u8).offset(offset) as *mut usize;
    *ptr = id;
}

// ============================================================================
// Public: create a trampoline instance wired to a callback
// ============================================================================

/// Create an MalusTrampoline instance with the given callback ID stored as an ivar.
pub(crate) unsafe fn make_action_target(callback_id: usize) -> Id {
    ensure_trampoline_class();
    let cls = cls!("MalusTrampoline");
    let obj = alloc_init(cls as Id);
    set_callback_id(obj, callback_id);
    obj
}

/// Wire an NSControl (NSButton, etc.) to a callback using target-action.
pub(crate) unsafe fn wire_action(control: Id, callback_id: usize) {
    let target = make_action_target(callback_id);

    let _: () = msg_send!(
        control,
        sel!("setTarget:"),
        fn(Id, Sel, Id) -> (),
        target
    );
    let _: () = msg_send!(
        control,
        sel!("setAction:"),
        fn(Id, Sel, Sel) -> (),
        sel!("handleAction:")
    );
}

/// Wire an NSTextField to a change callback using its delegate.
pub(crate) unsafe fn wire_text_change(text_field: Id, callback_id: usize) {
    let target = make_action_target(callback_id);

    let _: () = msg_send!(
        text_field,
        sel!("setDelegate:"),
        fn(Id, Sel, Id) -> (),
        target
    );
}
