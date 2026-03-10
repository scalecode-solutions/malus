//! NSControl — base class for most AppKit controls.

use crate::runtime::*;

/// A base class for controls — buttons, text fields, sliders, etc.
pub struct NSControl(pub(super) Id);

impl NSControl {
    pub fn as_raw(&self) -> Id {
        self.0
    }

    pub fn set_enabled(&self, enabled: bool) {
        unsafe {
            msg_send!(self.0, sel!("setEnabled:"), fn(Id, Sel, BOOL) -> (), from_bool(enabled))
        }
    }

    pub fn is_enabled(&self) -> bool {
        unsafe { to_bool(msg_send!(self.0, sel!("isEnabled"), fn(Id, Sel) -> BOOL)) }
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

    pub fn set_string_value(&self, val: &str) {
        unsafe {
            let ns = nsstring(val);
            msg_send!(self.0, sel!("setStringValue:"), fn(Id, Sel, Id) -> (), ns)
        }
    }

    pub fn string_value(&self) -> String {
        unsafe {
            let ns: Id = msg_send!(self.0, sel!("stringValue"), fn(Id, Sel) -> Id);
            from_nsstring(ns)
        }
    }

    pub fn set_int_value(&self, val: i32) {
        unsafe {
            msg_send!(self.0, sel!("setIntValue:"), fn(Id, Sel, i32) -> (), val)
        }
    }

    pub fn int_value(&self) -> i32 {
        unsafe { msg_send!(self.0, sel!("intValue"), fn(Id, Sel) -> i32) }
    }

    pub fn set_double_value(&self, val: f64) {
        unsafe {
            msg_send!(self.0, sel!("setDoubleValue:"), fn(Id, Sel, f64) -> (), val)
        }
    }

    pub fn double_value(&self) -> f64 {
        unsafe { msg_send!(self.0, sel!("doubleValue"), fn(Id, Sel) -> f64) }
    }

    pub fn set_font(&self, font: Id) {
        unsafe {
            msg_send!(self.0, sel!("setFont:"), fn(Id, Sel, Id) -> (), font)
        }
    }

    pub fn size_to_fit(&self) {
        unsafe {
            msg_send!(self.0, sel!("sizeToFit"), fn(Id, Sel) -> ())
        }
    }
}

impl Clone for NSControl {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSControl {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
