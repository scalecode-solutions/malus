//! CGColorSpace — describes how to interpret color component values.

use crate::core_foundation::base::{CFRef, CFTypeRef};
use std::ffi::c_void;
use std::fmt;

#[repr(C)]
pub struct __CGColorSpace(c_void);
pub type CGColorSpaceRef = *const __CGColorSpace;

extern "C" {
    fn CGColorSpaceCreateDeviceRGB() -> CGColorSpaceRef;
    fn CGColorSpaceCreateDeviceGray() -> CGColorSpaceRef;
    fn CGColorSpaceCreateDeviceCMYK() -> CGColorSpaceRef;
    fn CGColorSpaceGetNumberOfComponents(space: CGColorSpaceRef) -> usize;
    fn CGColorSpaceCreateWithName(name: *const c_void) -> CGColorSpaceRef;

    // Well-known color space name constants
    static kCGColorSpaceSRGB: *const c_void;
    static kCGColorSpaceDisplayP3: *const c_void;
    static kCGColorSpaceGenericGrayGamma2_2: *const c_void;
    static kCGColorSpaceLinearSRGB: *const c_void;
    static kCGColorSpaceExtendedSRGB: *const c_void;
    static kCGColorSpaceExtendedLinearSRGB: *const c_void;
}

// ============================================================================
// CGColorSpace newtype
// ============================================================================

pub struct CGColorSpace(CFRef<__CGColorSpace>);

impl CGColorSpace {
    pub fn device_rgb() -> Self {
        unsafe {
            let raw = CGColorSpaceCreateDeviceRGB();
            Self(CFRef::wrap(raw).expect("CGColorSpaceCreateDeviceRGB returned null"))
        }
    }

    pub fn device_gray() -> Self {
        unsafe {
            let raw = CGColorSpaceCreateDeviceGray();
            Self(CFRef::wrap(raw).expect("CGColorSpaceCreateDeviceGray returned null"))
        }
    }

    pub fn device_cmyk() -> Self {
        unsafe {
            let raw = CGColorSpaceCreateDeviceCMYK();
            Self(CFRef::wrap(raw).expect("CGColorSpaceCreateDeviceCMYK returned null"))
        }
    }

    pub fn srgb() -> Self {
        unsafe { Self::from_name(kCGColorSpaceSRGB) }
    }

    pub fn display_p3() -> Self {
        unsafe { Self::from_name(kCGColorSpaceDisplayP3) }
    }

    pub fn linear_srgb() -> Self {
        unsafe { Self::from_name(kCGColorSpaceLinearSRGB) }
    }

    pub fn extended_srgb() -> Self {
        unsafe { Self::from_name(kCGColorSpaceExtendedSRGB) }
    }

    pub fn extended_linear_srgb() -> Self {
        unsafe { Self::from_name(kCGColorSpaceExtendedLinearSRGB) }
    }

    pub fn generic_gray_gamma_2_2() -> Self {
        unsafe { Self::from_name(kCGColorSpaceGenericGrayGamma2_2) }
    }

    unsafe fn from_name(name: *const c_void) -> Self {
        let raw = CGColorSpaceCreateWithName(name);
        Self(CFRef::wrap(raw).expect("CGColorSpaceCreateWithName returned null"))
    }

    pub fn number_of_components(&self) -> usize {
        unsafe { CGColorSpaceGetNumberOfComponents(self.as_raw()) }
    }

    #[inline]
    pub unsafe fn wrap(ptr: CGColorSpaceRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    #[inline]
    pub unsafe fn retain(ptr: CGColorSpaceRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CGColorSpaceRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CGColorSpace {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CGColorSpace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CGColorSpace(components={})", self.number_of_components())
    }
}
