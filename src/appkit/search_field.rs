//! NSSearchField — a text field optimized for search.

use crate::runtime::*;

pub struct NSSearchField(pub(super) Id);

impl NSSearchField {
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSSearchField") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }

    pub fn set_placeholder(&self, text: &str) {
        unsafe {
            let ns = nsstring(text);
            msg_send!(self.0, sel!("setPlaceholderString:"), fn(Id, Sel, Id) -> (), ns)
        }
    }

    pub fn placeholder(&self) -> String {
        unsafe {
            let ns: Id = msg_send!(self.0, sel!("placeholderString"), fn(Id, Sel) -> Id);
            from_nsstring(ns)
        }
    }

    pub fn string_value(&self) -> String {
        unsafe {
            let ns: Id = msg_send!(self.0, sel!("stringValue"), fn(Id, Sel) -> Id);
            from_nsstring(ns)
        }
    }

    pub fn set_string_value(&self, val: &str) {
        unsafe {
            let ns = nsstring(val);
            msg_send!(self.0, sel!("setStringValue:"), fn(Id, Sel, Id) -> (), ns)
        }
    }

    pub fn set_delegate(&self, delegate: Id) {
        unsafe {
            msg_send!(self.0, sel!("setDelegate:"), fn(Id, Sel, Id) -> (), delegate)
        }
    }

    /// Returns the recent searches array as a raw NSArray Id.
    pub fn recent_searches(&self) -> Id {
        unsafe { msg_send!(self.0, sel!("recentSearches"), fn(Id, Sel) -> Id) }
    }

    pub fn set_sends_whole_search_string(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setSendsWholeSearchString:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            )
        }
    }

    pub fn set_sends_search_string_immediately(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setSendsSearchStringImmediately:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            )
        }
    }
}

impl Clone for NSSearchField {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSSearchField {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
