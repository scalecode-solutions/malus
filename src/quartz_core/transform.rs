//! CATransform3D — a 4x4 matrix used for layer transforms.
//!
//! This is a value type (C struct), not an ObjC object.

use crate::runtime::CGFloat;

/// A 4x4 transformation matrix used to rotate, scale, translate, skew, and
/// project layer content.
#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CATransform3D {
    pub m11: CGFloat, pub m12: CGFloat, pub m13: CGFloat, pub m14: CGFloat,
    pub m21: CGFloat, pub m22: CGFloat, pub m23: CGFloat, pub m24: CGFloat,
    pub m31: CGFloat, pub m32: CGFloat, pub m33: CGFloat, pub m34: CGFloat,
    pub m41: CGFloat, pub m42: CGFloat, pub m43: CGFloat, pub m44: CGFloat,
}

extern "C" {
    fn CATransform3DIsIdentity(t: CATransform3D) -> bool;
    #[allow(dead_code)]
    fn CATransform3DEqualToTransform(a: CATransform3D, b: CATransform3D) -> bool;
    fn CATransform3DInvert(t: CATransform3D) -> CATransform3D;
    fn CATransform3DConcat(a: CATransform3D, b: CATransform3D) -> CATransform3D;
    fn CATransform3DMakeTranslation(tx: CGFloat, ty: CGFloat, tz: CGFloat) -> CATransform3D;
    fn CATransform3DMakeScale(sx: CGFloat, sy: CGFloat, sz: CGFloat) -> CATransform3D;
    fn CATransform3DMakeRotation(angle: CGFloat, x: CGFloat, y: CGFloat, z: CGFloat) -> CATransform3D;
}

impl CATransform3D {
    /// The identity transform.
    pub const IDENTITY: Self = Self {
        m11: 1.0, m12: 0.0, m13: 0.0, m14: 0.0,
        m21: 0.0, m22: 1.0, m23: 0.0, m24: 0.0,
        m31: 0.0, m32: 0.0, m33: 1.0, m34: 0.0,
        m41: 0.0, m42: 0.0, m43: 0.0, m44: 1.0,
    };

    /// Return the identity transform.
    pub fn identity() -> Self {
        Self::IDENTITY
    }

    /// Create a translation transform.
    pub fn make_translation(tx: CGFloat, ty: CGFloat, tz: CGFloat) -> Self {
        unsafe { CATransform3DMakeTranslation(tx, ty, tz) }
    }

    /// Create a scaling transform.
    pub fn make_scale(sx: CGFloat, sy: CGFloat, sz: CGFloat) -> Self {
        unsafe { CATransform3DMakeScale(sx, sy, sz) }
    }

    /// Create a rotation transform. `angle` is in radians, `(x, y, z)` is the
    /// rotation axis.
    pub fn make_rotation(angle: CGFloat, x: CGFloat, y: CGFloat, z: CGFloat) -> Self {
        unsafe { CATransform3DMakeRotation(angle, x, y, z) }
    }

    /// Concatenate two transforms: `self * other`.
    pub fn concat(self, other: Self) -> Self {
        unsafe { CATransform3DConcat(self, other) }
    }

    /// Return the inverse of this transform.
    pub fn invert(self) -> Self {
        unsafe { CATransform3DInvert(self) }
    }

    /// Check whether this is the identity transform.
    pub fn is_identity(self) -> bool {
        unsafe { CATransform3DIsIdentity(self) }
    }
}

impl Default for CATransform3D {
    fn default() -> Self {
        Self::IDENTITY
    }
}
