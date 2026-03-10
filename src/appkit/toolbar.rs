//! NSToolbar + NSToolbarItem — window toolbar management.

use crate::runtime::*;

// ============================================================================
// NSToolbarDisplayMode
// ============================================================================

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NSToolbarDisplayMode {
    Default = 0,
    IconAndLabel = 1,
    IconOnly = 2,
    LabelOnly = 3,
}

// ============================================================================
// NSToolbar
// ============================================================================

pub struct NSToolbar(pub(super) Id);

impl NSToolbar {
    pub fn new(identifier: &str) -> Self {
        unsafe {
            let obj = alloc(cls!("NSToolbar") as Id);
            let ident_ns = nsstring(identifier);
            let raw: Id = msg_send!(
                obj,
                sel!("initWithIdentifier:"),
                fn(Id, Sel, Id) -> Id,
                ident_ns
            );
            Self(raw)
        }
    }

    pub fn set_delegate(&self, delegate: Id) {
        unsafe {
            msg_send!(self.0, sel!("setDelegate:"), fn(Id, Sel, Id) -> (), delegate);
        }
    }

    pub fn set_display_mode(&self, mode: NSToolbarDisplayMode) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setDisplayMode:"),
                fn(Id, Sel, u64) -> (),
                mode as u64
            );
        }
    }

    pub fn set_allows_user_customization(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setAllowsUserCustomization:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    pub fn set_visible(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setVisible:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    pub fn is_visible(&self) -> bool {
        unsafe {
            to_bool(msg_send!(self.0, sel!("isVisible"), fn(Id, Sel) -> BOOL))
        }
    }

    pub fn set_shows_baseline_separator(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setShowsBaselineSeparator:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for NSToolbar {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSToolbar {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

// ============================================================================
// NSToolbarItem
// ============================================================================

pub struct NSToolbarItem(pub(super) Id);

impl NSToolbarItem {
    pub fn new(identifier: &str) -> Self {
        unsafe {
            let obj = alloc(cls!("NSToolbarItem") as Id);
            let ident_ns = nsstring(identifier);
            let raw: Id = msg_send!(
                obj,
                sel!("initWithItemIdentifier:"),
                fn(Id, Sel, Id) -> Id,
                ident_ns
            );
            Self(raw)
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let label_ns = nsstring(label);
            msg_send!(self.0, sel!("setLabel:"), fn(Id, Sel, Id) -> (), label_ns);
        }
    }

    pub fn label(&self) -> String {
        unsafe {
            let raw: Id = msg_send!(self.0, sel!("label"), fn(Id, Sel) -> Id);
            from_nsstring(raw)
        }
    }

    pub fn set_palette_label(&self, label: &str) {
        unsafe {
            let label_ns = nsstring(label);
            msg_send!(self.0, sel!("setPaletteLabel:"), fn(Id, Sel, Id) -> (), label_ns);
        }
    }

    pub fn set_tool_tip(&self, tip: &str) {
        unsafe {
            let tip_ns = nsstring(tip);
            msg_send!(self.0, sel!("setToolTip:"), fn(Id, Sel, Id) -> (), tip_ns);
        }
    }

    pub fn set_image(&self, image: Id) {
        unsafe {
            msg_send!(self.0, sel!("setImage:"), fn(Id, Sel, Id) -> (), image);
        }
    }

    pub fn set_view(&self, view: Id) {
        unsafe {
            msg_send!(self.0, sel!("setView:"), fn(Id, Sel, Id) -> (), view);
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

    pub fn set_enabled(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setEnabled:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    pub fn identifier(&self) -> String {
        unsafe {
            let raw: Id = msg_send!(self.0, sel!("itemIdentifier"), fn(Id, Sel) -> Id);
            from_nsstring(raw)
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for NSToolbarItem {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSToolbarItem {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
