//! NSSplitView — a view that arranges two or more subviews with dividers.

use crate::runtime::*;

/// The style of the divider drawn between split view panes.
#[repr(isize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum NSSplitViewDividerStyle {
    Thick = 1,
    Thin = 2,
    PaneSplitter = 3,
}

/// A view that arranges subviews in a linear stack separated by dividers.
pub struct NSSplitView(pub(super) Id);

impl NSSplitView {
    pub fn as_raw(&self) -> Id { self.0 }

    /// Create a new split view with the given frame.
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSSplitView") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    pub fn set_vertical(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setVertical:"), fn(Id, Sel, BOOL) -> (), from_bool(flag));
        }
    }

    pub fn is_vertical(&self) -> bool {
        unsafe { to_bool(msg_send!(self.0, sel!("isVertical"), fn(Id, Sel) -> BOOL)) }
    }

    /// Add a subview to the split view.
    pub fn add_subview(&self, view: Id) {
        unsafe {
            msg_send!(self.0, sel!("addSubview:"), fn(Id, Sel, Id) -> (), view);
        }
    }

    pub fn set_divider_style(&self, style: NSSplitViewDividerStyle) {
        unsafe {
            msg_send!(self.0, sel!("setDividerStyle:"), fn(Id, Sel, NSInteger) -> (), style as NSInteger);
        }
    }

    pub fn set_delegate(&self, delegate: Id) {
        unsafe {
            msg_send!(self.0, sel!("setDelegate:"), fn(Id, Sel, Id) -> (), delegate);
        }
    }

    /// Adjusts the sizes of the receiver's subviews.
    pub fn adjust_subviews(&self) {
        unsafe {
            msg_send!(self.0, sel!("adjustSubviews"), fn(Id, Sel) -> ());
        }
    }

    /// Set the holding priority for a subview at the given index.
    pub fn set_holding_priority(&self, priority: f32, at_index: i64) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setHoldingPriority:forSubviewAtIndex:"),
                fn(Id, Sel, f32, i64) -> (),
                priority, at_index
            );
        }
    }

    /// The number of subviews in the split view.
    pub fn number_of_subviews(&self) -> usize {
        unsafe {
            let arr: Id = msg_send!(self.0, sel!("subviews"), fn(Id, Sel) -> Id);
            msg_send!(arr, sel!("count"), fn(Id, Sel) -> NSUInteger)
        }
    }
}

impl Clone for NSSplitView {
    fn clone(&self) -> Self { Self(unsafe { retain(self.0) }) }
}

impl Drop for NSSplitView {
    fn drop(&mut self) { unsafe { release(self.0) } }
}
