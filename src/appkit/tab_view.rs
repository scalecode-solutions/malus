//! NSTabView and NSTabViewItem — tabbed container views.

use crate::runtime::*;

// ============================================================================
// NSTabViewItem
// ============================================================================

/// An item in a tab view. Each item manages its own label and content view.
pub struct NSTabViewItem(pub(super) Id);

impl NSTabViewItem {
    pub fn as_raw(&self) -> Id { self.0 }

    /// Create a new tab view item with a string identifier.
    pub fn new(identifier: &str) -> Self {
        unsafe {
            let obj = alloc(cls!("NSTabViewItem") as Id);
            let ns_id = nsstring(identifier);
            let obj: Id = msg_send!(obj, sel!("initWithIdentifier:"), fn(Id, Sel, Id) -> Id, ns_id);
            Self(obj)
        }
    }

    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns = nsstring(label);
            msg_send!(self.0, sel!("setLabel:"), fn(Id, Sel, Id) -> (), ns);
        }
    }

    pub fn label(&self) -> String {
        unsafe {
            let ns: Id = msg_send!(self.0, sel!("label"), fn(Id, Sel) -> Id);
            from_nsstring(ns)
        }
    }

    /// Set the content view for this tab (raw NSView Id).
    pub fn set_view(&self, view: Id) {
        unsafe {
            msg_send!(self.0, sel!("setView:"), fn(Id, Sel, Id) -> (), view);
        }
    }
}

impl Clone for NSTabViewItem {
    fn clone(&self) -> Self { Self(unsafe { retain(self.0) }) }
}

impl Drop for NSTabViewItem {
    fn drop(&mut self) { unsafe { release(self.0) } }
}

// ============================================================================
// NSTabView
// ============================================================================

/// A multipage interface with tabs for switching between views.
pub struct NSTabView(pub(super) Id);

impl NSTabView {
    pub fn as_raw(&self) -> Id { self.0 }

    /// Create a new tab view with the given frame.
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSTabView") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    pub fn add_tab_view_item(&self, item: &NSTabViewItem) {
        unsafe {
            msg_send!(self.0, sel!("addTabViewItem:"), fn(Id, Sel, Id) -> (), item.0);
        }
    }

    pub fn remove_tab_view_item(&self, item: &NSTabViewItem) {
        unsafe {
            msg_send!(self.0, sel!("removeTabViewItem:"), fn(Id, Sel, Id) -> (), item.0);
        }
    }

    pub fn select_tab_view_item_at_index(&self, index: i64) {
        unsafe {
            msg_send!(self.0, sel!("selectTabViewItemAtIndex:"), fn(Id, Sel, NSInteger) -> (), index as NSInteger);
        }
    }

    pub fn number_of_tab_view_items(&self) -> i64 {
        unsafe {
            msg_send!(self.0, sel!("numberOfTabViewItems"), fn(Id, Sel) -> NSInteger) as i64
        }
    }

    pub fn set_delegate(&self, delegate: Id) {
        unsafe {
            msg_send!(self.0, sel!("setDelegate:"), fn(Id, Sel, Id) -> (), delegate);
        }
    }
}

impl Clone for NSTabView {
    fn clone(&self) -> Self { Self(unsafe { retain(self.0) }) }
}

impl Drop for NSTabView {
    fn drop(&mut self) { unsafe { release(self.0) } }
}
