//! NSBox — a visual grouping element with an optional title.

use crate::runtime::*;

/// The type of box.
#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NSBoxType {
    Primary = 0,
    Secondary = 1,
    Separator = 2,
    Custom = 4,
}

/// Border type for NSBox.
#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NSBorderType {
    None = 0,
    Line = 1,
    Bezel = 2,
    Groove = 3,
}

/// Title position for NSBox.
#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NSTitlePosition {
    NoTitle = 0,
    AboveTop = 1,
    AtTop = 2,
    BelowTop = 3,
    AboveBottom = 4,
    AtBottom = 5,
    BelowBottom = 6,
}

/// A stylized view for grouping related UI elements, with an optional title.
pub struct NSBox(pub(super) Id);

impl NSBox {
    pub fn as_raw(&self) -> Id { self.0 }

    /// Create a new box with the given frame.
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSBox") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    /// Create a horizontal separator.
    pub fn separator() -> Self {
        unsafe {
            let obj = alloc(cls!("NSBox") as Id);
            let obj: Id = msg_send!(
                obj,
                sel!("initWithFrame:"),
                fn(Id, Sel, CGRect) -> Id,
                CGRect::zero()
            );
            msg_send!(obj, sel!("setBoxType:"), fn(Id, Sel, NSUInteger) -> (), NSBoxType::Separator as NSUInteger);
            Self(obj)
        }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            let ns = nsstring(title);
            msg_send!(self.0, sel!("setTitle:"), fn(Id, Sel, Id) -> (), ns);
        }
    }

    pub fn title(&self) -> String {
        unsafe {
            let ns: Id = msg_send!(self.0, sel!("title"), fn(Id, Sel) -> Id);
            from_nsstring(ns)
        }
    }

    /// Set the content view (raw NSView Id).
    pub fn set_content_view(&self, view: Id) {
        unsafe {
            msg_send!(self.0, sel!("setContentView:"), fn(Id, Sel, Id) -> (), view);
        }
    }

    pub fn set_box_type(&self, box_type: NSBoxType) {
        unsafe {
            msg_send!(self.0, sel!("setBoxType:"), fn(Id, Sel, NSUInteger) -> (), box_type as NSUInteger);
        }
    }

    pub fn set_border_type(&self, border: NSBorderType) {
        unsafe {
            msg_send!(self.0, sel!("setBorderType:"), fn(Id, Sel, NSUInteger) -> (), border as NSUInteger);
        }
    }

    pub fn set_title_position(&self, position: NSTitlePosition) {
        unsafe {
            msg_send!(self.0, sel!("setTitlePosition:"), fn(Id, Sel, NSUInteger) -> (), position as NSUInteger);
        }
    }

    pub fn set_transparent(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setTransparent:"), fn(Id, Sel, BOOL) -> (), from_bool(flag));
        }
    }

    pub fn is_transparent(&self) -> bool {
        unsafe { to_bool(msg_send!(self.0, sel!("isTransparent"), fn(Id, Sel) -> BOOL)) }
    }

    /// Set the margins around the content view.
    pub fn set_content_view_margins(&self, size: CGSize) {
        unsafe {
            msg_send!(self.0, sel!("setContentViewMargins:"), fn(Id, Sel, CGSize) -> (), size);
        }
    }
}

impl Clone for NSBox {
    fn clone(&self) -> Self { Self(unsafe { retain(self.0) }) }
}

impl Drop for NSBox {
    fn drop(&mut self) { unsafe { release(self.0) } }
}
