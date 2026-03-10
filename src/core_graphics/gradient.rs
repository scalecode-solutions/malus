//! CGGradient — smooth color transitions for fills.

use super::color_space::{CGColorSpace, CGColorSpaceRef};
use super::context::CGContextRef;
use super::geometry::{CGFloat, CGPoint};
use crate::core_foundation::base::{CFRef, CFTypeRef};
use std::ffi::c_void;
use std::fmt;

#[repr(C)]
pub struct __CGGradient(c_void);
pub type CGGradientRef = *const __CGGradient;

#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CGGradientDrawingOptions {
    None = 0,
    DrawsBeforeStartLocation = 1,
    DrawsAfterEndLocation = 2,
    Both = 3,
}

extern "C" {
    fn CGGradientCreateWithColorComponents(
        space: CGColorSpaceRef,
        components: *const CGFloat,
        locations: *const CGFloat,
        count: usize,
    ) -> CGGradientRef;

    fn CGContextDrawLinearGradient(
        c: CGContextRef,
        gradient: CGGradientRef,
        start_point: CGPoint,
        end_point: CGPoint,
        options: CGGradientDrawingOptions,
    );

    fn CGContextDrawRadialGradient(
        c: CGContextRef,
        gradient: CGGradientRef,
        start_center: CGPoint,
        start_radius: CGFloat,
        end_center: CGPoint,
        end_radius: CGFloat,
        options: CGGradientDrawingOptions,
    );
}

// ============================================================================
// CGGradient newtype
// ============================================================================

pub struct CGGradient(CFRef<__CGGradient>);

impl CGGradient {
    /// Create a gradient from color components and stop locations.
    ///
    /// `components` is a flat array: [r0, g0, b0, a0, r1, g1, b1, a1, ...].
    /// `locations` must have `count` elements, each in 0.0..=1.0.
    pub fn new(
        color_space: &CGColorSpace,
        components: &[CGFloat],
        locations: &[CGFloat],
    ) -> Self {
        unsafe {
            let raw = CGGradientCreateWithColorComponents(
                color_space.as_raw(),
                components.as_ptr(),
                locations.as_ptr(),
                locations.len(),
            );
            Self(CFRef::wrap(raw).expect("CGGradientCreateWithColorComponents returned null"))
        }
    }

    /// Create a simple two-stop linear gradient in sRGB.
    pub fn two_color(start: [CGFloat; 4], end: [CGFloat; 4]) -> Self {
        let space = CGColorSpace::srgb();
        let components = [
            start[0], start[1], start[2], start[3],
            end[0], end[1], end[2], end[3],
        ];
        Self::new(&space, &components, &[0.0, 1.0])
    }

    /// Draw this gradient as a linear fill into the given context.
    pub fn draw_linear(
        &self,
        ctx: &super::context::CGContext,
        start: CGPoint,
        end: CGPoint,
        options: CGGradientDrawingOptions,
    ) {
        unsafe {
            CGContextDrawLinearGradient(ctx.as_raw(), self.as_raw(), start, end, options);
        }
    }

    /// Draw this gradient as a radial fill into the given context.
    pub fn draw_radial(
        &self,
        ctx: &super::context::CGContext,
        start_center: CGPoint,
        start_radius: CGFloat,
        end_center: CGPoint,
        end_radius: CGFloat,
        options: CGGradientDrawingOptions,
    ) {
        unsafe {
            CGContextDrawRadialGradient(
                ctx.as_raw(),
                self.as_raw(),
                start_center, start_radius,
                end_center, end_radius,
                options,
            );
        }
    }

    #[inline]
    pub unsafe fn wrap(ptr: CGGradientRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CGGradientRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CGGradient {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CGGradient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CGGradient({:?})", self.0)
    }
}
