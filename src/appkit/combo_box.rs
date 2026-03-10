//! NSComboBox — a text field with a pop-up list of choices.

use crate::runtime::*;

pub struct NSComboBox(pub(super) Id);

impl NSComboBox {
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSComboBox") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }

    pub fn add_item(&self, value: &str) {
        unsafe {
            let ns = nsstring(value);
            msg_send!(self.0, sel!("addItemWithObjectValue:"), fn(Id, Sel, Id) -> (), ns)
        }
    }

    pub fn remove_all_items(&self) {
        unsafe {
            msg_send!(self.0, sel!("removeAllItems"), fn(Id, Sel) -> ())
        }
    }

    pub fn select_item_at_index(&self, index: i64) {
        unsafe {
            msg_send!(
                self.0,
                sel!("selectItemAtIndex:"),
                fn(Id, Sel, NSInteger) -> (),
                index as NSInteger
            )
        }
    }

    pub fn index_of_selected_item(&self) -> i64 {
        unsafe {
            msg_send!(self.0, sel!("indexOfSelectedItem"), fn(Id, Sel) -> NSInteger) as i64
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

    pub fn number_of_items(&self) -> i64 {
        unsafe {
            msg_send!(self.0, sel!("numberOfItems"), fn(Id, Sel) -> NSInteger) as i64
        }
    }

    pub fn set_completes(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setCompletes:"), fn(Id, Sel, BOOL) -> (), from_bool(flag))
        }
    }

    pub fn set_has_vertical_scroller(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setHasVerticalScroller:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            )
        }
    }

    pub fn set_delegate(&self, delegate: Id) {
        unsafe {
            msg_send!(self.0, sel!("setDelegate:"), fn(Id, Sel, Id) -> (), delegate)
        }
    }
}

impl Clone for NSComboBox {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSComboBox {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
