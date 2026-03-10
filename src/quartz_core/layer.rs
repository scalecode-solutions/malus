//! CALayer — the fundamental unit of visual content in Core Animation.

use crate::runtime::*;

#[cfg(feature = "core-graphics")]
use crate::core_graphics::color::CGColorRef;

/// An object that manages image-based content and allows you to perform
/// animations on that content.
pub struct CALayer(Id);

impl CALayer {
    /// Create a new layer.
    pub fn new() -> Self {
        unsafe { Self(alloc_init(cls!("CALayer") as Id)) }
    }

    /// Wrap an existing retained layer pointer.
    ///
    /// # Safety
    /// `ptr` must be a valid, retained `CALayer` (or subclass) instance.
    pub unsafe fn from_raw(ptr: Id) -> Self {
        Self(ptr)
    }

    /// Return the raw ObjC pointer (does not transfer ownership).
    pub fn as_raw(&self) -> Id {
        self.0
    }

    // -- Geometry ------------------------------------------------------------

    pub fn set_frame(&self, frame: CGRect) {
        unsafe {
            msg_send!(self.0, sel!("setFrame:"), fn(Id, Sel, CGRect) -> (), frame)
        }
    }

    pub fn frame(&self) -> CGRect {
        unsafe {
            msg_send!(self.0, sel!("frame"), fn(Id, Sel) -> CGRect)
        }
    }

    // -- Appearance ----------------------------------------------------------

    /// Set the background color. Pass a `CGColorRef` obtained from a `CGColor`.
    #[cfg(feature = "core-graphics")]
    pub fn set_background_color(&self, color: CGColorRef) {
        unsafe {
            msg_send!(self.0, sel!("setBackgroundColor:"), fn(Id, Sel, CGColorRef) -> (), color)
        }
    }

    pub fn set_contents_scale(&self, scale: CGFloat) {
        unsafe {
            msg_send!(self.0, sel!("setContentsScale:"), fn(Id, Sel, CGFloat) -> (), scale)
        }
    }

    pub fn contents_scale(&self) -> CGFloat {
        unsafe {
            msg_send!(self.0, sel!("contentsScale"), fn(Id, Sel) -> CGFloat)
        }
    }

    pub fn set_opaque(&self, opaque: bool) {
        unsafe {
            msg_send!(self.0, sel!("setOpaque:"), fn(Id, Sel, BOOL) -> (), from_bool(opaque))
        }
    }

    pub fn is_opaque(&self) -> bool {
        unsafe {
            to_bool(msg_send!(self.0, sel!("isOpaque"), fn(Id, Sel) -> BOOL))
        }
    }

    pub fn set_corner_radius(&self, radius: CGFloat) {
        unsafe {
            msg_send!(self.0, sel!("setCornerRadius:"), fn(Id, Sel, CGFloat) -> (), radius)
        }
    }

    pub fn corner_radius(&self) -> CGFloat {
        unsafe {
            msg_send!(self.0, sel!("cornerRadius"), fn(Id, Sel) -> CGFloat)
        }
    }

    pub fn set_mask_to_bounds(&self, mask: bool) {
        unsafe {
            msg_send!(self.0, sel!("setMasksToBounds:"), fn(Id, Sel, BOOL) -> (), from_bool(mask))
        }
    }

    // -- Sublayer management -------------------------------------------------

    pub fn add_sublayer(&self, layer: &CALayer) {
        unsafe {
            msg_send!(self.0, sel!("addSublayer:"), fn(Id, Sel, Id) -> (), layer.0)
        }
    }

    pub fn remove_from_superlayer(&self) {
        unsafe {
            msg_send!(self.0, sel!("removeFromSuperlayer"), fn(Id, Sel) -> ())
        }
    }

    // -- Display -------------------------------------------------------------

    pub fn set_needs_display(&self) {
        unsafe {
            msg_send!(self.0, sel!("setNeedsDisplay"), fn(Id, Sel) -> ())
        }
    }
}

impl Clone for CALayer {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for CALayer {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
