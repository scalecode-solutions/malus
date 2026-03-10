//! CATransaction — batch implicit animations and property changes.

use crate::runtime::*;

/// Groups Core Animation operations into atomic updates.
///
/// All methods are class methods called on the `CATransaction` class directly.
pub struct CATransaction;

impl CATransaction {
    /// Begin a new transaction.
    pub fn begin() {
        unsafe {
            msg_send!(cls!("CATransaction") as Id, sel!("begin"), fn(Id, Sel) -> ())
        }
    }

    /// Commit the current transaction.
    pub fn commit() {
        unsafe {
            msg_send!(cls!("CATransaction") as Id, sel!("commit"), fn(Id, Sel) -> ())
        }
    }

    /// Set the animation duration for the current transaction.
    pub fn set_animation_duration(dur: f64) {
        unsafe {
            msg_send!(cls!("CATransaction") as Id, sel!("setAnimationDuration:"), fn(Id, Sel, f64) -> (), dur)
        }
    }

    /// Disable or enable implicit animations in the current transaction.
    pub fn set_disable_actions(disable: bool) {
        unsafe {
            msg_send!(cls!("CATransaction") as Id, sel!("setDisableActions:"), fn(Id, Sel, BOOL) -> (), from_bool(disable))
        }
    }

    /// Flush all pending transactions.
    pub fn flush() {
        unsafe {
            msg_send!(cls!("CATransaction") as Id, sel!("flush"), fn(Id, Sel) -> ())
        }
    }
}
