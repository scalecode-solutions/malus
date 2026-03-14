//! NSButton — a push button, checkbox, radio button, etc.

use crate::runtime::*;

// ============================================================================
// NSControlStateValue
// ============================================================================

#[repr(isize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NSControlStateValue {
    Mixed = -1,
    Off = 0,
    On = 1,
}

impl NSControlStateValue {
    pub fn from_raw(v: NSInteger) -> Self {
        match v {
            -1 => Self::Mixed,
            1 => Self::On,
            _ => Self::Off,
        }
    }
}

// ============================================================================
// NSButtonType
// ============================================================================

#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NSButtonType {
    MomentaryLight = 0,
    PushOnPushOff = 1,
    Toggle = 2,
    Switch = 3,
    Radio = 4,
    MomentaryChange = 5,
    OnOff = 6,
    MomentaryPushIn = 7,
}

// ============================================================================
// NSBezelStyle
// ============================================================================

#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NSBezelStyle {
    Rounded = 1,
    RegularSquare = 2,
    Disclosure = 5,
    ShadowlessSquare = 6,
    Circular = 7,
    TexturedSquare = 8,
    HelpButton = 9,
    SmallSquare = 10,
    TexturedRounded = 11,
    RoundRect = 12,
    Recessed = 13,
    RoundedDisclosure = 14,
    Inline = 15,
}

// ============================================================================
// NSCellImagePosition
// ============================================================================

#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NSCellImagePosition {
    NoImage = 0,
    ImageOnly = 1,
    ImageLeft = 2,
    ImageRight = 3,
    ImageBelow = 4,
    ImageAbove = 5,
    ImageOverlaps = 6,
    ImageLeading = 7,
    ImageTrailing = 8,
}

// ============================================================================
// NSButton
// ============================================================================

pub struct NSButton(pub(super) Id);

impl NSButton {
    /// Create a standard push button with the given title.
    pub fn new(title: &str) -> Self {
        unsafe {
            let ns = nsstring(title);
            let raw: Id = msg_send!(
                cls!("NSButton") as Id,
                sel!("buttonWithTitle:target:action:"),
                fn(Id, Sel, Id, Id, Sel) -> Id,
                ns,
                nil,
                std::mem::zeroed::<Sel>()
            );
            Self(retain(raw))
        }
    }

    /// Create a checkbox button.
    pub fn checkbox(title: &str) -> Self {
        unsafe {
            let ns = nsstring(title);
            let raw: Id = msg_send!(
                cls!("NSButton") as Id,
                sel!("checkboxWithTitle:target:action:"),
                fn(Id, Sel, Id, Id, Sel) -> Id,
                ns,
                nil,
                std::mem::zeroed::<Sel>()
            );
            Self(retain(raw))
        }
    }

    /// Create a radio button.
    pub fn radio(title: &str) -> Self {
        unsafe {
            let ns = nsstring(title);
            let raw: Id = msg_send!(
                cls!("NSButton") as Id,
                sel!("radioButtonWithTitle:target:action:"),
                fn(Id, Sel, Id, Id, Sel) -> Id,
                ns,
                nil,
                std::mem::zeroed::<Sel>()
            );
            Self(retain(raw))
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            let ns = nsstring(title);
            msg_send!(self.0, sel!("setTitle:"), fn(Id, Sel, Id) -> (), ns)
        }
    }

    pub fn title(&self) -> String {
        unsafe {
            let ns: Id = msg_send!(self.0, sel!("title"), fn(Id, Sel) -> Id);
            from_nsstring(ns)
        }
    }

    pub fn set_state(&self, state: NSControlStateValue) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setState:"),
                fn(Id, Sel, NSInteger) -> (),
                state as NSInteger
            )
        }
    }

    pub fn state(&self) -> NSControlStateValue {
        unsafe {
            let v: NSInteger = msg_send!(self.0, sel!("state"), fn(Id, Sel) -> NSInteger);
            NSControlStateValue::from_raw(v)
        }
    }

    pub fn set_button_type(&self, button_type: NSButtonType) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setButtonType:"),
                fn(Id, Sel, NSUInteger) -> (),
                button_type as NSUInteger
            )
        }
    }

    pub fn set_bezel_style(&self, style: NSBezelStyle) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setBezelStyle:"),
                fn(Id, Sel, NSUInteger) -> (),
                style as NSUInteger
            )
        }
    }

    pub fn set_image(&self, image: Id) {
        unsafe {
            msg_send!(self.0, sel!("setImage:"), fn(Id, Sel, Id) -> (), image)
        }
    }

    pub fn set_image_position(&self, position: NSCellImagePosition) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setImagePosition:"),
                fn(Id, Sel, NSUInteger) -> (),
                position as NSUInteger
            )
        }
    }

    pub fn set_key_equivalent(&self, key: &str) {
        unsafe {
            let ns = nsstring(key);
            msg_send!(self.0, sel!("setKeyEquivalent:"), fn(Id, Sel, Id) -> (), ns)
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

    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            msg_send!(self.0, sel!("setEnabled:"), fn(Id, Sel, BOOL) -> (), from_bool(enabled))
        }
    }
}

impl Clone for NSButton {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSButton {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
