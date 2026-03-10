//! NSImageView — a view that displays an image.

use crate::runtime::*;

// ============================================================================
// NSImageScaling
// ============================================================================

#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NSImageScaling {
    ProportionallyDown = 0,
    AxesIndependently = 1,
    None = 2,
    ProportionallyUpOrDown = 3,
}

// ============================================================================
// NSImageAlignment
// ============================================================================

#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NSImageAlignment {
    Center = 0,
    Top = 1,
    TopLeft = 2,
    TopRight = 3,
    Left = 4,
    Bottom = 5,
    BottomLeft = 6,
    BottomRight = 7,
    Right = 8,
}

// ============================================================================
// NSImageView
// ============================================================================

pub struct NSImageView(pub(super) Id);

impl NSImageView {
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSImageView") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }

    /// Set the image. Takes an NSImage as a raw Id.
    pub fn set_image(&self, image: Id) {
        unsafe {
            msg_send!(self.0, sel!("setImage:"), fn(Id, Sel, Id) -> (), image)
        }
    }

    /// Returns the image as a raw NSImage Id, or None if nil.
    pub fn image(&self) -> Option<Id> {
        unsafe {
            let raw: Id = msg_send!(self.0, sel!("image"), fn(Id, Sel) -> Id);
            if raw.is_null() { None } else { Some(raw) }
        }
    }

    pub fn set_image_scaling(&self, scaling: NSImageScaling) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setImageScaling:"),
                fn(Id, Sel, NSUInteger) -> (),
                scaling as NSUInteger
            )
        }
    }

    pub fn set_image_alignment(&self, alignment: NSImageAlignment) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setImageAlignment:"),
                fn(Id, Sel, NSUInteger) -> (),
                alignment as NSUInteger
            )
        }
    }

    pub fn set_editable(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setEditable:"), fn(Id, Sel, BOOL) -> (), from_bool(flag))
        }
    }

    pub fn set_animates(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setAnimates:"), fn(Id, Sel, BOOL) -> (), from_bool(flag))
        }
    }
}

impl Clone for NSImageView {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSImageView {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
