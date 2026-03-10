//! NSTableView and NSTableColumn — columnar data display.

use crate::runtime::*;

/// Column autoresizing styles for NSTableView.
#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NSTableViewColumnAutoresizingStyle {
    None = 0,
    Uniform = 1,
    Sequential = 2,
    ReverseSequential = 3,
    LastColumnOnly = 4,
    FirstColumnOnly = 5,
}

/// Table view visual styles (macOS 11+).
#[repr(isize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NSTableViewStyle {
    Automatic = 0,
    FullWidth = 1,
    Inset = 2,
    SourceList = 3,
    Plain = 4,
}

// ============================================================================
// NSTableColumn
// ============================================================================

/// A column in a table view.
pub struct NSTableColumn(pub(super) Id);

impl NSTableColumn {
    pub fn as_raw(&self) -> Id { self.0 }

    /// Create a new table column with a string identifier.
    pub fn new(identifier: &str) -> Self {
        unsafe {
            let obj = alloc(cls!("NSTableColumn") as Id);
            let ns_id = nsstring(identifier);
            let obj: Id = msg_send!(obj, sel!("initWithIdentifier:"), fn(Id, Sel, Id) -> Id, ns_id);
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

    pub fn set_width(&self, width: CGFloat) {
        unsafe {
            msg_send!(self.0, sel!("setWidth:"), fn(Id, Sel, CGFloat) -> (), width);
        }
    }

    pub fn width(&self) -> CGFloat {
        unsafe { msg_send!(self.0, sel!("width"), fn(Id, Sel) -> CGFloat) }
    }

    pub fn set_min_width(&self, w: CGFloat) {
        unsafe {
            msg_send!(self.0, sel!("setMinWidth:"), fn(Id, Sel, CGFloat) -> (), w);
        }
    }

    pub fn set_max_width(&self, w: CGFloat) {
        unsafe {
            msg_send!(self.0, sel!("setMaxWidth:"), fn(Id, Sel, CGFloat) -> (), w);
        }
    }

    pub fn set_resizing_mask(&self, mask: NSUInteger) {
        unsafe {
            msg_send!(self.0, sel!("setResizingMask:"), fn(Id, Sel, NSUInteger) -> (), mask);
        }
    }

    pub fn set_editable(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setEditable:"), fn(Id, Sel, BOOL) -> (), from_bool(flag));
        }
    }
}

impl Clone for NSTableColumn {
    fn clone(&self) -> Self { Self(unsafe { retain(self.0) }) }
}

impl Drop for NSTableColumn {
    fn drop(&mut self) { unsafe { release(self.0) } }
}

// ============================================================================
// NSTableView
// ============================================================================

/// A view that displays data in rows and columns.
pub struct NSTableView(pub(super) Id);

impl NSTableView {
    pub fn as_raw(&self) -> Id { self.0 }

    /// Create a new table view with the given frame.
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSTableView") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    pub fn add_table_column(&self, column: &NSTableColumn) {
        unsafe {
            msg_send!(self.0, sel!("addTableColumn:"), fn(Id, Sel, Id) -> (), column.0);
        }
    }

    pub fn remove_table_column(&self, column: &NSTableColumn) {
        unsafe {
            msg_send!(self.0, sel!("removeTableColumn:"), fn(Id, Sel, Id) -> (), column.0);
        }
    }

    pub fn reload_data(&self) {
        unsafe {
            msg_send!(self.0, sel!("reloadData"), fn(Id, Sel) -> ());
        }
    }

    pub fn number_of_rows(&self) -> i64 {
        unsafe { msg_send!(self.0, sel!("numberOfRows"), fn(Id, Sel) -> NSInteger) as i64 }
    }

    pub fn selected_row(&self) -> i64 {
        unsafe { msg_send!(self.0, sel!("selectedRow"), fn(Id, Sel) -> NSInteger) as i64 }
    }

    /// Get the selected row indexes as a raw NSIndexSet Id.
    pub fn selected_row_indexes(&self) -> Id {
        unsafe { msg_send!(self.0, sel!("selectedRowIndexes"), fn(Id, Sel) -> Id) }
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

    pub fn set_allows_multiple_selection(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setAllowsMultipleSelection:"), fn(Id, Sel, BOOL) -> (), from_bool(flag));
        }
    }

    pub fn set_allows_empty_selection(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setAllowsEmptySelection:"), fn(Id, Sel, BOOL) -> (), from_bool(flag));
        }
    }

    pub fn set_uses_alternating_row_background_colors(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setUsesAlternatingRowBackgroundColors:"), fn(Id, Sel, BOOL) -> (), from_bool(flag));
        }
    }

    pub fn set_row_height(&self, height: CGFloat) {
        unsafe {
            msg_send!(self.0, sel!("setRowHeight:"), fn(Id, Sel, CGFloat) -> (), height);
        }
    }

    pub fn set_column_autoresizing_style(&self, style: NSTableViewColumnAutoresizingStyle) {
        unsafe {
            msg_send!(self.0, sel!("setColumnAutoresizingStyle:"), fn(Id, Sel, NSUInteger) -> (), style as NSUInteger);
        }
    }

    /// Set the grid line style mask (NSTableViewGridLineStyle bit mask).
    pub fn set_grid_style_mask(&self, mask: NSUInteger) {
        unsafe {
            msg_send!(self.0, sel!("setGridStyleMask:"), fn(Id, Sel, NSUInteger) -> (), mask);
        }
    }

    /// Set the header view (raw Id, or null to hide headers).
    pub fn set_header_view(&self, view: Id) {
        unsafe {
            msg_send!(self.0, sel!("setHeaderView:"), fn(Id, Sel, Id) -> (), view);
        }
    }

    /// Set the table view style (macOS 11+).
    pub fn set_style(&self, style: NSTableViewStyle) {
        unsafe {
            msg_send!(self.0, sel!("setStyle:"), fn(Id, Sel, NSInteger) -> (), style as NSInteger);
        }
    }
}

impl Clone for NSTableView {
    fn clone(&self) -> Self { Self(unsafe { retain(self.0) }) }
}

impl Drop for NSTableView {
    fn drop(&mut self) { unsafe { release(self.0) } }
}
