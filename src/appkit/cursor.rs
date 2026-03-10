//! NSCursor — cursor management.

use crate::runtime::*;

/// A cursor.
pub struct NSCursor(pub(super) Id);

impl NSCursor {
    /// Return the raw ObjC pointer (does not transfer ownership).
    pub fn as_raw(&self) -> Id {
        self.0
    }

    // -- Singleton cursors (class methods, must retain) -----------------------

    fn class_cursor(sel_name: &str) -> Self {
        unsafe {
            let sel = sel_registerName(
                std::ffi::CString::new(sel_name).unwrap().as_ptr(),
            );
            let raw: Id = msg_send!(cls!("NSCursor") as Id, sel, fn(Id, Sel) -> Id);
            Self(retain(raw))
        }
    }

    /// The arrow cursor.
    pub fn arrow() -> Self {
        Self::class_cursor("arrowCursor")
    }

    /// The I-beam cursor (text selection).
    pub fn ibeam() -> Self {
        Self::class_cursor("IBeamCursor")
    }

    /// The crosshair cursor.
    pub fn crosshair() -> Self {
        Self::class_cursor("crosshairCursor")
    }

    /// The pointing hand cursor.
    pub fn pointing_hand() -> Self {
        Self::class_cursor("pointingHandCursor")
    }

    /// The open hand cursor.
    pub fn open_hand() -> Self {
        Self::class_cursor("openHandCursor")
    }

    /// The closed hand cursor.
    pub fn closed_hand() -> Self {
        Self::class_cursor("closedHandCursor")
    }

    /// The horizontal resize cursor.
    pub fn resize_left_right() -> Self {
        Self::class_cursor("resizeLeftRightCursor")
    }

    /// The vertical resize cursor.
    pub fn resize_up_down() -> Self {
        Self::class_cursor("resizeUpDownCursor")
    }

    // -- Instance methods ----------------------------------------------------

    /// Push this cursor onto the cursor stack.
    pub fn push(&self) {
        unsafe {
            msg_send!(self.0, sel!("push"), fn(Id, Sel) -> ());
        }
    }

    /// Make this cursor the current cursor.
    pub fn set(&self) {
        unsafe {
            msg_send!(self.0, sel!("set"), fn(Id, Sel) -> ());
        }
    }

    // -- Class methods -------------------------------------------------------

    /// Pop the top cursor off the cursor stack.
    pub fn pop() {
        unsafe {
            msg_send!(
                cls!("NSCursor") as Id,
                sel!("pop"),
                fn(Id, Sel) -> ()
            );
        }
    }

    /// Hide the cursor.
    pub fn hide() {
        unsafe {
            msg_send!(
                cls!("NSCursor") as Id,
                sel!("hide"),
                fn(Id, Sel) -> ()
            );
        }
    }

    /// Unhide the cursor.
    pub fn unhide() {
        unsafe {
            msg_send!(
                cls!("NSCursor") as Id,
                sel!("unhide"),
                fn(Id, Sel) -> ()
            );
        }
    }
}

impl Clone for NSCursor {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSCursor {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
