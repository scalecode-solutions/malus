//! NSColorWell — a control for selecting colors.

use crate::runtime::*;

pub struct NSColorWell(pub(super) Id);

impl NSColorWell {
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSColorWell") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }

    /// Returns the current color as a raw NSColor Id.
    pub fn color(&self) -> Id {
        unsafe { msg_send!(self.0, sel!("color"), fn(Id, Sel) -> Id) }
    }

    /// Set the color. Takes an NSColor as a raw Id.
    pub fn set_color(&self, color: Id) {
        unsafe {
            msg_send!(self.0, sel!("setColor:"), fn(Id, Sel, Id) -> (), color)
        }
    }

    pub fn set_target(&self, target: Id) {
        unsafe {
            msg_send!(self.0, sel!("setTarget:"), fn(Id, Sel, Id) -> (), target)
        }
    }

    pub fn set_action(&self, action: Sel) {
        unsafe {
            msg_send!(self.0, sel!("setAction:"), fn(Id, Sel, Sel) -> (), action)
        }
    }

    pub fn activate(&self, exclusive: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("activate:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(exclusive)
            )
        }
    }

    pub fn deactivate(&self) {
        unsafe {
            msg_send!(self.0, sel!("deactivate"), fn(Id, Sel) -> ())
        }
    }

    pub fn is_active(&self) -> bool {
        unsafe { to_bool(msg_send!(self.0, sel!("isActive"), fn(Id, Sel) -> BOOL)) }
    }
}

impl Clone for NSColorWell {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSColorWell {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
