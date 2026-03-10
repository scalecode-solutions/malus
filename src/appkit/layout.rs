//! NSLayoutConstraint and Auto Layout helpers.

use crate::runtime::*;

/// An Auto Layout constraint.
pub struct NSLayoutConstraint(pub(super) Id);

impl NSLayoutConstraint {
    pub fn as_raw(&self) -> Id { self.0 }

    /// Wrap an existing retained constraint pointer.
    ///
    /// # Safety
    /// `ptr` must be a valid, retained `NSLayoutConstraint` instance.
    pub unsafe fn from_raw(ptr: Id) -> Self {
        Self(ptr)
    }

    /// Activate an array of constraints.
    pub fn activate(constraints: &[NSLayoutConstraint]) {
        unsafe {
            let ptrs: Vec<Id> = constraints.iter().map(|c| c.0).collect();
            let ns_array_cls = cls!("NSArray") as Id;
            let arr: Id = msg_send!(
                ns_array_cls,
                sel!("arrayWithObjects:count:"),
                fn(Id, Sel, *const Id, NSUInteger) -> Id,
                ptrs.as_ptr(), ptrs.len() as NSUInteger
            );
            let cls = cls!("NSLayoutConstraint") as Id;
            msg_send!(cls, sel!("activateConstraints:"), fn(Id, Sel, Id) -> (), arr);
        }
    }

    /// Deactivate an array of constraints.
    pub fn deactivate(constraints: &[NSLayoutConstraint]) {
        unsafe {
            let ptrs: Vec<Id> = constraints.iter().map(|c| c.0).collect();
            let ns_array_cls = cls!("NSArray") as Id;
            let arr: Id = msg_send!(
                ns_array_cls,
                sel!("arrayWithObjects:count:"),
                fn(Id, Sel, *const Id, NSUInteger) -> Id,
                ptrs.as_ptr(), ptrs.len() as NSUInteger
            );
            let cls = cls!("NSLayoutConstraint") as Id;
            msg_send!(cls, sel!("deactivateConstraints:"), fn(Id, Sel, Id) -> (), arr);
        }
    }

    pub fn set_constant(&self, val: CGFloat) {
        unsafe {
            msg_send!(self.0, sel!("setConstant:"), fn(Id, Sel, CGFloat) -> (), val);
        }
    }

    pub fn constant(&self) -> CGFloat {
        unsafe { msg_send!(self.0, sel!("constant"), fn(Id, Sel) -> CGFloat) }
    }

    pub fn set_priority(&self, val: f32) {
        unsafe {
            msg_send!(self.0, sel!("setPriority:"), fn(Id, Sel, f32) -> (), val);
        }
    }

    pub fn priority(&self) -> f32 {
        unsafe { msg_send!(self.0, sel!("priority"), fn(Id, Sel) -> f32) }
    }

    pub fn set_active(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setActive:"), fn(Id, Sel, BOOL) -> (), from_bool(flag));
        }
    }

    pub fn is_active(&self) -> bool {
        unsafe { to_bool(msg_send!(self.0, sel!("isActive"), fn(Id, Sel) -> BOOL)) }
    }

    pub fn set_identifier(&self, id: &str) {
        unsafe {
            let ns = nsstring(id);
            msg_send!(self.0, sel!("setIdentifier:"), fn(Id, Sel, Id) -> (), ns);
        }
    }
}

impl Clone for NSLayoutConstraint {
    fn clone(&self) -> Self { Self(unsafe { retain(self.0) }) }
}

impl Drop for NSLayoutConstraint {
    fn drop(&mut self) { unsafe { release(self.0) } }
}

// ============================================================================
// Helper functions
// ============================================================================

/// Pin all four edges of `view` to `to_superview` with zero insets.
///
/// Both views should have `translatesAutoresizingMaskIntoConstraints` set to
/// `false` (at least `view`).
pub fn pin_edges(view: Id, to_superview: Id) -> Vec<NSLayoutConstraint> {
    unsafe {
        let top = constrain_equal(
            msg_send!(view, sel!("topAnchor"), fn(Id, Sel) -> Id),
            msg_send!(to_superview, sel!("topAnchor"), fn(Id, Sel) -> Id),
        );
        let bottom = constrain_equal(
            msg_send!(view, sel!("bottomAnchor"), fn(Id, Sel) -> Id),
            msg_send!(to_superview, sel!("bottomAnchor"), fn(Id, Sel) -> Id),
        );
        let leading = constrain_equal(
            msg_send!(view, sel!("leadingAnchor"), fn(Id, Sel) -> Id),
            msg_send!(to_superview, sel!("leadingAnchor"), fn(Id, Sel) -> Id),
        );
        let trailing = constrain_equal(
            msg_send!(view, sel!("trailingAnchor"), fn(Id, Sel) -> Id),
            msg_send!(to_superview, sel!("trailingAnchor"), fn(Id, Sel) -> Id),
        );
        let constraints = vec![top, bottom, leading, trailing];
        NSLayoutConstraint::activate(&constraints);
        constraints
    }
}

/// Create a constraint: `anchor1 == anchor2`.
pub fn constrain_equal(anchor1: Id, anchor2: Id) -> NSLayoutConstraint {
    unsafe {
        let raw: Id = msg_send!(
            anchor1,
            sel!("constraintEqualToAnchor:"),
            fn(Id, Sel, Id) -> Id,
            anchor2
        );
        NSLayoutConstraint(retain(raw))
    }
}

/// Create a constraint: `anchor1 == anchor2 + constant`.
pub fn constrain_equal_constant(anchor1: Id, anchor2: Id, constant: CGFloat) -> NSLayoutConstraint {
    unsafe {
        let raw: Id = msg_send!(
            anchor1,
            sel!("constraintEqualToAnchor:constant:"),
            fn(Id, Sel, Id, CGFloat) -> Id,
            anchor2, constant
        );
        NSLayoutConstraint(retain(raw))
    }
}

/// Create a constraint: `view.widthAnchor == width`.
pub fn constrain_width(view: Id, width: CGFloat) -> NSLayoutConstraint {
    unsafe {
        let anchor: Id = msg_send!(view, sel!("widthAnchor"), fn(Id, Sel) -> Id);
        let raw: Id = msg_send!(
            anchor,
            sel!("constraintEqualToConstant:"),
            fn(Id, Sel, CGFloat) -> Id,
            width
        );
        NSLayoutConstraint(retain(raw))
    }
}

/// Create a constraint: `view.heightAnchor == height`.
pub fn constrain_height(view: Id, height: CGFloat) -> NSLayoutConstraint {
    unsafe {
        let anchor: Id = msg_send!(view, sel!("heightAnchor"), fn(Id, Sel) -> Id);
        let raw: Id = msg_send!(
            anchor,
            sel!("constraintEqualToConstant:"),
            fn(Id, Sel, CGFloat) -> Id,
            height
        );
        NSLayoutConstraint(retain(raw))
    }
}
