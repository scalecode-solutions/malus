//! CFString — CoreFoundation immutable string.

use super::base::*;
use std::ffi::c_char;
use std::fmt;

pub type CFStringRef = *const __CFString;

pub type CFStringEncoding = u32;
#[allow(non_upper_case_globals)]
pub const kCFStringEncodingUTF8: CFStringEncoding = 0x0800_0100;

extern "C" {
    fn CFStringCreateWithBytes(
        alloc: CFAllocatorRef,
        bytes: *const u8,
        num_bytes: CFIndex,
        encoding: CFStringEncoding,
        is_external_representation: Boolean,
    ) -> CFStringRef;

    fn CFStringGetLength(string: CFStringRef) -> CFIndex;

    fn CFStringGetCStringPtr(
        string: CFStringRef,
        encoding: CFStringEncoding,
    ) -> *const c_char;

    fn CFStringGetCString(
        string: CFStringRef,
        buffer: *mut c_char,
        buffer_size: CFIndex,
        encoding: CFStringEncoding,
    ) -> Boolean;

    fn CFStringGetMaximumSizeForEncoding(
        length: CFIndex,
        encoding: CFStringEncoding,
    ) -> CFIndex;
}

// ============================================================================
// CFString newtype
// ============================================================================

pub struct CFString(CFRef<__CFString>);

impl CFString {
    /// Create a CFString from a Rust string slice.
    pub fn new(s: &str) -> Self {
        unsafe {
            let raw = CFStringCreateWithBytes(
                std::ptr::null(),
                s.as_ptr(),
                s.len() as CFIndex,
                kCFStringEncodingUTF8,
                0,
            );
            assert!(!raw.is_null(), "CFStringCreateWithBytes returned null");
            Self(CFRef::wrap(raw).unwrap())
        }
    }

    /// Wrap a CFStringRef you own (from a Create/Copy function).
    /// Returns `None` if null.
    #[inline]
    pub unsafe fn wrap(ptr: CFStringRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    /// Wrap a borrowed CFStringRef (from a Get function) by retaining it.
    /// Returns `None` if null.
    #[inline]
    pub unsafe fn retain(ptr: CFStringRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    /// Convert to a Rust String.
    pub fn to_string(&self) -> String {
        unsafe {
            // Fast path: direct pointer if internal encoding is UTF-8
            let cptr = CFStringGetCStringPtr(self.as_raw(), kCFStringEncodingUTF8);
            if !cptr.is_null() {
                return std::ffi::CStr::from_ptr(cptr)
                    .to_string_lossy()
                    .into_owned();
            }

            // Slow path: copy into buffer
            let len = CFStringGetLength(self.as_raw());
            let max_size = CFStringGetMaximumSizeForEncoding(len, kCFStringEncodingUTF8) + 1;
            let mut buf = vec![0u8; max_size as usize];
            let ok = CFStringGetCString(
                self.as_raw(),
                buf.as_mut_ptr() as *mut c_char,
                max_size,
                kCFStringEncodingUTF8,
            );
            if ok != 0 {
                let cstr = std::ffi::CStr::from_ptr(buf.as_ptr() as *const c_char);
                cstr.to_string_lossy().into_owned()
            } else {
                String::new()
            }
        }
    }

    /// Length in UTF-16 code units (matches Apple's semantics).
    pub fn len(&self) -> usize {
        unsafe { CFStringGetLength(self.as_raw()) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn as_raw(&self) -> CFStringRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CFString {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for CFString {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Debug for CFString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CFString({:?})", self.to_string())
    }
}

impl fmt::Display for CFString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}

impl From<&str> for CFString {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<&String> for CFString {
    fn from(s: &String) -> Self {
        Self::new(s)
    }
}
