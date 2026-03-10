//! NSWorkspace — interface to the shared workspace.

use crate::runtime::*;

// ============================================================================
// NSWorkspace
// ============================================================================

pub struct NSWorkspace(pub(super) Id);

impl NSWorkspace {
    pub fn shared() -> Self {
        unsafe {
            let raw: Id = msg_send!(
                cls!("NSWorkspace") as Id,
                sel!("sharedWorkspace"),
                fn(Id, Sel) -> Id
            );
            Self(retain(raw))
        }
    }

    pub fn open_url(&self, url_string: &str) -> bool {
        unsafe {
            let url_cls = cls!("NSURL") as Id;
            let url_ns = nsstring(url_string);
            let url: Id = msg_send!(
                url_cls,
                sel!("URLWithString:"),
                fn(Id, Sel, Id) -> Id,
                url_ns
            );
            if url.is_null() {
                return false;
            }
            let result: BOOL = msg_send!(
                self.0,
                sel!("openURL:"),
                fn(Id, Sel, Id) -> BOOL,
                url
            );
            to_bool(result)
        }
    }

    pub fn open_file(&self, path: &str) -> bool {
        unsafe {
            let path_ns = nsstring(path);
            let result: BOOL = msg_send!(
                self.0,
                sel!("openFile:"),
                fn(Id, Sel, Id) -> BOOL,
                path_ns
            );
            to_bool(result)
        }
    }

    pub fn open_application(&self, bundle_id: &str) -> bool {
        unsafe {
            let bundle_ns = nsstring(bundle_id);
            let result: BOOL = msg_send!(
                self.0,
                sel!("launchApplication:"),
                fn(Id, Sel, Id) -> BOOL,
                bundle_ns
            );
            to_bool(result)
        }
    }

    pub fn running_applications(&self) -> Vec<Id> {
        unsafe {
            let array: Id = msg_send!(
                self.0,
                sel!("runningApplications"),
                fn(Id, Sel) -> Id
            );
            let count: NSUInteger = msg_send!(array, sel!("count"), fn(Id, Sel) -> NSUInteger);
            let mut result = Vec::with_capacity(count);
            for i in 0..count {
                let app: Id = msg_send!(
                    array,
                    sel!("objectAtIndex:"),
                    fn(Id, Sel, NSUInteger) -> Id,
                    i
                );
                result.push(app);
            }
            result
        }
    }

    pub fn frontmost_application(&self) -> Option<Id> {
        unsafe {
            let raw: Id = msg_send!(
                self.0,
                sel!("frontmostApplication"),
                fn(Id, Sel) -> Id
            );
            if raw.is_null() {
                None
            } else {
                Some(raw)
            }
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for NSWorkspace {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSWorkspace {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
