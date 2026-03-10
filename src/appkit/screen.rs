//! NSScreen — display information.

use crate::runtime::*;

/// A display screen.
pub struct NSScreen(pub(super) Id);

impl NSScreen {
    /// Return the raw ObjC pointer (does not transfer ownership).
    pub fn as_raw(&self) -> Id {
        self.0
    }

    /// Get the main screen, if available (may be nil in headless environments).
    pub fn main_screen() -> Option<Self> {
        unsafe {
            let raw: Id = msg_send!(
                cls!("NSScreen") as Id,
                sel!("mainScreen"),
                fn(Id, Sel) -> Id
            );
            if raw.is_null() {
                None
            } else {
                Some(Self(retain(raw)))
            }
        }
    }

    /// Get all screens.
    pub fn screens() -> Vec<Self> {
        unsafe {
            let arr: Id = msg_send!(
                cls!("NSScreen") as Id,
                sel!("screens"),
                fn(Id, Sel) -> Id
            );
            let count: NSUInteger = msg_send!(arr, sel!("count"), fn(Id, Sel) -> NSUInteger);
            let mut result = Vec::with_capacity(count);
            for i in 0..count {
                let obj: Id = msg_send!(
                    arr,
                    sel!("objectAtIndex:"),
                    fn(Id, Sel, NSUInteger) -> Id,
                    i
                );
                result.push(Self(retain(obj)));
            }
            result
        }
    }

    /// Get the screen's full frame.
    pub fn frame(&self) -> CGRect {
        unsafe {
            msg_send!(self.0, sel!("frame"), fn(Id, Sel) -> CGRect)
        }
    }

    /// Get the visible frame (excludes menu bar, dock, etc.).
    pub fn visible_frame(&self) -> CGRect {
        unsafe {
            msg_send!(self.0, sel!("visibleFrame"), fn(Id, Sel) -> CGRect)
        }
    }

    /// Get the backing scale factor (e.g. 2.0 for Retina).
    pub fn backing_scale_factor(&self) -> CGFloat {
        unsafe {
            msg_send!(self.0, sel!("backingScaleFactor"), fn(Id, Sel) -> CGFloat)
        }
    }
}

impl Clone for NSScreen {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSScreen {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
