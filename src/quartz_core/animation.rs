//! CAAnimation, CABasicAnimation, CAKeyframeAnimation — property animations.

use crate::runtime::*;

// ============================================================================
// CAAnimation (abstract base)
// ============================================================================

/// Abstract superclass for Core Animation animations.
pub struct CAAnimation(Id);

impl CAAnimation {
    /// Wrap an existing retained pointer.
    ///
    /// # Safety
    /// `ptr` must be a valid, retained `CAAnimation` (or subclass) instance.
    pub unsafe fn from_raw(ptr: Id) -> Self {
        Self(ptr)
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }

    pub fn set_duration(&self, dur: f64) {
        unsafe {
            msg_send!(self.0, sel!("setDuration:"), fn(Id, Sel, f64) -> (), dur)
        }
    }

    pub fn duration(&self) -> f64 {
        unsafe {
            msg_send!(self.0, sel!("duration"), fn(Id, Sel) -> f64)
        }
    }

    pub fn set_repeats_forever(&self, repeats: bool) {
        let count: f32 = if repeats { f32::MAX } else { 0.0 };
        unsafe {
            msg_send!(self.0, sel!("setRepeatCount:"), fn(Id, Sel, f32) -> (), count)
        }
    }

    pub fn set_removes_on_completion(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setRemovedOnCompletion:"), fn(Id, Sel, BOOL) -> (), from_bool(flag))
        }
    }
}

impl Clone for CAAnimation {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for CAAnimation {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

// ============================================================================
// CABasicAnimation
// ============================================================================

/// An animation that interpolates between a from-value and a to-value.
pub struct CABasicAnimation(Id);

impl CABasicAnimation {
    /// Create a new basic animation for the given key path (e.g. `"opacity"`, `"position"`).
    pub fn new(key_path: &str) -> Self {
        unsafe {
            let ns = nsstring(key_path);
            let obj: Id = msg_send!(
                cls!("CABasicAnimation") as Id,
                sel!("animationWithKeyPath:"),
                fn(Id, Sel, Id) -> Id,
                ns
            );
            Self(retain(obj))
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }

    /// Set the starting value (pass an `NSValue` / `NSNumber`).
    pub fn set_from_value(&self, val: Id) {
        unsafe {
            msg_send!(self.0, sel!("setFromValue:"), fn(Id, Sel, Id) -> (), val)
        }
    }

    /// Set the ending value.
    pub fn set_to_value(&self, val: Id) {
        unsafe {
            msg_send!(self.0, sel!("setToValue:"), fn(Id, Sel, Id) -> (), val)
        }
    }

    pub fn set_duration(&self, dur: f64) {
        unsafe {
            msg_send!(self.0, sel!("setDuration:"), fn(Id, Sel, f64) -> (), dur)
        }
    }

    pub fn duration(&self) -> f64 {
        unsafe {
            msg_send!(self.0, sel!("duration"), fn(Id, Sel) -> f64)
        }
    }
}

impl Clone for CABasicAnimation {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for CABasicAnimation {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

// ============================================================================
// CAKeyframeAnimation
// ============================================================================

/// An animation that progresses through a sequence of values.
pub struct CAKeyframeAnimation(Id);

impl CAKeyframeAnimation {
    /// Create a keyframe animation for the given key path.
    pub fn new(key_path: &str) -> Self {
        unsafe {
            let ns = nsstring(key_path);
            let obj: Id = msg_send!(
                cls!("CAKeyframeAnimation") as Id,
                sel!("animationWithKeyPath:"),
                fn(Id, Sel, Id) -> Id,
                ns
            );
            Self(retain(obj))
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }

    /// Set the array of keyframe values (pass an `NSArray`).
    pub fn set_values(&self, values: Id) {
        unsafe {
            msg_send!(self.0, sel!("setValues:"), fn(Id, Sel, Id) -> (), values)
        }
    }

    /// Set the array of key times (pass an `NSArray` of `NSNumber` in 0.0..1.0).
    pub fn set_key_times(&self, times: Id) {
        unsafe {
            msg_send!(self.0, sel!("setKeyTimes:"), fn(Id, Sel, Id) -> (), times)
        }
    }

    pub fn set_duration(&self, dur: f64) {
        unsafe {
            msg_send!(self.0, sel!("setDuration:"), fn(Id, Sel, f64) -> (), dur)
        }
    }

    pub fn duration(&self) -> f64 {
        unsafe {
            msg_send!(self.0, sel!("duration"), fn(Id, Sel) -> f64)
        }
    }
}

impl Clone for CAKeyframeAnimation {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for CAKeyframeAnimation {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
