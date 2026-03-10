//! CFDictionary — CoreFoundation immutable and mutable dictionaries.

use super::base::*;
use std::ffi::c_void;
use std::fmt;

pub type CFDictionaryRef = *const __CFDictionary;
pub type CFMutableDictionaryRef = *mut __CFDictionary;

// Callback structs — layout must match C, but we only use the kCFType* statics.

#[repr(C)]
pub struct CFDictionaryKeyCallBacks {
    version: CFIndex,
    retain: Option<unsafe extern "C" fn(CFAllocatorRef, *const c_void) -> *const c_void>,
    release: Option<unsafe extern "C" fn(CFAllocatorRef, *const c_void)>,
    copy_description: Option<unsafe extern "C" fn(*const c_void) -> *const __CFString>,
    equal: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> Boolean>,
    hash: Option<unsafe extern "C" fn(*const c_void) -> CFHashCode>,
}

#[repr(C)]
pub struct CFDictionaryValueCallBacks {
    version: CFIndex,
    retain: Option<unsafe extern "C" fn(CFAllocatorRef, *const c_void) -> *const c_void>,
    release: Option<unsafe extern "C" fn(CFAllocatorRef, *const c_void)>,
    copy_description: Option<unsafe extern "C" fn(*const c_void) -> *const __CFString>,
    equal: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> Boolean>,
}

extern "C" {
    static kCFTypeDictionaryKeyCallBacks: CFDictionaryKeyCallBacks;
    static kCFTypeDictionaryValueCallBacks: CFDictionaryValueCallBacks;

    fn CFDictionaryCreate(
        allocator: CFAllocatorRef,
        keys: *const *const c_void,
        values: *const *const c_void,
        num_values: CFIndex,
        key_call_backs: *const CFDictionaryKeyCallBacks,
        value_call_backs: *const CFDictionaryValueCallBacks,
    ) -> CFDictionaryRef;

    fn CFDictionaryCreateMutable(
        allocator: CFAllocatorRef,
        capacity: CFIndex,
        key_call_backs: *const CFDictionaryKeyCallBacks,
        value_call_backs: *const CFDictionaryValueCallBacks,
    ) -> CFMutableDictionaryRef;

    fn CFDictionaryGetCount(dict: CFDictionaryRef) -> CFIndex;
    fn CFDictionaryGetValue(dict: CFDictionaryRef, key: *const c_void) -> *const c_void;
    fn CFDictionaryContainsKey(dict: CFDictionaryRef, key: *const c_void) -> Boolean;
    fn CFDictionarySetValue(dict: CFMutableDictionaryRef, key: *const c_void, value: *const c_void);
    fn CFDictionaryRemoveValue(dict: CFMutableDictionaryRef, key: *const c_void);
}

// ============================================================================
// CFDictionary — immutable
// ============================================================================

pub struct CFDictionary(CFRef<__CFDictionary>);

impl CFDictionary {
    /// Create from parallel slices of keys and values.
    /// Both slices must have the same length.
    pub fn from_pairs(keys: &[CFTypeRef], values: &[CFTypeRef]) -> Self {
        assert_eq!(keys.len(), values.len(), "keys and values must have equal length");
        unsafe {
            let raw = CFDictionaryCreate(
                std::ptr::null(),
                keys.as_ptr() as *const *const c_void,
                values.as_ptr() as *const *const c_void,
                keys.len() as CFIndex,
                &kCFTypeDictionaryKeyCallBacks,
                &kCFTypeDictionaryValueCallBacks,
            );
            Self(CFRef::wrap(raw).expect("CFDictionaryCreate returned null"))
        }
    }

    pub fn len(&self) -> usize {
        unsafe { CFDictionaryGetCount(self.as_raw()) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Look up a value by key. Returns null if not found.
    pub fn get(&self, key: CFTypeRef) -> CFTypeRef {
        unsafe { CFDictionaryGetValue(self.as_raw(), key) }
    }

    pub fn contains_key(&self, key: CFTypeRef) -> bool {
        unsafe { CFDictionaryContainsKey(self.as_raw(), key) != 0 }
    }

    /// Wrap a CFDictionaryRef you own (from a Create/Copy function).
    #[inline]
    pub unsafe fn wrap(ptr: CFDictionaryRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    /// Wrap a borrowed CFDictionaryRef (from a Get function) by retaining it.
    #[inline]
    pub unsafe fn retain(ptr: CFDictionaryRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CFDictionaryRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CFDictionary {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CFDictionary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CFDictionary(len={})", self.len())
    }
}

// ============================================================================
// CFMutableDictionary
// ============================================================================

pub struct CFMutableDictionary(CFRef<__CFDictionary>);

impl CFMutableDictionary {
    /// Create an empty mutable dictionary.
    pub fn new() -> Self {
        unsafe {
            let raw = CFDictionaryCreateMutable(
                std::ptr::null(),
                0,
                &kCFTypeDictionaryKeyCallBacks,
                &kCFTypeDictionaryValueCallBacks,
            );
            Self(CFRef::wrap(raw as CFDictionaryRef).expect("CFDictionaryCreateMutable returned null"))
        }
    }

    /// Insert or replace a key-value pair. The dictionary retains both.
    pub fn set(&mut self, key: CFTypeRef, value: CFTypeRef) {
        unsafe {
            CFDictionarySetValue(self.as_raw_mut(), key, value);
        }
    }

    /// Remove a key-value pair.
    pub fn remove(&mut self, key: CFTypeRef) {
        unsafe {
            CFDictionaryRemoveValue(self.as_raw_mut(), key);
        }
    }

    pub fn len(&self) -> usize {
        unsafe { CFDictionaryGetCount(self.as_raw()) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get(&self, key: CFTypeRef) -> CFTypeRef {
        unsafe { CFDictionaryGetValue(self.as_raw(), key) }
    }

    pub fn contains_key(&self, key: CFTypeRef) -> bool {
        unsafe { CFDictionaryContainsKey(self.as_raw(), key) != 0 }
    }

    #[inline]
    fn as_raw_mut(&self) -> CFMutableDictionaryRef {
        self.0.as_ptr() as CFMutableDictionaryRef
    }

    #[inline]
    pub fn as_raw(&self) -> CFDictionaryRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Default for CFMutableDictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for CFMutableDictionary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CFMutableDictionary(len={})", self.len())
    }
}
