//! NSMenu + NSMenuItem — application and contextual menus.

use crate::runtime::*;

// ============================================================================
// NSMenu
// ============================================================================

pub struct NSMenu(pub(super) Id);

impl NSMenu {
    pub fn new(title: &str) -> Self {
        unsafe {
            let obj = alloc(cls!("NSMenu") as Id);
            let title_ns = nsstring(title);
            let raw: Id = msg_send!(obj, sel!("initWithTitle:"), fn(Id, Sel, Id) -> Id, title_ns);
            Self(raw)
        }
    }

    pub fn add_item(&self, item: &NSMenuItem) {
        unsafe {
            msg_send!(self.0, sel!("addItem:"), fn(Id, Sel, Id) -> (), item.0);
        }
    }

    pub fn add_item_with_title(&self, title: &str, action: Sel, key_equivalent: &str) -> NSMenuItem {
        unsafe {
            let title_ns = nsstring(title);
            let key_ns = nsstring(key_equivalent);
            let raw: Id = msg_send!(
                self.0,
                sel!("addItemWithTitle:action:keyEquivalent:"),
                fn(Id, Sel, Id, Sel, Id) -> Id,
                title_ns, action, key_ns
            );
            NSMenuItem(retain(raw))
        }
    }

    pub fn insert_item(&self, item: &NSMenuItem, at_index: i64) {
        unsafe {
            msg_send!(
                self.0,
                sel!("insertItem:atIndex:"),
                fn(Id, Sel, Id, i64) -> (),
                item.0, at_index
            );
        }
    }

    pub fn remove_item(&self, item: &NSMenuItem) {
        unsafe {
            msg_send!(self.0, sel!("removeItem:"), fn(Id, Sel, Id) -> (), item.0);
        }
    }

    pub fn remove_all_items(&self) {
        unsafe {
            msg_send!(self.0, sel!("removeAllItems"), fn(Id, Sel) -> ());
        }
    }

    pub fn number_of_items(&self) -> i64 {
        unsafe {
            msg_send!(self.0, sel!("numberOfItems"), fn(Id, Sel) -> i64)
        }
    }

    pub fn item_at_index(&self, index: i64) -> Option<NSMenuItem> {
        unsafe {
            let raw: Id = msg_send!(
                self.0,
                sel!("itemAtIndex:"),
                fn(Id, Sel, i64) -> Id,
                index
            );
            if raw.is_null() {
                None
            } else {
                Some(NSMenuItem(retain(raw)))
            }
        }
    }

    pub fn set_autoenables_items(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setAutoenablesItems:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    pub fn add_separator(&self) {
        unsafe {
            let sep: Id = msg_send!(
                cls!("NSMenuItem") as Id,
                sel!("separatorItem"),
                fn(Id, Sel) -> Id
            );
            msg_send!(self.0, sel!("addItem:"), fn(Id, Sel, Id) -> (), sep);
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for NSMenu {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSMenu {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

// ============================================================================
// NSMenuItem
// ============================================================================

pub struct NSMenuItem(pub(super) Id);

impl NSMenuItem {
    pub fn new(title: &str, action: Sel, key_equivalent: &str) -> Self {
        unsafe {
            let obj = alloc(cls!("NSMenuItem") as Id);
            let title_ns = nsstring(title);
            let key_ns = nsstring(key_equivalent);
            let raw: Id = msg_send!(
                obj,
                sel!("initWithTitle:action:keyEquivalent:"),
                fn(Id, Sel, Id, Sel, Id) -> Id,
                title_ns, action, key_ns
            );
            Self(raw)
        }
    }

    pub fn separator() -> Self {
        unsafe {
            let raw: Id = msg_send!(
                cls!("NSMenuItem") as Id,
                sel!("separatorItem"),
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

    pub fn title(&self) -> String {
        unsafe {
            let raw: Id = msg_send!(self.0, sel!("title"), fn(Id, Sel) -> Id);
            from_nsstring(raw)
        }
    }

    pub fn set_submenu(&self, menu: &NSMenu) {
        unsafe {
            msg_send!(self.0, sel!("setSubmenu:"), fn(Id, Sel, Id) -> (), menu.0);
        }
    }

    pub fn submenu(&self) -> Option<NSMenu> {
        unsafe {
            let raw: Id = msg_send!(self.0, sel!("submenu"), fn(Id, Sel) -> Id);
            if raw.is_null() {
                None
            } else {
                Some(NSMenu(retain(raw)))
            }
        }
    }

    pub fn set_key_equivalent(&self, key: &str) {
        unsafe {
            let key_ns = nsstring(key);
            msg_send!(self.0, sel!("setKeyEquivalent:"), fn(Id, Sel, Id) -> (), key_ns);
        }
    }

    pub fn set_key_equivalent_modifier_mask(&self, mask: NSUInteger) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setKeyEquivalentModifierMask:"),
                fn(Id, Sel, NSUInteger) -> (),
                mask
            );
        }
    }

    pub fn set_target(&self, target: Id) {
        unsafe {
            msg_send!(self.0, sel!("setTarget:"), fn(Id, Sel, Id) -> (), target);
        }
    }

    pub fn set_action(&self, action: Sel) {
        unsafe {
            msg_send!(self.0, sel!("setAction:"), fn(Id, Sel, Sel) -> (), action);
        }
    }

    pub fn set_enabled(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setEnabled:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe {
            to_bool(msg_send!(self.0, sel!("isEnabled"), fn(Id, Sel) -> BOOL))
        }
    }

    pub fn set_state(&self, state: i64) {
        unsafe {
            msg_send!(self.0, sel!("setState:"), fn(Id, Sel, i64) -> (), state);
        }
    }

    pub fn set_tag(&self, tag: i64) {
        unsafe {
            msg_send!(self.0, sel!("setTag:"), fn(Id, Sel, i64) -> (), tag);
        }
    }

    pub fn tag(&self) -> i64 {
        unsafe {
            msg_send!(self.0, sel!("tag"), fn(Id, Sel) -> i64)
        }
    }

    pub fn set_represented_object(&self, obj: Id) {
        unsafe {
            msg_send!(self.0, sel!("setRepresentedObject:"), fn(Id, Sel, Id) -> (), obj);
        }
    }

    pub fn set_image(&self, image: Id) {
        unsafe {
            msg_send!(self.0, sel!("setImage:"), fn(Id, Sel, Id) -> (), image);
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for NSMenuItem {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSMenuItem {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
