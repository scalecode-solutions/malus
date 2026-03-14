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

    /// Convenience: add to the main run loop's common modes.
    ///
    /// This is the correct approach for macOS when the display link is
    /// installed before entering the `NSApplication` run loop.
    pub fn add_to_main_run_loop(&self) {
        unsafe {
            let rl: Id = msg_send!(
                cls!("NSRunLoop") as Id,
                sel!("mainRunLoop"),
                fn(Id, Sel) -> Id
            );
            // On macOS, NSRunLoopCommonModes = "kCFRunLoopCommonModes"
            // But we need the actual CF constant, not a string with that name.
            // Use extern to get the real symbol.
            extern "C" {
                static kCFRunLoopCommonModes: Id;
            }
            let mode = kCFRunLoopCommonModes;
            self.add_to_run_loop(rl, mode);
        }
    }

    /// Create a display link associated with a specific NSView's display.
    ///
    /// On macOS 14+, this is the preferred way to create a CADisplayLink —
    /// it associates the link with the view's screen refresh rate.
    ///
    /// # Safety
    /// `view` must be a valid NSView. `target` must respond to `selector`.
    pub unsafe fn from_view(view: Id, target: Id, selector: Sel) -> Self {
        let obj: Id = msg_send!(
            view,
            sel!("displayLinkWithTarget:selector:"),
            fn(Id, Sel, Id, Sel) -> Id,
            target,
            selector
        );
        Self(retain(obj))
    }

    /// Create a display link associated with a specific NSWindow's display.
    ///
    /// # Safety
    /// `window` must be a valid NSWindow. `target` must respond to `selector`.
    pub unsafe fn from_window(window: Id, target: Id, selector: Sel) -> Self {
        let obj: Id = msg_send!(
            window,
            sel!("displayLinkWithTarget:selector:"),
            fn(Id, Sel, Id, Sel) -> Id,
            target,
            selector
        );
        Self(retain(obj))
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
