//! NSTextField — a single-line or multi-line text control.

use crate::runtime::*;

// ============================================================================
// NSTextAlignment
// ============================================================================

#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NSTextAlignment {
    Left = 0,
    Right = 1,
    Center = 2,
    Justified = 3,
    Natural = 4,
}

// ============================================================================
// NSLineBreakMode
// ============================================================================

#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NSLineBreakMode {
    ByWordWrapping = 0,
    ByCharWrapping = 1,
    ByClipping = 2,
    ByTruncatingHead = 3,
    ByTruncatingTail = 4,
    ByTruncatingMiddle = 5,
}

// ============================================================================
// NSTextField
// ============================================================================

pub struct NSTextField(pub(super) Id);

impl NSTextField {
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSTextField") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    /// Create a non-editable, borderless label.
    pub fn label(text: &str) -> Self {
        unsafe {
            let ns = nsstring(text);
            let raw: Id = msg_send!(
                cls!("NSTextField") as Id,
                sel!("labelWithString:"),
                fn(Id, Sel, Id) -> Id,
                ns
            );
            Self(retain(raw))
        }
    }

    /// Create a wrapping label.
    pub fn wrapping_label(text: &str) -> Self {
        unsafe {
            let ns = nsstring(text);
            let raw: Id = msg_send!(
                cls!("NSTextField") as Id,
                sel!("wrappingLabelWithString:"),
                fn(Id, Sel, Id) -> Id,
                ns
            );
            Self(retain(raw))
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }

    pub fn set_string_value(&self, val: &str) {
        unsafe {
            let ns = nsstring(val);
            msg_send!(self.0, sel!("setStringValue:"), fn(Id, Sel, Id) -> (), ns)
        }
    }

    pub fn string_value(&self) -> String {
        unsafe {
            let ns: Id = msg_send!(self.0, sel!("stringValue"), fn(Id, Sel) -> Id);
            from_nsstring(ns)
        }
    }

    pub fn set_placeholder(&self, text: &str) {
        unsafe {
            let ns = nsstring(text);
            msg_send!(self.0, sel!("setPlaceholderString:"), fn(Id, Sel, Id) -> (), ns)
        }
    }

    pub fn placeholder(&self) -> String {
        unsafe {
            let ns: Id = msg_send!(self.0, sel!("placeholderString"), fn(Id, Sel) -> Id);
            from_nsstring(ns)
        }
    }

    pub fn set_editable(&self, editable: bool) {
        unsafe {
            msg_send!(self.0, sel!("setEditable:"), fn(Id, Sel, BOOL) -> (), from_bool(editable))
        }
    }

    pub fn is_editable(&self) -> bool {
        unsafe { to_bool(msg_send!(self.0, sel!("isEditable"), fn(Id, Sel) -> BOOL)) }
    }

    pub fn set_selectable(&self, selectable: bool) {
        unsafe {
            msg_send!(self.0, sel!("setSelectable:"), fn(Id, Sel, BOOL) -> (), from_bool(selectable))
        }
    }

    pub fn set_bordered(&self, bordered: bool) {
        unsafe {
            msg_send!(self.0, sel!("setBordered:"), fn(Id, Sel, BOOL) -> (), from_bool(bordered))
        }
    }

    pub fn set_bezeled(&self, bezeled: bool) {
        unsafe {
            msg_send!(self.0, sel!("setBezeled:"), fn(Id, Sel, BOOL) -> (), from_bool(bezeled))
        }
    }

    pub fn set_draws_background(&self, draws: bool) {
        unsafe {
            msg_send!(self.0, sel!("setDrawsBackground:"), fn(Id, Sel, BOOL) -> (), from_bool(draws))
        }
    }

    pub fn set_text_color(&self, color: Id) {
        unsafe {
            msg_send!(self.0, sel!("setTextColor:"), fn(Id, Sel, Id) -> (), color)
        }
    }

    pub fn set_background_color(&self, color: Id) {
        unsafe {
            msg_send!(self.0, sel!("setBackgroundColor:"), fn(Id, Sel, Id) -> (), color)
        }
    }

    pub fn set_font(&self, font: Id) {
        unsafe {
            msg_send!(self.0, sel!("setFont:"), fn(Id, Sel, Id) -> (), font)
        }
    }

    pub fn set_alignment(&self, alignment: NSTextAlignment) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setAlignment:"),
                fn(Id, Sel, NSUInteger) -> (),
                alignment as NSUInteger
            )
        }
    }

    pub fn set_line_break_mode(&self, mode: NSLineBreakMode) {
        unsafe {
            let cell: Id = msg_send!(self.0, sel!("cell"), fn(Id, Sel) -> Id);
            msg_send!(
                cell,
                sel!("setLineBreakMode:"),
                fn(Id, Sel, NSUInteger) -> (),
                mode as NSUInteger
            )
        }
    }

    pub fn set_maximum_number_of_lines(&self, lines: i64) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setMaximumNumberOfLines:"),
                fn(Id, Sel, NSInteger) -> (),
                lines as NSInteger
            )
        }
    }

    pub fn set_delegate(&self, delegate: Id) {
        unsafe {
            msg_send!(self.0, sel!("setDelegate:"), fn(Id, Sel, Id) -> (), delegate)
        }
    }

    pub fn set_uses_single_line_mode(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setUsesSingleLineMode:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            )
        }
    }
}

impl Clone for NSTextField {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSTextField {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
