//! Low-level ObjC runtime helpers.
//!
//! This module wraps objc-sys into ergonomic, safe-ish Rust primitives.
//! Everything here is internal — users never see it.

pub use objc_sys::*;
use std::ffi::{CStr, CString, c_void};
use std::os::raw::c_char;

// ============================================================================
// Type aliases matching the runtime
// ============================================================================

pub type Sel = SEL;
#[allow(dead_code)]
pub type Class = *const objc_class;
pub type Id = *mut objc_object;

// ============================================================================
// Convenience macros
// ============================================================================

/// Register (or fetch) an ObjC selector at runtime.
macro_rules! sel {
    ($name:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            objc_sys::sel_registerName(
                concat!($name, "\0").as_ptr() as *const std::os::raw::c_char,
            )
        }
    }};
}
#[allow(unused_imports)]
pub(crate) use sel;

/// Fetch an ObjC class by name.
macro_rules! cls {
    ($name:expr) => {{
        #[allow(unused_unsafe)]
        unsafe {
            objc_sys::objc_getClass(
                concat!($name, "\0").as_ptr() as *const std::os::raw::c_char,
            )
        }
    }};
}
#[allow(unused_imports)]
pub(crate) use cls;

/// Send an ObjC message — caller provides the C function signature via turbofish.
///
/// `msg_send!(obj, sel!("init"), fn(Id, Sel) -> Id)`
macro_rules! msg_send {
    ($obj:expr, $sel:expr, fn( $($arg_ty:ty),* ) -> $ret:ty) => {{
        let f: unsafe extern "C" fn( $($arg_ty),* ) -> $ret =
            core::mem::transmute($crate::runtime::msg_send_fn());
        f($obj, $sel)
    }};
    ($obj:expr, $sel:expr, fn( $($arg_ty:ty),* ) -> $ret:ty, $($arg:expr),+) => {{
        let f: unsafe extern "C" fn( $($arg_ty),* ) -> $ret =
            core::mem::transmute($crate::runtime::msg_send_fn());
        f($obj, $sel, $($arg),+)
    }};
}
#[allow(unused_imports)]
pub(crate) use msg_send;

// ============================================================================
// msg_send function pointer
// ============================================================================

#[inline(always)]
pub fn msg_send_fn() -> unsafe extern "C" fn() {
    objc_msgSend
}

// ============================================================================
// BOOL helpers (platform-dependent: bool on ARM64, i8 on x86_64)
// ============================================================================

#[inline(always)]
pub fn yes() -> BOOL {
    #[cfg(target_arch = "aarch64")]
    { true }
    #[cfg(not(target_arch = "aarch64"))]
    { 1 }
}

#[inline(always)]
pub fn no() -> BOOL {
    #[cfg(target_arch = "aarch64")]
    { false }
    #[cfg(not(target_arch = "aarch64"))]
    { 0 }
}

#[inline(always)]
pub fn to_bool(b: BOOL) -> bool {
    #[cfg(target_arch = "aarch64")]
    { b }
    #[cfg(not(target_arch = "aarch64"))]
    { b != 0 }
}

#[inline(always)]
pub fn from_bool(b: bool) -> BOOL {
    if b { yes() } else { no() }
}

// ============================================================================
// NSString <-> &str
// ============================================================================

/// Create an autoreleased `NSString` from a Rust string.
///
/// Uses `initWithBytes:length:encoding:` instead of `stringWithUTF8String:`
/// because Rust `&str` is NOT null-terminated. Using `stringWithUTF8String:`
/// would read past the string boundary into adjacent memory.
pub unsafe fn nsstring(s: &str) -> Id {
    let cls = cls!("NSString");
    let alloc: Id = msg_send!(cls as Id, sel!("alloc"), fn(Id, Sel) -> Id);
    let utf8_encoding: NSUInteger = 4; // NSUTF8StringEncoding
    msg_send!(
        alloc,
        sel!("initWithBytes:length:encoding:"),
        fn(Id, Sel, *const c_char, NSUInteger, NSUInteger) -> Id,
        s.as_ptr() as *const c_char,
        s.len() as NSUInteger,
        utf8_encoding
    )
}

/// Read an `NSString` into a Rust `String`. Returns empty string on null.
pub unsafe fn from_nsstring(obj: Id) -> String {
    if obj.is_null() {
        return String::new();
    }
    let sel = sel!("UTF8String");
    let ptr: *const c_char = msg_send!(obj, sel, fn(Id, Sel) -> *const c_char);
    if ptr.is_null() {
        String::new()
    } else {
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}

// ============================================================================
// alloc + init
// ============================================================================

pub unsafe fn alloc(class: Id) -> Id {
    msg_send!(class, sel!("alloc"), fn(Id, Sel) -> Id)
}

pub unsafe fn init(obj: Id) -> Id {
    msg_send!(obj, sel!("init"), fn(Id, Sel) -> Id)
}

pub unsafe fn alloc_init(class: Id) -> Id {
    init(alloc(class))
}

pub unsafe fn retain(obj: Id) -> Id {
    msg_send!(obj, sel!("retain"), fn(Id, Sel) -> Id)
}

pub unsafe fn release(obj: Id) {
    msg_send!(obj, sel!("release"), fn(Id, Sel) -> ());
}

// ============================================================================
// CGFloat + geometry types
// ============================================================================

#[cfg(target_pointer_width = "64")]
pub type CGFloat = f64;
#[cfg(target_pointer_width = "32")]
pub type CGFloat = f32;

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct CGPoint {
    pub x: CGFloat,
    pub y: CGFloat,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct CGSize {
    pub width: CGFloat,
    pub height: CGFloat,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct CGRect {
    pub origin: CGPoint,
    pub size: CGSize,
}

impl CGRect {
    pub fn new(x: CGFloat, y: CGFloat, w: CGFloat, h: CGFloat) -> Self {
        Self {
            origin: CGPoint { x, y },
            size: CGSize { width: w, height: h },
        }
    }
    pub fn zero() -> Self {
        Self::default()
    }
}

// ============================================================================
// NSUInteger / NSInteger
// ============================================================================

pub type NSUInteger = usize;
pub type NSInteger = isize;

// ============================================================================
// Dynamic class creation (for trampoline targets/delegates)
// ============================================================================

#[allow(dead_code)]
pub unsafe fn create_class(
    name: &str,
    superclass_name: &str,
) -> *mut objc_class {
    let super_cls = objc_getClass(
        CString::new(superclass_name).unwrap().as_ptr(),
    );
    let name_c = CString::new(name).unwrap();
    let cls = objc_allocateClassPair(super_cls as *mut objc_class, name_c.as_ptr(), 0);
    assert!(!cls.is_null(), "Failed to create class {name}");
    cls
}

#[allow(dead_code)]
pub unsafe fn register_class(cls: *mut objc_class) {
    objc_registerClassPair(cls);
}

#[allow(dead_code)]
pub unsafe fn add_method(
    cls: *mut objc_class,
    sel: Sel,
    imp: unsafe extern "C" fn(),
    types: &str,
) {
    let types_c = CString::new(types).unwrap();
    class_addMethod(cls, sel, Some(imp), types_c.as_ptr());
}

// ============================================================================
// Associated objects
// ============================================================================

/// Association policy for `set_associated_object`.
#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AssociationPolicy {
    /// Weak reference (OBJC_ASSOCIATION_ASSIGN).
    Assign = 0,
    /// Strong reference, non-atomic (OBJC_ASSOCIATION_RETAIN_NONATOMIC).
    RetainNonatomic = 1,
    /// Copy, non-atomic (OBJC_ASSOCIATION_COPY_NONATOMIC).
    CopyNonatomic = 3,
    /// Strong reference, atomic (OBJC_ASSOCIATION_RETAIN).
    Retain = 0x301,
    /// Copy, atomic (OBJC_ASSOCIATION_COPY).
    Copy = 0x303,
}

extern "C" {
    fn objc_setAssociatedObject(
        object: Id,
        key: *const c_void,
        value: Id,
        policy: usize,
    );
    fn objc_getAssociatedObject(
        object: Id,
        key: *const c_void,
    ) -> Id;
    fn objc_removeAssociatedObjects(object: Id);
}

/// Set an associated object on an ObjC object.
#[allow(dead_code)]
pub unsafe fn set_associated_object(
    object: Id,
    key: *const c_void,
    value: Id,
    policy: AssociationPolicy,
) {
    objc_setAssociatedObject(object, key, value, policy as usize);
}

/// Get an associated object from an ObjC object.
#[allow(dead_code)]
pub unsafe fn get_associated_object(object: Id, key: *const c_void) -> Id {
    objc_getAssociatedObject(object, key)
}

/// Remove all associated objects from an ObjC object.
#[allow(dead_code)]
pub unsafe fn remove_associated_objects(object: Id) {
    objc_removeAssociatedObjects(object);
}

// ============================================================================
// Instance variable helpers
// ============================================================================

extern "C" {
    fn class_addIvar(
        cls: *mut objc_class,
        name: *const c_char,
        size: usize,
        alignment: u8,
        types: *const c_char,
    ) -> BOOL;
    fn object_getInstanceVariable(
        obj: Id,
        name: *const c_char,
        out_value: *mut *mut c_void,
    ) -> Id;
    fn object_setInstanceVariable(
        obj: Id,
        name: *const c_char,
        value: *mut c_void,
    ) -> Id;
}

/// Add an ivar to a class (before registration).
#[allow(dead_code)]
pub unsafe fn add_ivar(cls: *mut objc_class, name: &str, size: usize, alignment: u8) {
    let name_c = CString::new(name).unwrap();
    let types_c = CString::new("^v").unwrap(); // void pointer
    class_addIvar(cls, name_c.as_ptr(), size, alignment, types_c.as_ptr());
}

/// Get a pointer-sized ivar value from an object.
#[allow(dead_code)]
pub unsafe fn get_ivar(obj: Id, name: &str) -> *mut c_void {
    let name_c = CString::new(name).unwrap();
    let mut value: *mut c_void = std::ptr::null_mut();
    object_getInstanceVariable(obj, name_c.as_ptr(), &mut value);
    value
}

/// Set a pointer-sized ivar value on an object.
#[allow(dead_code)]
pub unsafe fn set_ivar(obj: Id, name: &str, value: *mut c_void) {
    let name_c = CString::new(name).unwrap();
    object_setInstanceVariable(obj, name_c.as_ptr(), value);
}
