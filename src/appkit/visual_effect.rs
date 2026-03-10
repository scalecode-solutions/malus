//! NSVisualEffectView — a view that applies visual effects like blur.

use crate::runtime::*;

/// Material types for visual effect views.
#[repr(isize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NSVisualEffectMaterial {
    Titlebar = 3,
    Selection = 4,
    Menu = 5,
    Popover = 6,
    Sidebar = 7,
    HeaderView = 10,
    Sheet = 11,
    WindowBackground = 12,
    HUDWindow = 13,
    FullScreenUI = 15,
    ToolTip = 17,
    ContentBackground = 18,
    UnderWindowBackground = 21,
    UnderPageBackground = 22,
}

/// Blending modes for visual effect views.
#[repr(isize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NSVisualEffectBlendingMode {
    BehindWindow = 0,
    WithinWindow = 1,
}

/// State values for visual effect views.
#[repr(isize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NSVisualEffectState {
    FollowsWindowActiveState = 0,
    Active = 1,
    Inactive = 2,
}

/// A view that applies a translucency and vibrancy effect to content
/// behind or within it.
pub struct NSVisualEffectView(pub(super) Id);

impl NSVisualEffectView {
    pub fn as_raw(&self) -> Id { self.0 }

    /// Create a new visual effect view with the given frame.
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSVisualEffectView") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    pub fn set_material(&self, material: NSVisualEffectMaterial) {
        unsafe {
            msg_send!(self.0, sel!("setMaterial:"), fn(Id, Sel, NSInteger) -> (), material as NSInteger);
        }
    }

    pub fn set_blending_mode(&self, mode: NSVisualEffectBlendingMode) {
        unsafe {
            msg_send!(self.0, sel!("setBlendingMode:"), fn(Id, Sel, NSInteger) -> (), mode as NSInteger);
        }
    }

    pub fn set_state(&self, state: NSVisualEffectState) {
        unsafe {
            msg_send!(self.0, sel!("setState:"), fn(Id, Sel, NSInteger) -> (), state as NSInteger);
        }
    }

    pub fn set_emphasized(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setEmphasized:"), fn(Id, Sel, BOOL) -> (), from_bool(flag));
        }
    }

    /// Set the mask image (raw NSImage Id).
    pub fn set_mask_image(&self, image: Id) {
        unsafe {
            msg_send!(self.0, sel!("setMaskImage:"), fn(Id, Sel, Id) -> (), image);
        }
    }
}

impl Clone for NSVisualEffectView {
    fn clone(&self) -> Self { Self(unsafe { retain(self.0) }) }
}

impl Drop for NSVisualEffectView {
    fn drop(&mut self) { unsafe { release(self.0) } }
}
