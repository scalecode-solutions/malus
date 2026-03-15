//! NSSplitViewController + NSSplitViewItem — managed split views with
//! native sidebar support, collapse animations, and auto-save.

use crate::runtime::*;
use super::view_controller::NSViewController;

// ============================================================================
// NSSplitViewItemBehavior
// ============================================================================

/// The behavior of a split view item.
#[repr(isize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NSSplitViewItemBehavior {
    /// Default behavior.
    Default = 0,
    /// Sidebar behavior: collapsible, native appearance.
    Sidebar = 1,
    /// Content list behavior (middle pane in three-column layouts).
    ContentList = 2,
}

// ============================================================================
// NSSplitViewItemCollapseBehavior
// ============================================================================

/// How a split view item collapses.
#[repr(isize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NSSplitViewItemCollapseBehavior {
    /// The item uses its default collapsing behavior.
    Default = 0,
    /// The item prefers to collapse toward the leading edge.
    PreferResizingSplitViewWithFixedSiblings = 1,
    /// The item prefers to collapse toward the trailing edge.
    PreferResizingSiblingsWithFixedSplitView = 2,
    /// The item uses equal resizing when collapsing.
    UseConstraints = 3,
}

// ============================================================================
// NSSplitViewItem
// ============================================================================

/// An item managed by an NSSplitViewController.
pub struct NSSplitViewItem(pub(super) Id);

impl NSSplitViewItem {
    pub fn as_raw(&self) -> Id { self.0 }

    /// Create a split view item with the given view controller.
    pub fn with_view_controller(vc: &NSViewController) -> Self {
        unsafe {
            let raw: Id = msg_send!(
                cls!("NSSplitViewItem") as Id,
                sel!("splitViewItemWithViewController:"),
                fn(Id, Sel, Id) -> Id,
                vc.as_raw()
            );
            Self(retain(raw))
        }
    }

    /// Create a sidebar split view item.
    pub fn sidebar(vc: &NSViewController) -> Self {
        unsafe {
            let raw: Id = msg_send!(
                cls!("NSSplitViewItem") as Id,
                sel!("sidebarWithViewController:"),
                fn(Id, Sel, Id) -> Id,
                vc.as_raw()
            );
            Self(retain(raw))
        }
    }

    /// Create a content list split view item.
    pub fn content_list(vc: &NSViewController) -> Self {
        unsafe {
            let raw: Id = msg_send!(
                cls!("NSSplitViewItem") as Id,
                sel!("contentListWithViewController:"),
                fn(Id, Sel, Id) -> Id,
                vc.as_raw()
            );
            Self(retain(raw))
        }
    }

    /// Get the view controller.
    pub fn view_controller(&self) -> NSViewController {
        unsafe {
            let raw: Id = msg_send!(self.0, sel!("viewController"), fn(Id, Sel) -> Id);
            NSViewController::from_raw(raw)
        }
    }

    /// Whether the item is collapsed.
    pub fn is_collapsed(&self) -> bool {
        unsafe { to_bool(msg_send!(self.0, sel!("isCollapsed"), fn(Id, Sel) -> BOOL)) }
    }

    /// Set collapsed state (animatable).
    pub fn set_collapsed(&self, collapsed: bool) {
        unsafe {
            msg_send!(self.0, sel!("setCollapsed:"), fn(Id, Sel, BOOL) -> (), from_bool(collapsed));
        }
    }

    /// Whether the item can collapse.
    pub fn can_collapse(&self) -> bool {
        unsafe { to_bool(msg_send!(self.0, sel!("canCollapse"), fn(Id, Sel) -> BOOL)) }
    }

    /// Set whether the item can collapse.
    pub fn set_can_collapse(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setCanCollapse:"), fn(Id, Sel, BOOL) -> (), from_bool(flag));
        }
    }

    /// Set the collapse behavior.
    pub fn set_collapse_behavior(&self, behavior: NSSplitViewItemCollapseBehavior) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setCollapseBehavior:"),
                fn(Id, Sel, NSInteger) -> (),
                behavior as NSInteger
            );
        }
    }

    /// Set the minimum thickness (width for vertical splits, height for horizontal).
    pub fn set_minimum_thickness(&self, thickness: CGFloat) {
        unsafe {
            msg_send!(self.0, sel!("setMinimumThickness:"), fn(Id, Sel, CGFloat) -> (), thickness);
        }
    }

    /// Set the maximum thickness.
    pub fn set_maximum_thickness(&self, thickness: CGFloat) {
        unsafe {
            msg_send!(self.0, sel!("setMaximumThickness:"), fn(Id, Sel, CGFloat) -> (), thickness);
        }
    }

    /// Set the preferred width fraction (0.0 to 1.0).
    pub fn set_preferred_thickness_fraction(&self, fraction: CGFloat) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setPreferredThicknessFraction:"),
                fn(Id, Sel, CGFloat) -> (),
                fraction
            );
        }
    }

    /// Set the holding priority. Higher priority means the item resists resizing.
    pub fn set_holding_priority(&self, priority: f32) {
        unsafe {
            msg_send!(self.0, sel!("setHoldingPriority:"), fn(Id, Sel, f32) -> (), priority);
        }
    }
}

impl Clone for NSSplitViewItem {
    fn clone(&self) -> Self { Self(unsafe { retain(self.0) }) }
}

impl Drop for NSSplitViewItem {
    fn drop(&mut self) { unsafe { release(self.0) } }
}

// ============================================================================
// NSSplitViewController
// ============================================================================

/// A container view controller that manages child view controllers
/// in a split view interface.
pub struct NSSplitViewController(pub(super) Id);

impl NSSplitViewController {
    pub fn as_raw(&self) -> Id { self.0 }

    /// Create a new split view controller.
    pub fn new() -> Self {
        unsafe {
            let obj = alloc_init(cls!("NSSplitViewController") as Id);
            Self(obj)
        }
    }

    /// The managed split view.
    pub fn split_view(&self) -> Id {
        unsafe { msg_send!(self.0, sel!("splitView"), fn(Id, Sel) -> Id) }
    }

    /// Add a split view item.
    pub fn add_split_view_item(&self, item: &NSSplitViewItem) {
        unsafe {
            msg_send!(self.0, sel!("addSplitViewItem:"), fn(Id, Sel, Id) -> (), item.as_raw());
        }
    }

    /// Insert a split view item at a specific index.
    pub fn insert_split_view_item(&self, item: &NSSplitViewItem, at_index: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("insertSplitViewItem:atIndex:"),
                fn(Id, Sel, Id, NSInteger) -> (),
                item.as_raw(), at_index as NSInteger
            );
        }
    }

    /// Remove a split view item.
    pub fn remove_split_view_item(&self, item: &NSSplitViewItem) {
        unsafe {
            msg_send!(self.0, sel!("removeSplitViewItem:"), fn(Id, Sel, Id) -> (), item.as_raw());
        }
    }

    /// Number of split view items.
    pub fn split_view_item_count(&self) -> usize {
        unsafe {
            let arr: Id = msg_send!(self.0, sel!("splitViewItems"), fn(Id, Sel) -> Id);
            msg_send!(arr, sel!("count"), fn(Id, Sel) -> NSUInteger)
        }
    }

    /// Toggle the sidebar (first item with sidebar behavior).
    pub fn toggle_sidebar(&self, sender: Id) {
        unsafe {
            msg_send!(self.0, sel!("toggleSidebar:"), fn(Id, Sel, Id) -> (), sender);
        }
    }

    /// The controller's view (inherited from NSViewController).
    pub fn view(&self) -> Id {
        unsafe { msg_send!(self.0, sel!("view"), fn(Id, Sel) -> Id) }
    }

    /// Set the controller's title.
    pub fn set_title(&self, title: &str) {
        unsafe {
            let ns = nsstring(title);
            msg_send!(self.0, sel!("setTitle:"), fn(Id, Sel, Id) -> (), ns);
        }
    }

    /// Wrap a raw pointer (retained).
    pub unsafe fn from_raw(ptr: Id) -> Self {
        Self(retain(ptr))
    }
}

impl Clone for NSSplitViewController {
    fn clone(&self) -> Self { Self(unsafe { retain(self.0) }) }
}

impl Drop for NSSplitViewController {
    fn drop(&mut self) { unsafe { release(self.0) } }
}
