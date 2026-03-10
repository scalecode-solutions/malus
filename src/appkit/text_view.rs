//! NSTextView — a rich, multi-line text editing view.

use crate::runtime::*;

pub struct NSTextView(pub(super) Id);

impl NSTextView {
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSTextView") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }

    pub fn string(&self) -> String {
        unsafe {
            let ns: Id = msg_send!(self.0, sel!("string"), fn(Id, Sel) -> Id);
            from_nsstring(ns)
        }
    }

    pub fn set_string(&self, text: &str) {
        unsafe {
            let ns = nsstring(text);
            msg_send!(self.0, sel!("setString:"), fn(Id, Sel, Id) -> (), ns)
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

    pub fn set_rich_text(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setRichText:"), fn(Id, Sel, BOOL) -> (), from_bool(flag))
        }
    }

    pub fn set_font(&self, font: Id) {
        unsafe {
            msg_send!(self.0, sel!("setFont:"), fn(Id, Sel, Id) -> (), font)
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

    pub fn set_draws_background(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setDrawsBackground:"), fn(Id, Sel, BOOL) -> (), from_bool(flag))
        }
    }

    pub fn set_allows_undo(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setAllowsUndo:"), fn(Id, Sel, BOOL) -> (), from_bool(flag))
        }
    }

    pub fn set_delegate(&self, delegate: Id) {
        unsafe {
            msg_send!(self.0, sel!("setDelegate:"), fn(Id, Sel, Id) -> (), delegate)
        }
    }

    /// Scroll to the end of the text.
    pub fn scroll_to_end(&self) {
        unsafe {
            let ns_str: Id = msg_send!(self.0, sel!("string"), fn(Id, Sel) -> Id);
            let len: NSUInteger = msg_send!(ns_str, sel!("length"), fn(Id, Sel) -> NSUInteger);
            // NSRange { location, length }
            msg_send!(
                self.0,
                sel!("scrollRangeToVisible:"),
                fn(Id, Sel, NSUInteger, NSUInteger) -> (),
                len,
                0_usize
            )
        }
    }

    /// Returns the selected range as (location, length).
    pub fn selected_range(&self) -> (usize, usize) {
        unsafe {
            // NSRange is { NSUInteger location, NSUInteger length }
            // On 64-bit this is two u64 values packed into a struct.
            // We receive it as two usize values since msg_send returns in registers.
            #[repr(C)]
            struct NSRange {
                location: NSUInteger,
                length: NSUInteger,
            }
            let range: NSRange = msg_send!(self.0, sel!("selectedRange"), fn(Id, Sel) -> NSRange);
            (range.location, range.length)
        }
    }
}

impl Clone for NSTextView {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSTextView {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
