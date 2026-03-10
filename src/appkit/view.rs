//! NSView — the base class for all visual elements in AppKit.

use crate::runtime::*;

/// A view — the fundamental building block of the visual hierarchy.
pub struct NSView(pub(super) Id);

impl NSView {
    /// Return the raw ObjC pointer (does not transfer ownership).
    pub fn as_raw(&self) -> Id {
        self.0
    }

    /// Create a new view with the given frame.
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSView") as Id);
            let obj: Id = msg_send!(
                obj,
                sel!("initWithFrame:"),
                fn(Id, Sel, CGRect) -> Id,
                frame
            );
            Self(obj)
        }
    }

    /// Get the view's frame rectangle.
    pub fn frame(&self) -> CGRect {
        unsafe {
            msg_send!(self.0, sel!("frame"), fn(Id, Sel) -> CGRect)
        }
    }

    /// Get the view's bounds rectangle.
    pub fn bounds(&self) -> CGRect {
        unsafe {
            msg_send!(self.0, sel!("bounds"), fn(Id, Sel) -> CGRect)
        }
    }

    /// Set the view's frame rectangle.
    pub fn set_frame(&self, rect: CGRect) {
        unsafe {
            msg_send!(self.0, sel!("setFrame:"), fn(Id, Sel, CGRect) -> (), rect);
        }
    }

    /// Add a subview.
    pub fn add_subview(&self, child: &NSView) {
        unsafe {
            msg_send!(
                self.0,
                sel!("addSubview:"),
                fn(Id, Sel, Id) -> (),
                child.0
            );
        }
    }

    /// Remove this view from its superview.
    pub fn remove_from_superview(&self) {
        unsafe {
            msg_send!(self.0, sel!("removeFromSuperview"), fn(Id, Sel) -> ());
        }
    }

    /// Mark the view as needing display.
    pub fn set_needs_display(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setNeedsDisplay:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    /// Set whether the view uses a Core Animation layer.
    pub fn set_wants_layer(&self, wants: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setWantsLayer:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(wants)
            );
        }
    }

    /// Set whether autoresizing mask is translated into Auto Layout constraints.
    pub fn set_translates_autoresizing_mask(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setTranslatesAutoresizingMaskIntoConstraints:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    /// Set whether the view is hidden.
    pub fn set_hidden(&self, hidden: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setHidden:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(hidden)
            );
        }
    }

    /// Check whether the view is hidden.
    pub fn is_hidden(&self) -> bool {
        unsafe {
            to_bool(msg_send!(self.0, sel!("isHidden"), fn(Id, Sel) -> BOOL))
        }
    }

    /// Set the view's alpha value.
    pub fn set_alpha_value(&self, alpha: CGFloat) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setAlphaValue:"),
                fn(Id, Sel, CGFloat) -> (),
                alpha
            );
        }
    }

    /// Get the subviews array as a raw NSArray Id.
    pub fn subviews(&self) -> Id {
        unsafe {
            msg_send!(self.0, sel!("subviews"), fn(Id, Sel) -> Id)
        }
    }

    /// Get the superview, if any.
    pub fn superview(&self) -> Option<NSView> {
        unsafe {
            let raw: Id = msg_send!(self.0, sel!("superview"), fn(Id, Sel) -> Id);
            if raw.is_null() {
                None
            } else {
                Some(NSView(retain(raw)))
            }
        }
    }

    /// Set the view's identifier.
    pub fn set_identifier(&self, id: &str) {
        unsafe {
            let ns = nsstring(id);
            msg_send!(self.0, sel!("setIdentifier:"), fn(Id, Sel, Id) -> (), ns);
        }
    }

    // -- Auto Layout anchors -------------------------------------------------

    /// Width layout anchor (raw Id).
    pub fn width_anchor(&self) -> Id {
        unsafe { msg_send!(self.0, sel!("widthAnchor"), fn(Id, Sel) -> Id) }
    }

    /// Height layout anchor (raw Id).
    pub fn height_anchor(&self) -> Id {
        unsafe { msg_send!(self.0, sel!("heightAnchor"), fn(Id, Sel) -> Id) }
    }

    /// Leading layout anchor (raw Id).
    pub fn leading_anchor(&self) -> Id {
        unsafe { msg_send!(self.0, sel!("leadingAnchor"), fn(Id, Sel) -> Id) }
    }

    /// Trailing layout anchor (raw Id).
    pub fn trailing_anchor(&self) -> Id {
        unsafe { msg_send!(self.0, sel!("trailingAnchor"), fn(Id, Sel) -> Id) }
    }

    /// Top layout anchor (raw Id).
    pub fn top_anchor(&self) -> Id {
        unsafe { msg_send!(self.0, sel!("topAnchor"), fn(Id, Sel) -> Id) }
    }

    /// Bottom layout anchor (raw Id).
    pub fn bottom_anchor(&self) -> Id {
        unsafe { msg_send!(self.0, sel!("bottomAnchor"), fn(Id, Sel) -> Id) }
    }

    /// Center-X layout anchor (raw Id).
    pub fn center_x_anchor(&self) -> Id {
        unsafe { msg_send!(self.0, sel!("centerXAnchor"), fn(Id, Sel) -> Id) }
    }

    /// Center-Y layout anchor (raw Id).
    pub fn center_y_anchor(&self) -> Id {
        unsafe { msg_send!(self.0, sel!("centerYAnchor"), fn(Id, Sel) -> Id) }
    }

    // -- Layer ---------------------------------------------------------------

    /// Get the view's backing CALayer as a raw Id.
    pub fn layer(&self) -> Id {
        unsafe { msg_send!(self.0, sel!("layer"), fn(Id, Sel) -> Id) }
    }

    /// Mark the view as needing layout.
    pub fn set_needs_layout(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setNeedsLayout:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }
}

impl Clone for NSView {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSView {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
