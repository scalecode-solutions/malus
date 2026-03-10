//! NSResponder — the base class for event handling.

use crate::runtime::*;

/// A responder in the responder chain.
pub struct NSResponder(pub(super) Id);

impl NSResponder {
    /// Return the raw ObjC pointer (does not transfer ownership).
    pub fn as_raw(&self) -> Id {
        self.0
    }

    /// Get the next responder in the chain, if any.
    pub fn next_responder(&self) -> Option<NSResponder> {
        unsafe {
            let raw: Id = msg_send!(self.0, sel!("nextResponder"), fn(Id, Sel) -> Id);
            if raw.is_null() {
                None
            } else {
                Some(NSResponder(retain(raw)))
            }
        }
    }

    /// Set the next responder (pass a raw Id).
    pub fn set_next_responder(&self, responder: Id) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setNextResponder:"),
                fn(Id, Sel, Id) -> (),
                responder
            );
        }
    }
}

impl Clone for NSResponder {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSResponder {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
