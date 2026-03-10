//! NSFontPanel + NSFontManager — font selection UI and font management.

use crate::runtime::*;

// ============================================================================
// NSFontPanel
// ============================================================================

pub struct NSFontPanel(pub(super) Id);

impl NSFontPanel {
    pub fn shared() -> Self {
        unsafe {
            let raw: Id = msg_send!(
                cls!("NSFontPanel") as Id,
                sel!("sharedFontPanel"),
                fn(Id, Sel) -> Id
            );
            Self(retain(raw))
        }
    }

    pub fn order_front(&self) {
        unsafe {
            let null: Id = std::ptr::null_mut();
            msg_send!(
                self.0,
                sel!("makeKeyAndOrderFront:"),
                fn(Id, Sel, Id) -> (),
                nil
            );
        }
    }

    pub fn set_is_visible(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setIsVisible:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    pub fn is_visible(&self) -> bool {
        unsafe {
            to_bool(msg_send!(self.0, sel!("isVisible"), fn(Id, Sel) -> BOOL))
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for NSFontPanel {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSFontPanel {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

// ============================================================================
// NSFontManager
// ============================================================================

pub struct NSFontManager(pub(super) Id);

impl NSFontManager {
    pub fn shared() -> Self {
        unsafe {
            let raw: Id = msg_send!(
                cls!("NSFontManager") as Id,
                sel!("sharedFontManager"),
                fn(Id, Sel) -> Id
            );
            Self(retain(raw))
        }
    }

    pub fn selected_font(&self) -> Option<Id> {
        unsafe {
            let raw: Id = msg_send!(self.0, sel!("selectedFont"), fn(Id, Sel) -> Id);
            if raw.is_null() {
                None
            } else {
                Some(raw)
            }
        }
    }

    pub fn set_selected_font(&self, font: Id, multiple: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setSelectedFont:isMultiple:"),
                fn(Id, Sel, Id, BOOL) -> (),
                font, from_bool(multiple)
            );
        }
    }

    pub fn available_fonts(&self) -> Vec<String> {
        unsafe {
            let array: Id = msg_send!(self.0, sel!("availableFonts"), fn(Id, Sel) -> Id);
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

    pub fn convert_font(&self, font: Id, to_size: CGFloat) -> Id {
        unsafe {
            msg_send!(
                self.0,
                sel!("convertFont:toSize:"),
                fn(Id, Sel, Id, CGFloat) -> Id,
                font, to_size
            )
        }
    }

    pub fn convert_font_to_have_trait(&self, font: Id, trait_mask: NSUInteger) -> Id {
        unsafe {
            msg_send!(
                self.0,
                sel!("convertFont:toHaveTrait:"),
                fn(Id, Sel, Id, NSUInteger) -> Id,
                font, trait_mask
            )
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for NSFontManager {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSFontManager {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
