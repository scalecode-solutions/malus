//! NSAlert — modal alert dialogs.

use crate::runtime::*;

// ============================================================================
// Types
// ============================================================================

pub type NSModalResponse = i64;

pub const ALERT_FIRST_BUTTON_RETURN: NSModalResponse = 1000;
pub const ALERT_SECOND_BUTTON_RETURN: NSModalResponse = 1001;
pub const ALERT_THIRD_BUTTON_RETURN: NSModalResponse = 1002;

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NSAlertStyle {
    Warning = 0,
    Informational = 1,
    Critical = 2,
}

// ============================================================================
// NSAlert
// ============================================================================

pub struct NSAlert(pub(super) Id);

impl NSAlert {
    pub fn new() -> Self {
        unsafe {
            Self(alloc_init(cls!("NSAlert") as Id))
        }
    }

    pub fn set_message_text(&self, text: &str) {
        unsafe {
            let text_ns = nsstring(text);
            msg_send!(self.0, sel!("setMessageText:"), fn(Id, Sel, Id) -> (), text_ns);
        }
    }

    pub fn set_informative_text(&self, text: &str) {
        unsafe {
            let text_ns = nsstring(text);
            msg_send!(self.0, sel!("setInformativeText:"), fn(Id, Sel, Id) -> (), text_ns);
        }
    }

    pub fn set_alert_style(&self, style: NSAlertStyle) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setAlertStyle:"),
                fn(Id, Sel, u64) -> (),
                style as u64
            );
        }
    }

    pub fn add_button(&self, title: &str) {
        unsafe {
            let title_ns = nsstring(title);
            msg_send!(self.0, sel!("addButtonWithTitle:"), fn(Id, Sel, Id) -> (), title_ns);
        }
    }

    pub fn run_modal(&self) -> NSModalResponse {
        unsafe {
            msg_send!(self.0, sel!("runModal"), fn(Id, Sel) -> NSModalResponse)
        }
    }

    pub fn set_icon(&self, image: Id) {
        unsafe {
            msg_send!(self.0, sel!("setIcon:"), fn(Id, Sel, Id) -> (), image);
        }
    }

    pub fn set_shows_suppression_button(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setShowsSuppressionButton:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    pub fn suppression_button(&self) -> Id {
        unsafe {
            msg_send!(self.0, sel!("suppressionButton"), fn(Id, Sel) -> Id)
        }
    }

    pub fn begin_sheet_modal(&self, window: Id, completion: Id) {
        unsafe {
            msg_send!(
                self.0,
                sel!("beginSheetModalForWindow:completionHandler:"),
                fn(Id, Sel, Id, Id) -> (),
                window, completion
            );
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for NSAlert {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSAlert {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
