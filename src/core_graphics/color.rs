//! CGColor — an immutable color value with a color space.

use super::color_space::{CGColorSpace, CGColorSpaceRef};
use super::geometry::CGFloat;
use crate::core_foundation::base::{CFRef, CFTypeRef};
use std::ffi::c_void;
use std::fmt;

#[repr(C)]
pub struct __CGColor(c_void);
pub type CGColorRef = *const __CGColor;

extern "C" {
    fn CGColorCreate(space: CGColorSpaceRef, components: *const CGFloat) -> CGColorRef;
    fn CGColorGetNumberOfComponents(color: CGColorRef) -> usize;
    fn CGColorGetComponents(color: CGColorRef) -> *const CGFloat;
    fn CGColorGetAlpha(color: CGColorRef) -> CGFloat;
    fn CGColorGetColorSpace(color: CGColorRef) -> CGColorSpaceRef;
}

// ============================================================================
// CGColor newtype
// ============================================================================

pub struct CGColor(CFRef<__CGColor>);

impl CGColor {
    /// Create a color in the given color space.
    /// `components` length must match the color space's component count + 1 (for alpha).
    pub fn new(space: &CGColorSpace, components: &[CGFloat]) -> Self {
        unsafe {
            let raw = CGColorCreate(space.as_raw(), components.as_ptr());
            Self(CFRef::wrap(raw).expect("CGColorCreate returned null"))
        }
    }

    /// Create an sRGB color.
    pub fn rgba(r: CGFloat, g: CGFloat, b: CGFloat, a: CGFloat) -> Self {
        let space = CGColorSpace::srgb();
        Self::new(&space, &[r, g, b, a])
    }

    /// Create an opaque sRGB color.
    pub fn rgb(r: CGFloat, g: CGFloat, b: CGFloat) -> Self {
        Self::rgba(r, g, b, 1.0)
    }

    /// Create a grayscale color.
    pub fn gray(white: CGFloat, alpha: CGFloat) -> Self {
        let space = CGColorSpace::device_gray();
        Self::new(&space, &[white, alpha])
    }

    pub fn white() -> Self { Self::gray(1.0, 1.0) }
    pub fn black() -> Self { Self::gray(0.0, 1.0) }
    pub fn clear() -> Self { Self::gray(0.0, 0.0) }

    pub fn alpha(&self) -> CGFloat {
        unsafe { CGColorGetAlpha(self.as_raw()) }
    }

    pub fn number_of_components(&self) -> usize {
        unsafe { CGColorGetNumberOfComponents(self.as_raw()) }
    }

    /// Get the raw component values (including alpha as the last element).
    pub fn components(&self) -> &[CGFloat] {
        unsafe {
            let ptr = CGColorGetComponents(self.as_raw());
            let count = self.number_of_components();
            std::slice::from_raw_parts(ptr, count)
        }
    }

    /// Get the color space (borrowed — the color retains it).
    pub fn color_space(&self) -> CGColorSpace {
        unsafe {
            let ptr = CGColorGetColorSpace(self.as_raw());
            // Get function — we don't own it, must retain.
            CGColorSpace::retain(ptr).expect("CGColorGetColorSpace returned null")
        }
    }

    #[inline]
    pub unsafe fn wrap(ptr: CGColorRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    #[inline]
    pub unsafe fn retain(ptr: CGColorRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CGColorRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CGColor {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CGColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = self.components();
        write!(f, "CGColor({c:?})")
    }
}
