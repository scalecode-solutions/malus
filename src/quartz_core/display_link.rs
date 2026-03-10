//! CADisplayLink — a timer that fires in sync with the display refresh rate.

use crate::runtime::*;

/// A timer object that synchronises drawing to the refresh rate of the display.
pub struct CADisplayLink(Id);

impl CADisplayLink {
    /// Create a display link that calls `selector` on `target` each frame.
    ///
    /// # Safety
    /// `target` must be a valid ObjC object that responds to `selector`.
    pub unsafe fn new_with_target(target: Id, selector: Sel) -> Self {
        let obj: Id = msg_send!(
            cls!("CADisplayLink") as Id,
            sel!("displayLinkWithTarget:selector:"),
            fn(Id, Sel, Id, Sel) -> Id,
            target,
            selector
        );
        // Class method returns autoreleased — retain for ownership.
        Self(retain(obj))
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }

    /// Add this display link to a run loop.
    /// Typically: `NSRunLoopCommonModes` (`"kCFRunLoopCommonModes"`).
    pub fn add_to_run_loop(&self, run_loop: Id, mode: Id) {
        unsafe {
            msg_send!(
                self.0,
                sel!("addToRunLoop:forMode:"),
                fn(Id, Sel, Id, Id) -> (),
                run_loop,
                mode
            )
        }
    }

    /// Convenience: add to the current run loop's common modes.
    pub fn add_to_current_run_loop(&self) {
        unsafe {
            let rl: Id = msg_send!(
                cls!("NSRunLoop") as Id,
                sel!("currentRunLoop"),
                fn(Id, Sel) -> Id
            );
            let mode = nsstring("kCFRunLoopCommonModes");
            self.add_to_run_loop(rl, mode);
        }
    }

    /// Remove this display link from all run loops and release resources.
    pub fn invalidate(&self) {
        unsafe {
            msg_send!(self.0, sel!("invalidate"), fn(Id, Sel) -> ())
        }
    }

    /// The time interval of the most recent frame.
    pub fn timestamp(&self) -> f64 {
        unsafe {
            msg_send!(self.0, sel!("timestamp"), fn(Id, Sel) -> f64)
        }
    }

    /// The expected time of the next frame.
    pub fn target_timestamp(&self) -> f64 {
        unsafe {
            msg_send!(self.0, sel!("targetTimestamp"), fn(Id, Sel) -> f64)
        }
    }
}

impl Clone for CADisplayLink {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for CADisplayLink {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
