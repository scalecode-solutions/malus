//! NSColorPanel — system color picker.

use crate::runtime::*;

// ============================================================================
// NSColorPanel
// ============================================================================

pub struct NSColorPanel(pub(super) Id);

impl NSColorPanel {
    pub fn shared() -> Self {
        unsafe {
            let raw: Id = msg_send!(
                cls!("NSColorPanel") as Id,
                sel!("sharedColorPanel"),
                fn(Id, Sel) -> Id
            );
            Self(retain(raw))
        }
    }

    pub fn color(&self) -> Id {
        unsafe {
            msg_send!(self.0, sel!("color"), fn(Id, Sel) -> Id)
        }
    }

    pub fn set_color(&self, color: Id) {
        unsafe {
            msg_send!(self.0, sel!("setColor:"), fn(Id, Sel, Id) -> (), color);
        }
    }

    pub fn set_shows_alpha(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setShowsAlpha:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    pub fn set_continuous(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setContinuous:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    pub fn order_front(&self) {
        unsafe {
            msg_send!(
                self.0,
                sel!("makeKeyAndOrderFront:"),
                fn(Id, Sel, Id) -> (),
                nil
            );
        }
    }

    pub fn set_target(&self, target: Id) {
        unsafe {
            msg_send!(self.0, sel!("setTarget:"), fn(Id, Sel, Id) -> (), target);
        }
    }

    pub fn set_action(&self, action: Sel) {
        unsafe {
            msg_send!(self.0, sel!("setAction:"), fn(Id, Sel, Sel) -> (), action);
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for NSColorPanel {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSColorPanel {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
