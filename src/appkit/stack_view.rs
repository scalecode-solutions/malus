//! NSStackView — a view that arranges subviews in a stack.

use crate::runtime::*;

/// The orientation of the stack view.
#[repr(isize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NSUserInterfaceLayoutOrientation {
    Horizontal = 0,
    Vertical = 1,
}

/// Distribution modes for the stack view.
#[repr(isize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NSStackViewDistribution {
    GravityAreas = -1,
    Fill = 0,
    FillEqually = 1,
    FillProportionally = 2,
    EqualSpacing = 3,
    EqualCentering = 4,
}

/// Edge insets for the stack view.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct NSEdgeInsets {
    pub top: CGFloat,
    pub left: CGFloat,
    pub bottom: CGFloat,
    pub right: CGFloat,
}

/// A view that arranges an array of views horizontally or vertically with
/// Auto Layout constraints.
pub struct NSStackView(pub(super) Id);

impl NSStackView {
    pub fn as_raw(&self) -> Id { self.0 }

    /// Create a new stack view with the given frame.
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSStackView") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    /// Create a stack view pre-populated with the given views.
    ///
    /// Uses `+[NSStackView stackViewWithViews:]` class method.
    pub fn with_views(views: &[Id]) -> Self {
        unsafe {
            // Build an NSArray from the slice
            let ns_array_cls = cls!("NSArray") as Id;
            let arr: Id = msg_send!(
                ns_array_cls,
                sel!("arrayWithObjects:count:"),
                fn(Id, Sel, *const Id, NSUInteger) -> Id,
                views.as_ptr(), views.len() as NSUInteger
            );
            let cls = cls!("NSStackView") as Id;
            let obj: Id = msg_send!(
                cls,
                sel!("stackViewWithViews:"),
                fn(Id, Sel, Id) -> Id,
                arr
            );
            Self(retain(obj))
        }
    }

    /// Add a view to the end of the arranged subviews.
    pub fn add_arranged_subview(&self, view: Id) {
        unsafe {
            msg_send!(self.0, sel!("addArrangedSubview:"), fn(Id, Sel, Id) -> (), view);
        }
    }

    /// Remove a view from the arranged subviews.
    pub fn remove_arranged_subview(&self, view: Id) {
        unsafe {
            msg_send!(self.0, sel!("removeArrangedSubview:"), fn(Id, Sel, Id) -> (), view);
        }
    }

    pub fn set_orientation(&self, orientation: NSUserInterfaceLayoutOrientation) {
        unsafe {
            msg_send!(self.0, sel!("setOrientation:"), fn(Id, Sel, NSInteger) -> (), orientation as NSInteger);
        }
    }

    pub fn set_spacing(&self, spacing: CGFloat) {
        unsafe {
            msg_send!(self.0, sel!("setSpacing:"), fn(Id, Sel, CGFloat) -> (), spacing);
        }
    }

    pub fn spacing(&self) -> CGFloat {
        unsafe { msg_send!(self.0, sel!("spacing"), fn(Id, Sel) -> CGFloat) }
    }

    /// Set the alignment (as a raw `NSLayoutAttribute` / `NSInteger` value).
    pub fn set_alignment(&self, alignment: NSInteger) {
        unsafe {
            msg_send!(self.0, sel!("setAlignment:"), fn(Id, Sel, NSInteger) -> (), alignment);
        }
    }

    pub fn set_distribution(&self, dist: NSStackViewDistribution) {
        unsafe {
            msg_send!(self.0, sel!("setDistribution:"), fn(Id, Sel, NSInteger) -> (), dist as NSInteger);
        }
    }

    /// Set the edge insets around the stack view's content.
    pub fn set_edge_insets(&self, top: CGFloat, left: CGFloat, bottom: CGFloat, right: CGFloat) {
        unsafe {
            let insets = NSEdgeInsets { top, left, bottom, right };
            msg_send!(self.0, sel!("setEdgeInsets:"), fn(Id, Sel, NSEdgeInsets) -> (), insets);
        }
    }
}

impl Clone for NSStackView {
    fn clone(&self) -> Self { Self(unsafe { retain(self.0) }) }
}

impl Drop for NSStackView {
    fn drop(&mut self) { unsafe { release(self.0) } }
}
