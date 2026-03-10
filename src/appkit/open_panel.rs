//! NSOpenPanel — file open dialog.

use crate::runtime::*;

// ============================================================================
// NSOpenPanel
// ============================================================================

pub struct NSOpenPanel(pub(super) Id);

impl NSOpenPanel {
    pub fn new() -> Self {
        unsafe {
            let raw: Id = msg_send!(
                cls!("NSOpenPanel") as Id,
                sel!("openPanel"),
                fn(Id, Sel) -> Id
            );
            Self(retain(raw))
        }
    }

    pub fn set_can_choose_files(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setCanChooseFiles:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    pub fn can_choose_files(&self) -> bool {
        unsafe {
            to_bool(msg_send!(self.0, sel!("canChooseFiles"), fn(Id, Sel) -> BOOL))
        }
    }

    pub fn set_can_choose_directories(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setCanChooseDirectories:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    pub fn set_allows_multiple_selection(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setAllowsMultipleSelection:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    pub fn set_resolves_aliases(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setResolvesAliases:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
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

    pub fn urls(&self) -> Vec<String> {
        unsafe {
            let array: Id = msg_send!(self.0, sel!("URLs"), fn(Id, Sel) -> Id);
            let count: NSUInteger = msg_send!(array, sel!("count"), fn(Id, Sel) -> NSUInteger);
            let mut result = Vec::with_capacity(count);
            for i in 0..count {
                let url: Id = msg_send!(
                    array,
                    sel!("objectAtIndex:"),
                    fn(Id, Sel, NSUInteger) -> Id,
                    i
                );
                let abs: Id = msg_send!(url, sel!("absoluteString"), fn(Id, Sel) -> Id);
                result.push(from_nsstring(abs));
            }
            result
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

impl Clone for NSOpenPanel {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSOpenPanel {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
