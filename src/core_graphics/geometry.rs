//! Geometry types and affine transforms.
//!
//! CGFloat, CGPoint, CGSize, CGRect live in `runtime.rs` (they're needed even
//! without the core-graphics feature). This module re-exports them and adds
//! CGAffineTransform.

pub use crate::runtime::{CGFloat, CGPoint, CGSize, CGRect};

// ============================================================================
// CGAffineTransform
// ============================================================================

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct CGAffineTransform {
    pub a: CGFloat,
    pub b: CGFloat,
    pub c: CGFloat,
    pub d: CGFloat,
    pub tx: CGFloat,
    pub ty: CGFloat,
}

extern "C" {
    fn CGAffineTransformIsIdentity(t: CGAffineTransform) -> bool;
    fn CGAffineTransformEqualToTransform(t1: CGAffineTransform, t2: CGAffineTransform) -> bool;
    fn CGPointApplyAffineTransform(point: CGPoint, t: CGAffineTransform) -> CGPoint;
    fn CGSizeApplyAffineTransform(size: CGSize, t: CGAffineTransform) -> CGSize;
    fn CGRectApplyAffineTransform(rect: CGRect, t: CGAffineTransform) -> CGRect;
}

impl CGAffineTransform {
    pub const IDENTITY: Self = Self {
        a: 1.0, b: 0.0,
        c: 0.0, d: 1.0,
        tx: 0.0, ty: 0.0,
    };

    pub fn new(a: CGFloat, b: CGFloat, c: CGFloat, d: CGFloat, tx: CGFloat, ty: CGFloat) -> Self {
        Self { a, b, c, d, tx, ty }
    }

    pub fn translation(tx: CGFloat, ty: CGFloat) -> Self {
        Self { a: 1.0, b: 0.0, c: 0.0, d: 1.0, tx, ty }
    }

    pub fn scale(sx: CGFloat, sy: CGFloat) -> Self {
        Self { a: sx, b: 0.0, c: 0.0, d: sy, tx: 0.0, ty: 0.0 }
    }

    pub fn rotation(angle: CGFloat) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self { a: cos, b: sin, c: -sin, d: cos, tx: 0.0, ty: 0.0 }
    }

    pub fn concat(self, other: Self) -> Self {
        Self {
            a: self.a * other.a + self.b * other.c,
            b: self.a * other.b + self.b * other.d,
            c: self.c * other.a + self.d * other.c,
            d: self.c * other.b + self.d * other.d,
            tx: self.tx * other.a + self.ty * other.c + other.tx,
            ty: self.tx * other.b + self.ty * other.d + other.ty,
        }
    }

    pub fn invert(self) -> Self {
        let det = self.a * self.d - self.b * self.c;
        if det.abs() < 1e-15 {
            return Self::IDENTITY;
        }
        let inv = 1.0 / det;
        Self {
            a: self.d * inv,
            b: -self.b * inv,
            c: -self.c * inv,
            d: self.a * inv,
            tx: (self.c * self.ty - self.d * self.tx) * inv,
            ty: (self.b * self.tx - self.a * self.ty) * inv,
        }
    }

    pub fn is_identity(self) -> bool {
        unsafe { CGAffineTransformIsIdentity(self) }
    }

    pub fn apply_to_point(self, point: CGPoint) -> CGPoint {
        unsafe { CGPointApplyAffineTransform(point, self) }
    }

    pub fn apply_to_size(self, size: CGSize) -> CGSize {
        unsafe { CGSizeApplyAffineTransform(size, self) }
    }

    pub fn apply_to_rect(self, rect: CGRect) -> CGRect {
        unsafe { CGRectApplyAffineTransform(rect, self) }
    }
}

impl PartialEq for CGAffineTransform {
    fn eq(&self, other: &Self) -> bool {
        unsafe { CGAffineTransformEqualToTransform(*self, *other) }
    }
}
