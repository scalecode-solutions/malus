//! CFArray — CoreFoundation immutable and mutable arrays.

use super::base::*;
use std::ffi::c_void;
use std::fmt;

pub type CFArrayRef = *const __CFArray;
pub type CFMutableArrayRef = *mut __CFArray;

// Callback struct — we only ever use kCFTypeArrayCallBacks, so the fields are
// declared but never accessed from Rust. Layout must match the C struct.
#[repr(C)]
pub struct CFArrayCallBacks {
    version: CFIndex,
    retain: Option<unsafe extern "C" fn(CFAllocatorRef, *const c_void) -> *const c_void>,
    release: Option<unsafe extern "C" fn(CFAllocatorRef, *const c_void)>,
    copy_description: Option<unsafe extern "C" fn(*const c_void) -> *const __CFString>,
    equal: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> Boolean>,
}

extern "C" {
    static kCFTypeArrayCallBacks: CFArrayCallBacks;

    fn CFArrayCreate(
        allocator: CFAllocatorRef,
        values: *const *const c_void,
        num_values: CFIndex,
        call_backs: *const CFArrayCallBacks,
    ) -> CFArrayRef;

    fn CFArrayCreateMutable(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        call_backs: *const CFArrayCallBacks,
    ) -> CFMutableArrayRef;

    fn CFArrayGetCount(array: CFArrayRef) -> CFIndex;
    fn CFArrayGetValueAtIndex(array: CFArrayRef, idx: CFIndex) -> *const c_void;
    fn CFArrayAppendValue(array: CFMutableArrayRef, value: *const c_void);
}

// ============================================================================
// CFArray — immutable
// ============================================================================

pub struct CFArray(CFRef<__CFArray>);

impl CFArray {
    /// Create a CFArray from a slice of CFTypeRef pointers.
    /// The array retains all values.
    pub fn from_type_refs(values: &[CFTypeRef]) -> Self {
        unsafe {
            let raw = CFArrayCreate(
                std::ptr::null(),
                values.as_ptr() as *const *const c_void,
                values.len() as CFIndex,
                &kCFTypeArrayCallBacks,
            );
            Self(CFRef::wrap(raw).expect("CFArrayCreate returned null"))
        }
    }

    pub fn len(&self) -> usize {
        unsafe { CFArrayGetCount(self.as_raw()) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Get the raw CFTypeRef at an index. Returns null if out of bounds.
    pub fn get(&self, idx: usize) -> CFTypeRef {
        if idx >= self.len() {
            return std::ptr::null();
        }
        unsafe { CFArrayGetValueAtIndex(self.as_raw(), idx as CFIndex) }
    }

    /// Wrap a CFArrayRef you own (from a Create/Copy function).
    #[inline]
    pub unsafe fn wrap(ptr: CFArrayRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    /// Wrap a borrowed CFArrayRef (from a Get function) by retaining it.
    #[inline]
    pub unsafe fn retain(ptr: CFArrayRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CFArrayRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CFArray {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CFArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CFArray(len={})", self.len())
    }
}

// ============================================================================
// CFMutableArray
// ============================================================================

pub struct CFMutableArray(CFRef<__CFArray>);

impl CFMutableArray {
    /// Create an empty mutable array.
    pub fn new() -> Self {
        unsafe {
            let raw = CFArrayCreateMutable(
                std::ptr::null(),
                0, // 0 = no limit
                &kCFTypeArrayCallBacks,
            );
            Self(CFRef::wrap(raw as CFArrayRef).expect("CFArrayCreateMutable returned null"))
        }
    }

    /// Append a CFTypeRef value. The array retains it.
    pub fn push(&mut self, value: CFTypeRef) {
        unsafe {
            CFArrayAppendValue(self.as_raw_mut(), value);
        }
    }

    pub fn len(&self) -> usize {
        unsafe { CFArrayGetCount(self.as_raw() as CFArrayRef) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, idx: usize) -> CFTypeRef {
        if idx >= self.len() {
            return std::ptr::null();
        }
        unsafe { CFArrayGetValueAtIndex(self.as_raw() as CFArrayRef, idx as CFIndex) }
    }

    #[inline]
    fn as_raw_mut(&self) -> CFMutableArrayRef {
        self.0.as_ptr() as CFMutableArrayRef
    }

    #[inline]
    pub fn as_raw(&self) -> CFArrayRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Default for CFMutableArray {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for CFMutableArray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CFMutableArray(len={})", self.len())
    }
}
