//! Low-level ObjC runtime helpers.
//!
//! This module wraps objc-sys into ergonomic, safe-ish Rust primitives.
//! Everything here is internal — users never see it.

pub use objc_sys::*;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;

// ============================================================================
// Type aliases matching the runtime
// ============================================================================

pub type Sel = SEL;
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
pub unsafe fn nsstring(s: &str) -> Id {
    let cls = cls!("NSString");
    let bytes = s.as_ptr() as *const c_char;
    let len = s.len();
    let sel = sel!("stringWithUTF8String:");
    msg_send!(cls as Id, sel, fn(Id, Sel, *const c_char) -> Id, bytes)
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

pub unsafe fn register_class(cls: *mut objc_class) {
    objc_registerClassPair(cls);
}

pub unsafe fn add_method(
    cls: *mut objc_class,
    sel: Sel,
    imp: unsafe extern "C" fn(),
    types: &str,
) {
    let types_c = CString::new(types).unwrap();
    class_addMethod(cls, sel, Some(imp), types_c.as_ptr());
}
