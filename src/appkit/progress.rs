//! NSProgressIndicator — a bar or spinning progress indicator.

use crate::runtime::*;

// ============================================================================
// NSProgressIndicatorStyle
// ============================================================================

#[repr(usize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum NSProgressIndicatorStyle {
    Bar = 0,
    Spinning = 1,
}

// ============================================================================
// NSProgressIndicator
// ============================================================================

pub struct NSProgressIndicator(pub(super) Id);

impl NSProgressIndicator {
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSProgressIndicator") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }

    pub fn set_style(&self, style: NSProgressIndicatorStyle) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setStyle:"),
                fn(Id, Sel, NSUInteger) -> (),
                style as NSUInteger
            )
        }
    }

    pub fn set_indeterminate(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setIndeterminate:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            )
        }
    }

    pub fn is_indeterminate(&self) -> bool {
        unsafe { to_bool(msg_send!(self.0, sel!("isIndeterminate"), fn(Id, Sel) -> BOOL)) }
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

    pub fn start_animation(&self) {
        unsafe {
            let null: Id = std::ptr::null_mut();
            msg_send!(self.0, sel!("startAnimation:"), fn(Id, Sel, Id) -> (), null)
        }
    }

    pub fn stop_animation(&self) {
        unsafe {
            let null: Id = std::ptr::null_mut();
            msg_send!(self.0, sel!("stopAnimation:"), fn(Id, Sel, Id) -> (), null)
        }
    }

    pub fn increment_by(&self, delta: f64) {
        unsafe {
            msg_send!(self.0, sel!("incrementBy:"), fn(Id, Sel, f64) -> (), delta)
        }
    }
}

impl Clone for NSProgressIndicator {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSProgressIndicator {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
