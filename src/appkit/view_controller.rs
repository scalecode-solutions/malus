//! NSViewController — base view controller for macOS.

use crate::runtime::*;

/// A controller that manages a view.
pub struct NSViewController(pub(super) Id);

impl NSViewController {
    pub fn as_raw(&self) -> Id { self.0 }

    /// Create a new empty view controller.
    pub fn new() -> Self {
        unsafe {
            let obj = alloc_init(cls!("NSViewController") as Id);
            Self(obj)
        }
    }

    /// Get the controller's view.
    pub fn view(&self) -> Id {
        unsafe { msg_send!(self.0, sel!("view"), fn(Id, Sel) -> Id) }
    }

    /// Set the controller's view.
    pub fn set_view(&self, view: Id) {
        unsafe {
            msg_send!(self.0, sel!("setView:"), fn(Id, Sel, Id) -> (), view);
        }
    }

    /// The view controller's title.
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

    /// Wrap a raw pointer without retaining (takes ownership).
    pub unsafe fn from_raw_owned(ptr: Id) -> Self {
        Self(ptr)
    }
}

impl Clone for NSViewController {
    fn clone(&self) -> Self { Self(unsafe { retain(self.0) }) }
}

impl Drop for NSViewController {
    fn drop(&mut self) { unsafe { release(self.0) } }
}
