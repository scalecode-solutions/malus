//! NSSavePanel — file save dialog.

use crate::runtime::*;

// ============================================================================
// NSSavePanel
// ============================================================================

pub struct NSSavePanel(pub(super) Id);

impl NSSavePanel {
    pub fn new() -> Self {
        unsafe {
            let raw: Id = msg_send!(
                cls!("NSSavePanel") as Id,
                sel!("savePanel"),
                fn(Id, Sel) -> Id
            );
            Self(retain(raw))
        }
    }

    pub fn set_title(&self, title: &str) {
        unsafe {
            let title_ns = nsstring(title);
            msg_send!(self.0, sel!("setTitle:"), fn(Id, Sel, Id) -> (), title_ns);
        }
    }

    pub fn set_message(&self, msg: &str) {
        unsafe {
            let msg_ns = nsstring(msg);
            msg_send!(self.0, sel!("setMessage:"), fn(Id, Sel, Id) -> (), msg_ns);
        }
    }

    pub fn set_prompt(&self, prompt: &str) {
        unsafe {
            let prompt_ns = nsstring(prompt);
            msg_send!(self.0, sel!("setPrompt:"), fn(Id, Sel, Id) -> (), prompt_ns);
        }
    }

    pub fn set_name_field_string_value(&self, name: &str) {
        unsafe {
            let name_ns = nsstring(name);
            msg_send!(
                self.0,
                sel!("setNameFieldStringValue:"),
                fn(Id, Sel, Id) -> (),
                name_ns
            );
        }
    }

    pub fn set_can_create_directories(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setCanCreateDirectories:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    pub fn set_allowed_content_types(&self, types: Id) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setAllowedContentTypes:"),
                fn(Id, Sel, Id) -> (),
                types
            );
        }
    }

    pub fn set_directory_url(&self, url: Id) {
        unsafe {
            msg_send!(self.0, sel!("setDirectoryURL:"), fn(Id, Sel, Id) -> (), url);
        }
    }

    pub fn run_modal(&self) -> i64 {
        unsafe {
            msg_send!(self.0, sel!("runModal"), fn(Id, Sel) -> i64)
        }
    }

    pub fn url(&self) -> Option<String> {
        unsafe {
            let url: Id = msg_send!(self.0, sel!("URL"), fn(Id, Sel) -> Id);
            if url.is_null() {
                None
            } else {
                let abs: Id = msg_send!(url, sel!("absoluteString"), fn(Id, Sel) -> Id);
                Some(from_nsstring(abs))
            }
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for NSSavePanel {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSSavePanel {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
