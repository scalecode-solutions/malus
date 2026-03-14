//! CGPath / CGMutablePath — reusable vector paths.

use super::geometry::*;
use crate::core_foundation::base::{CFRef, CFTypeRef};
use std::ffi::c_void;
use std::fmt;

#[repr(C)]
pub struct __CGPath(c_void);
pub type CGPathRef = *const __CGPath;
pub type CGMutablePathRef = *mut __CGPath;

extern "C" {
    fn CGPathCreateMutable() -> CGMutablePathRef;
    fn CGPathCreateCopy(path: CGPathRef) -> CGPathRef;
    #[allow(dead_code)]
    fn CGPathCreateMutableCopy(path: CGPathRef) -> CGMutablePathRef;

    fn CGPathIsEmpty(path: CGPathRef) -> bool;
    fn CGPathGetBoundingBox(path: CGPathRef) -> CGRect;

    fn CGPathMoveToPoint(path: CGMutablePathRef, m: *const CGAffineTransform, x: CGFloat, y: CGFloat);
    fn CGPathAddLineToPoint(path: CGMutablePathRef, m: *const CGAffineTransform, x: CGFloat, y: CGFloat);
    fn CGPathAddCurveToPoint(
        path: CGMutablePathRef, m: *const CGAffineTransform,
        cp1x: CGFloat, cp1y: CGFloat,
        cp2x: CGFloat, cp2y: CGFloat,
        x: CGFloat, y: CGFloat,
    );
    fn CGPathAddQuadCurveToPoint(
        path: CGMutablePathRef, m: *const CGAffineTransform,
        cpx: CGFloat, cpy: CGFloat,
        x: CGFloat, y: CGFloat,
    );
    fn CGPathAddRect(path: CGMutablePathRef, m: *const CGAffineTransform, rect: CGRect);
    fn CGPathAddEllipseInRect(path: CGMutablePathRef, m: *const CGAffineTransform, rect: CGRect);
    fn CGPathAddArc(
        path: CGMutablePathRef, m: *const CGAffineTransform,
        x: CGFloat, y: CGFloat,
        radius: CGFloat,
        start_angle: CGFloat, end_angle: CGFloat,
        clockwise: bool,
    );
    fn CGPathAddRoundedRect(
        path: CGMutablePathRef, m: *const CGAffineTransform,
        rect: CGRect, corner_width: CGFloat, corner_height: CGFloat,
    );
    fn CGPathCloseSubpath(path: CGMutablePathRef);
}

// ============================================================================
// CGPath — immutable
// ============================================================================

pub struct CGPath(CFRef<__CGPath>);

impl CGPath {
    pub fn is_empty(&self) -> bool {
        unsafe { CGPathIsEmpty(self.as_raw()) }
    }

    pub fn bounding_box(&self) -> CGRect {
        unsafe { CGPathGetBoundingBox(self.as_raw()) }
    }

    #[inline]
    pub unsafe fn wrap(ptr: CGPathRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    #[inline]
    pub unsafe fn retain(ptr: CGPathRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CGPathRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CGPath {
    fn clone(&self) -> Self {
        unsafe {
            let copy = CGPathCreateCopy(self.as_raw());
            Self(CFRef::wrap(copy).unwrap())
        }
    }
}

impl fmt::Debug for CGPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CGPath(bbox={:?})", self.bounding_box())
    }
}

// ============================================================================
// CGMutablePath
// ============================================================================

pub struct CGMutablePath(CFRef<__CGPath>);

impl CGMutablePath {
    pub fn new() -> Self {
        unsafe {
            let raw = CGPathCreateMutable();
            Self(CFRef::wrap(raw as CGPathRef).expect("CGPathCreateMutable returned null"))
        }
    }

    pub fn move_to(&self, x: CGFloat, y: CGFloat) {
        unsafe { CGPathMoveToPoint(self.as_raw_mut(), std::ptr::null(), x, y) }
    }

    pub fn line_to(&self, x: CGFloat, y: CGFloat) {
        unsafe { CGPathAddLineToPoint(self.as_raw_mut(), std::ptr::null(), x, y) }
    }

    pub fn curve_to(
        &self,
        cp1x: CGFloat, cp1y: CGFloat,
        cp2x: CGFloat, cp2y: CGFloat,
        x: CGFloat, y: CGFloat,
    ) {
        unsafe { CGPathAddCurveToPoint(self.as_raw_mut(), std::ptr::null(), cp1x, cp1y, cp2x, cp2y, x, y) }
    }

    pub fn quad_curve_to(&self, cpx: CGFloat, cpy: CGFloat, x: CGFloat, y: CGFloat) {
        unsafe { CGPathAddQuadCurveToPoint(self.as_raw_mut(), std::ptr::null(), cpx, cpy, x, y) }
    }

    pub fn add_rect(&self, rect: CGRect) {
        unsafe { CGPathAddRect(self.as_raw_mut(), std::ptr::null(), rect) }
    }

    pub fn add_ellipse(&self, rect: CGRect) {
        unsafe { CGPathAddEllipseInRect(self.as_raw_mut(), std::ptr::null(), rect) }
    }

    pub fn add_arc(
        &self,
        x: CGFloat, y: CGFloat,
        radius: CGFloat,
        start_angle: CGFloat, end_angle: CGFloat,
        clockwise: bool,
    ) {
        unsafe { CGPathAddArc(self.as_raw_mut(), std::ptr::null(), x, y, radius, start_angle, end_angle, clockwise) }
    }

    pub fn add_rounded_rect(&self, rect: CGRect, corner_width: CGFloat, corner_height: CGFloat) {
        unsafe { CGPathAddRoundedRect(self.as_raw_mut(), std::ptr::null(), rect, corner_width, corner_height) }
    }

    pub fn close(&self) {
        unsafe { CGPathCloseSubpath(self.as_raw_mut()) }
    }

    pub fn is_empty(&self) -> bool {
        unsafe { CGPathIsEmpty(self.as_raw()) }
    }

    pub fn bounding_box(&self) -> CGRect {
        unsafe { CGPathGetBoundingBox(self.as_raw()) }
    }

    /// Freeze into an immutable CGPath.
    pub fn to_path(&self) -> CGPath {
        unsafe {
            let copy = CGPathCreateCopy(self.as_raw());
            CGPath(CFRef::wrap(copy).unwrap())
        }
    }

    #[inline]
    fn as_raw_mut(&self) -> CGMutablePathRef {
        self.0.as_ptr() as CGMutablePathRef
    }

    #[inline]
    pub fn as_raw(&self) -> CGPathRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Default for CGMutablePath {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Debug for CGMutablePath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CGMutablePath(bbox={:?})", self.bounding_box())
    }
}
