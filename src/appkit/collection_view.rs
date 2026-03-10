//! NSCollectionView — a grid-based content display.

use crate::runtime::*;

/// A view that displays an ordered collection of data items using
/// customizable layouts.
pub struct NSCollectionView(pub(super) Id);

impl NSCollectionView {
    pub fn as_raw(&self) -> Id { self.0 }

    /// Create a new collection view with the given frame.
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSCollectionView") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    pub fn reload_data(&self) {
        unsafe {
            msg_send!(self.0, sel!("reloadData"), fn(Id, Sel) -> ());
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

    pub fn set_selectable(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setSelectable:"), fn(Id, Sel, BOOL) -> (), from_bool(flag));
        }
    }

    pub fn set_allows_multiple_selection(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setAllowsMultipleSelection:"), fn(Id, Sel, BOOL) -> (), from_bool(flag));
        }
    }

    /// Set the background colors (raw NSArray of NSColor).
    pub fn set_background_colors(&self, colors: Id) {
        unsafe {
            msg_send!(self.0, sel!("setBackgroundColors:"), fn(Id, Sel, Id) -> (), colors);
        }
    }

    /// Register a class for creating items with the given identifier.
    pub fn register_class_for_item(&self, cls: Id, identifier: &str) {
        unsafe {
            let ns_id = nsstring(identifier);
            msg_send!(
                self.0,
                sel!("registerClass:forItemWithIdentifier:"),
                fn(Id, Sel, Id, Id) -> (),
                cls, ns_id
            );
        }
    }

    /// Get the selection index paths as a raw NSSet of NSIndexPath.
    pub fn selection_index_paths(&self) -> Id {
        unsafe { msg_send!(self.0, sel!("selectionIndexPaths"), fn(Id, Sel) -> Id) }
    }
}

impl Clone for NSCollectionView {
    fn clone(&self) -> Self { Self(unsafe { retain(self.0) }) }
}

impl Drop for NSCollectionView {
    fn drop(&mut self) { unsafe { release(self.0) } }
}
