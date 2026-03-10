//! NSPasteboard — clipboard / drag-and-drop data transfer.

use crate::runtime::*;

pub const PASTEBOARD_TYPE_STRING: &str = "public.utf8-plain-text";

// ============================================================================
// NSPasteboard
// ============================================================================

pub struct NSPasteboard(pub(super) Id);

impl NSPasteboard {
    pub fn general() -> Self {
        unsafe {
            let raw: Id = msg_send!(
                cls!("NSPasteboard") as Id,
                sel!("generalPasteboard"),
                fn(Id, Sel) -> Id
            );
            Self(retain(raw))
        }
    }

    pub fn clear_contents(&self) -> i64 {
        unsafe {
            msg_send!(self.0, sel!("clearContents"), fn(Id, Sel) -> i64)
        }
    }

    pub fn set_string(&self, string: &str, for_type: &str) -> bool {
        unsafe {
            let string_ns = nsstring(string);
            let type_ns = nsstring(for_type);
            let result: BOOL = msg_send!(
                self.0,
                sel!("setString:forType:"),
                fn(Id, Sel, Id, Id) -> BOOL,
                string_ns, type_ns
            );
            to_bool(result)
        }
    }

    pub fn string_for_type(&self, paste_type: &str) -> Option<String> {
        unsafe {
            let type_ns = nsstring(paste_type);
            let raw: Id = msg_send!(
                self.0,
                sel!("stringForType:"),
                fn(Id, Sel, Id) -> Id,
                type_ns
            );
            if raw.is_null() {
                None
            } else {
                Some(from_nsstring(raw))
            }
        }
    }

    pub fn types(&self) -> Vec<String> {
        unsafe {
            let array: Id = msg_send!(self.0, sel!("types"), fn(Id, Sel) -> Id);
            if array.is_null() {
                return Vec::new();
            }
            let count: NSUInteger = msg_send!(array, sel!("count"), fn(Id, Sel) -> NSUInteger);
            let mut result = Vec::with_capacity(count);
            for i in 0..count {
                let ns_str: Id = msg_send!(
                    array,
                    sel!("objectAtIndex:"),
                    fn(Id, Sel, NSUInteger) -> Id,
                    i
                );
                result.push(from_nsstring(ns_str));
            }
            result
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for NSPasteboard {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSPasteboard {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
