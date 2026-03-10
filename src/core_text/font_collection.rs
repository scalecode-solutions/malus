//! CTFontCollection — groups of font descriptors for matching.

use crate::core_foundation::array::CFArrayRef;
use crate::core_foundation::base::{CFRef, CFTypeRef};
use crate::core_foundation::dictionary::CFDictionaryRef;
use std::ffi::c_void;
use std::fmt;

#[repr(C)]
pub struct __CTFontCollection(c_void);
pub type CTFontCollectionRef = *const __CTFontCollection;

extern "C" {
    fn CTFontCollectionCreateFromAvailableFonts(
        options: CFDictionaryRef,
    ) -> CTFontCollectionRef;

    fn CTFontCollectionCreateMatchingFontDescriptors(
        collection: CTFontCollectionRef,
    ) -> CFArrayRef;
}

// ============================================================================
// CTFontCollection newtype
// ============================================================================

pub struct CTFontCollection(CFRef<__CTFontCollection>);

impl CTFontCollection {
    /// Create a collection from all available fonts on the system.
    pub fn from_available_fonts() -> Self {
        unsafe {
            let raw = CTFontCollectionCreateFromAvailableFonts(std::ptr::null());
            Self(CFRef::wrap(raw).expect("CTFontCollectionCreateFromAvailableFonts returned null"))
        }
    }

    /// Get an array of CTFontDescriptors matching this collection.
    pub fn matching_descriptors(&self) -> Option<crate::core_foundation::array::CFArray> {
        unsafe {
            let raw = CTFontCollectionCreateMatchingFontDescriptors(self.as_raw());
            crate::core_foundation::array::CFArray::wrap(raw)
        }
    }

    #[inline]
    pub unsafe fn wrap(ptr: CTFontCollectionRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    #[inline]
    pub unsafe fn retain(ptr: CTFontCollectionRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CTFontCollectionRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CTFontCollection {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CTFontCollection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CTFontCollection({:?})", self.0)
    }
}
