//! CTFontDescriptor — describes font attributes for matching.

use crate::core_foundation::base::{CFRef, CFTypeRef};
use crate::core_foundation::dictionary::CFDictionaryRef;
use crate::core_foundation::string::CFStringRef;
use crate::core_graphics::geometry::CGFloat;
use std::ffi::c_void;
use std::fmt;

#[repr(C)]
pub struct __CTFontDescriptor(c_void);
pub type CTFontDescriptorRef = *const __CTFontDescriptor;

extern "C" {
    fn CTFontDescriptorCreateWithNameAndSize(
        name: CFStringRef,
        size: CGFloat,
    ) -> CTFontDescriptorRef;

    fn CTFontDescriptorCreateWithAttributes(
        attributes: CFDictionaryRef,
    ) -> CTFontDescriptorRef;

    fn CTFontDescriptorCopyAttribute(
        descriptor: CTFontDescriptorRef,
        attribute: CFStringRef,
    ) -> CFTypeRef;

    fn CTFontDescriptorCopyAttributes(
        descriptor: CTFontDescriptorRef,
    ) -> CFDictionaryRef;
}

// ============================================================================
// CTFontDescriptor newtype
// ============================================================================

pub struct CTFontDescriptor(CFRef<__CTFontDescriptor>);

impl CTFontDescriptor {
    /// Create a descriptor from a font name and size.
    pub fn new_with_name_and_size(name: &crate::core_foundation::string::CFString, size: CGFloat) -> Self {
        unsafe {
            let raw = CTFontDescriptorCreateWithNameAndSize(name.as_raw(), size);
            Self(CFRef::wrap(raw).expect("CTFontDescriptorCreateWithNameAndSize returned null"))
        }
    }

    /// Create a descriptor from an attributes dictionary.
    pub fn new_with_attributes(attributes: &crate::core_foundation::dictionary::CFDictionary) -> Self {
        unsafe {
            let raw = CTFontDescriptorCreateWithAttributes(attributes.as_raw());
            Self(CFRef::wrap(raw).expect("CTFontDescriptorCreateWithAttributes returned null"))
        }
    }

    /// Copy a single attribute value. Returns null if the attribute is not present.
    pub fn copy_attribute(&self, attribute: CFStringRef) -> CFTypeRef {
        unsafe { CTFontDescriptorCopyAttribute(self.as_raw(), attribute) }
    }

    /// Copy the full attributes dictionary.
    pub fn copy_attributes(&self) -> Option<crate::core_foundation::dictionary::CFDictionary> {
        unsafe {
            let raw = CTFontDescriptorCopyAttributes(self.as_raw());
            crate::core_foundation::dictionary::CFDictionary::wrap(raw)
        }
    }

    #[inline]
    pub unsafe fn wrap(ptr: CTFontDescriptorRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    #[inline]
    pub unsafe fn retain(ptr: CTFontDescriptorRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CTFontDescriptorRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CTFontDescriptor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CTFontDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CTFontDescriptor({:?})", self.0)
    }
}
