//! CFData — CoreFoundation immutable byte buffer.

use super::base::*;
use std::fmt;

pub type CFDataRef = *const __CFData;

extern "C" {
    fn CFDataCreate(
        allocator: CFAllocatorRef,
        bytes: *const u8,
        length: CFIndex,
    ) -> CFDataRef;

    fn CFDataGetLength(data: CFDataRef) -> CFIndex;
    fn CFDataGetBytePtr(data: CFDataRef) -> *const u8;
}

// ============================================================================
// CFData newtype
// ============================================================================

pub struct CFData(CFRef<__CFData>);

impl CFData {
    /// Create a CFData from a byte slice (copies the data).
    pub fn new(bytes: &[u8]) -> Self {
        unsafe {
            let raw = CFDataCreate(
                std::ptr::null(),
                bytes.as_ptr(),
                bytes.len() as CFIndex,
            );
            Self(CFRef::wrap(raw).expect("CFDataCreate returned null"))
        }
    }

    /// View the contents as a byte slice.
    pub fn as_bytes(&self) -> &[u8] {
        let len = self.len();
        if len == 0 {
            return &[];
        }
        unsafe {
            let ptr = CFDataGetBytePtr(self.as_raw());
            std::slice::from_raw_parts(ptr, len)
        }
    }

    pub fn len(&self) -> usize {
        unsafe { CFDataGetLength(self.as_raw()) as usize }
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Wrap a CFDataRef you own (from a Create/Copy function).
    #[inline]
    pub unsafe fn wrap(ptr: CFDataRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    /// Wrap a borrowed CFDataRef (from a Get function) by retaining it.
    #[inline]
    pub unsafe fn retain(ptr: CFDataRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CFDataRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CFData {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CFData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CFData({} bytes)", self.len())
    }
}

impl From<&[u8]> for CFData {
    fn from(bytes: &[u8]) -> Self {
        Self::new(bytes)
    }
}

impl From<&Vec<u8>> for CFData {
    fn from(bytes: &Vec<u8>) -> Self {
        Self::new(bytes)
    }
}
