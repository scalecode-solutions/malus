//! NSPopUpButton — a pop-up menu / pull-down button.

use crate::runtime::*;

pub struct NSPopUpButton(pub(super) Id);

impl NSPopUpButton {
    pub fn new(frame: CGRect, pulls_down: bool) -> Self {
        unsafe {
            let obj = alloc(cls!("NSPopUpButton") as Id);
            let obj: Id = msg_send!(
                obj,
                sel!("initWithFrame:pullsDown:"),
                fn(Id, Sel, CGRect, BOOL) -> Id,
                frame,
                from_bool(pulls_down)
            );
            Self(obj)
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }

    pub fn add_item(&self, title: &str) {
        unsafe {
            let ns = nsstring(title);
            msg_send!(self.0, sel!("addItemWithTitle:"), fn(Id, Sel, Id) -> (), ns)
        }
    }

    /// Add multiple items by creating a temporary NSArray of NSStrings.
    pub fn add_items(&self, titles: &[&str]) {
        unsafe {
            // Build an NSMutableArray
            let arr: Id = msg_send!(
                cls!("NSMutableArray") as Id,
                sel!("arrayWithCapacity:"),
                fn(Id, Sel, NSUInteger) -> Id,
                titles.len() as NSUInteger
            );
            for &t in titles {
                let ns = nsstring(t);
                msg_send!(arr, sel!("addObject:"), fn(Id, Sel, Id) -> (), ns);
            }
            msg_send!(self.0, sel!("addItemsWithTitles:"), fn(Id, Sel, Id) -> (), arr)
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

    pub fn title_of_selected_item(&self) -> String {
        unsafe {
            let ns: Id = msg_send!(self.0, sel!("titleOfSelectedItem"), fn(Id, Sel) -> Id);
            from_nsstring(ns)
        }
    }

    pub fn number_of_items(&self) -> i64 {
        unsafe {
            msg_send!(self.0, sel!("numberOfItems"), fn(Id, Sel) -> NSInteger) as i64
        }
    }

    pub fn set_target(&self, target: Id) {
        unsafe {
            msg_send!(self.0, sel!("setTarget:"), fn(Id, Sel, Id) -> (), target)
        }
    }

    pub fn set_action(&self, action: Sel) {
        unsafe {
            msg_send!(self.0, sel!("setAction:"), fn(Id, Sel, Sel) -> (), action)
        }
    }
}

impl Clone for NSPopUpButton {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSPopUpButton {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
