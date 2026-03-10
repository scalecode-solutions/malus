//! NSStepper — an up/down arrow control for incrementing/decrementing a value.

use crate::runtime::*;

pub struct NSStepper(pub(super) Id);

impl NSStepper {
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSStepper") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }

    pub fn set_double_value(&self, val: f64) {
        unsafe {
            msg_send!(self.0, sel!("setDoubleValue:"), fn(Id, Sel, f64) -> (), val)
        }
    }

    pub fn double_value(&self) -> f64 {
        unsafe { msg_send!(self.0, sel!("doubleValue"), fn(Id, Sel) -> f64) }
    }

    pub fn set_min_value(&self, val: f64) {
        unsafe {
            msg_send!(self.0, sel!("setMinValue:"), fn(Id, Sel, f64) -> (), val)
        }
    }

    pub fn set_max_value(&self, val: f64) {
        unsafe {
            msg_send!(self.0, sel!("setMaxValue:"), fn(Id, Sel, f64) -> (), val)
        }
    }

    pub fn set_increment(&self, val: f64) {
        unsafe {
            msg_send!(self.0, sel!("setIncrement:"), fn(Id, Sel, f64) -> (), val)
        }
    }

    pub fn increment(&self) -> f64 {
        unsafe { msg_send!(self.0, sel!("increment"), fn(Id, Sel) -> f64) }
    }

    pub fn set_autorepeat(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setAutorepeat:"), fn(Id, Sel, BOOL) -> (), from_bool(flag))
        }
    }

    pub fn set_value_wraps(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setValueWraps:"), fn(Id, Sel, BOOL) -> (), from_bool(flag))
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

impl Clone for NSStepper {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSStepper {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
