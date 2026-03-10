//! NSSlider — a horizontal or vertical slider control.

use crate::runtime::*;

pub struct NSSlider(pub(super) Id);

impl NSSlider {
    pub fn new(frame: CGRect) -> Self {
        unsafe {
            let obj = alloc(cls!("NSSlider") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithFrame:"), fn(Id, Sel, CGRect) -> Id, frame);
            Self(obj)
        }
    }

    /// Create a slider with an initial value, min, and max.
    pub fn with_value(value: f64, min: f64, max: f64) -> Self {
        unsafe {
            let null: Id = std::ptr::null_mut();
            let raw: Id = msg_send!(
                cls!("NSSlider") as Id,
                sel!("sliderWithValue:minValue:maxValue:target:action:"),
                fn(Id, Sel, f64, f64, f64, Id, Sel) -> Id,
                value,
                min,
                max,
                nil,
                std::mem::zeroed::<Sel>()
            );
            Self(retain(raw))
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

    pub fn min_value(&self) -> f64 {
        unsafe { msg_send!(self.0, sel!("minValue"), fn(Id, Sel) -> f64) }
    }

    pub fn set_max_value(&self, val: f64) {
        unsafe {
            msg_send!(self.0, sel!("setMaxValue:"), fn(Id, Sel, f64) -> (), val)
        }
    }

    pub fn max_value(&self) -> f64 {
        unsafe { msg_send!(self.0, sel!("maxValue"), fn(Id, Sel) -> f64) }
    }

    pub fn set_number_of_tick_marks(&self, count: i64) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setNumberOfTickMarks:"),
                fn(Id, Sel, NSInteger) -> (),
                count as NSInteger
            )
        }
    }

    pub fn number_of_tick_marks(&self) -> i64 {
        unsafe {
            msg_send!(self.0, sel!("numberOfTickMarks"), fn(Id, Sel) -> NSInteger) as i64
        }
    }

    pub fn set_allows_tick_mark_values_only(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setAllowsTickMarkValuesOnly:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            )
        }
    }

    pub fn set_continuous(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setContinuous:"), fn(Id, Sel, BOOL) -> (), from_bool(flag))
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

    pub fn set_vertical(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setVertical:"), fn(Id, Sel, BOOL) -> (), from_bool(flag))
        }
    }
}

impl Clone for NSSlider {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSSlider {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
