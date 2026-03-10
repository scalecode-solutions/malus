//! NSScrollView — a scrollable container view.

use crate::runtime::*;

/// Border type for scroll views and boxes.
#[repr(isize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NSBorderType {
    None = 0,
    Line = 1,
    Bezel = 2,
    Groove = 3,
}

/// A view that displays a portion of a document view and provides scroll bars.
pub struct NSScrollView(pub(super) Id);

impl NSScrollView {
    pub fn as_raw(&self) -> Id { self.0 }

    /// Create a new scroll view with the given frame.
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSScrollView") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    /// Set the document view (the scrollable content).
    pub fn set_document_view(&self, view: Id) {
        unsafe {
            msg_send!(self.0, sel!("setDocumentView:"), fn(Id, Sel, Id) -> (), view);
        }
    }

    /// Get the document view, if any.
    pub fn document_view(&self) -> Option<Id> {
        unsafe {
            let raw: Id = msg_send!(self.0, sel!("documentView"), fn(Id, Sel) -> Id);
            if raw.is_null() { None } else { Some(raw) }
        }
    }

    pub fn set_has_vertical_scroller(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setHasVerticalScroller:"), fn(Id, Sel, BOOL) -> (), from_bool(flag));
        }
    }

    pub fn has_vertical_scroller(&self) -> bool {
        unsafe { to_bool(msg_send!(self.0, sel!("hasVerticalScroller"), fn(Id, Sel) -> BOOL)) }
    }

    pub fn set_has_horizontal_scroller(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setHasHorizontalScroller:"), fn(Id, Sel, BOOL) -> (), from_bool(flag));
        }
    }

    pub fn has_horizontal_scroller(&self) -> bool {
        unsafe { to_bool(msg_send!(self.0, sel!("hasHorizontalScroller"), fn(Id, Sel) -> BOOL)) }
    }

    pub fn set_autohides_scrollers(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setAutohidesScrollers:"), fn(Id, Sel, BOOL) -> (), from_bool(flag));
        }
    }

    pub fn set_border_type(&self, border: NSBorderType) {
        unsafe {
            msg_send!(self.0, sel!("setBorderType:"), fn(Id, Sel, NSUInteger) -> (), border as NSUInteger);
        }
    }

    pub fn set_draws_background(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setDrawsBackground:"), fn(Id, Sel, BOOL) -> (), from_bool(flag));
        }
    }

    /// The size of the content area (inside scroll bars and borders).
    pub fn content_size(&self) -> CGSize {
        unsafe { msg_send!(self.0, sel!("contentSize"), fn(Id, Sel) -> CGSize) }
    }

    /// Set the background color (raw NSColor Id).
    pub fn set_background_color(&self, color: Id) {
        unsafe {
            msg_send!(self.0, sel!("setBackgroundColor:"), fn(Id, Sel, Id) -> (), color);
        }
    }
}

impl Clone for NSScrollView {
    fn clone(&self) -> Self { Self(unsafe { retain(self.0) }) }
}

impl Drop for NSScrollView {
    fn drop(&mut self) { unsafe { release(self.0) } }
}
