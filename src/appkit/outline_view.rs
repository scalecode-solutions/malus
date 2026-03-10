//! NSOutlineView — a hierarchical data display (tree view).

use crate::runtime::*;

/// A view that displays hierarchical data organized into rows and columns.
pub struct NSOutlineView(pub(super) Id);

impl NSOutlineView {
    pub fn as_raw(&self) -> Id { self.0 }

    /// Create a new outline view with the given frame.
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSOutlineView") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    pub fn reload_data(&self) {
        unsafe {
            msg_send!(self.0, sel!("reloadData"), fn(Id, Sel) -> ());
        }
    }

    /// Reload a specific item and optionally its children.
    pub fn reload_item(&self, item: Id, children: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("reloadItem:reloadChildren:"),
                fn(Id, Sel, Id, BOOL) -> (),
                item, from_bool(children)
            );
        }
    }

    /// Expand an item and optionally all its children.
    pub fn expand_item(&self, item: Id, expand_children: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("expandItem:expandChildren:"),
                fn(Id, Sel, Id, BOOL) -> (),
                item, from_bool(expand_children)
            );
        }
    }

    /// Collapse an item and optionally all its children.
    pub fn collapse_item(&self, item: Id, collapse_children: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("collapseItem:collapseChildren:"),
                fn(Id, Sel, Id, BOOL) -> (),
                item, from_bool(collapse_children)
            );
        }
    }

    pub fn set_delegate(&self, delegate: Id) {
        unsafe {
            msg_send!(self.0, sel!("setDelegate:"), fn(Id, Sel, Id) -> (), delegate);
        }
    }

    pub fn set_data_source(&self, data_source: Id) {
        unsafe {
            msg_send!(self.0, sel!("setDataSource:"), fn(Id, Sel, Id) -> (), data_source);
        }
    }

    /// Set the outline table column (raw NSTableColumn Id).
    pub fn set_outline_table_column(&self, column: Id) {
        unsafe {
            msg_send!(self.0, sel!("setOutlineTableColumn:"), fn(Id, Sel, Id) -> (), column);
        }
    }

    pub fn set_autoresizes_outline_column(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setAutoresizesOutlineColumn:"), fn(Id, Sel, BOOL) -> (), from_bool(flag));
        }
    }

    pub fn selected_row(&self) -> i64 {
        unsafe { msg_send!(self.0, sel!("selectedRow"), fn(Id, Sel) -> NSInteger) as i64 }
    }
}

impl Clone for NSOutlineView {
    fn clone(&self) -> Self { Self(unsafe { retain(self.0) }) }
}

impl Drop for NSOutlineView {
    fn drop(&mut self) { unsafe { release(self.0) } }
}
