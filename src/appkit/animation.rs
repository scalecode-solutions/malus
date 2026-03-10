//! NSAnimationContext — implicit and explicit animation grouping.

use crate::runtime::*;

// ============================================================================
// NSAnimationContext
// ============================================================================

pub struct NSAnimationContext(pub(super) Id);

impl NSAnimationContext {
    pub fn begin_grouping() {
        unsafe {
            msg_send!(
                cls!("NSAnimationContext") as Id,
                sel!("beginGrouping"),
                fn(Id, Sel) -> ()
            );
        }
    }

    pub fn end_grouping() {
        unsafe {
            msg_send!(
                cls!("NSAnimationContext") as Id,
                sel!("endGrouping"),
                fn(Id, Sel) -> ()
            );
        }
    }

    pub fn current() -> Self {
        unsafe {
            let raw: Id = msg_send!(
                cls!("NSAnimationContext") as Id,
                sel!("currentContext"),
                fn(Id, Sel) -> Id
            );
            Self(retain(raw))
        }
    }

    pub fn set_duration(&self, duration: f64) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setDuration:"),
                fn(Id, Sel, f64) -> (),
                duration
            );
        }
    }

    pub fn duration(&self) -> f64 {
        unsafe {
            msg_send!(self.0, sel!("duration"), fn(Id, Sel) -> f64)
        }
    }

    pub fn set_allows_implicit_animation(&self, flag: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setAllowsImplicitAnimation:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(flag)
            );
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for NSAnimationContext {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSAnimationContext {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
