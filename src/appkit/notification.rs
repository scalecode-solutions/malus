//! NSNotificationCenter — intra-process notification dispatch.

use crate::runtime::*;

// ============================================================================
// NSNotificationCenter
// ============================================================================

pub struct NSNotificationCenter(pub(super) Id);

impl NSNotificationCenter {
    pub fn default_center() -> Self {
        unsafe {
            let raw: Id = msg_send!(
                cls!("NSNotificationCenter") as Id,
                sel!("defaultCenter"),
                fn(Id, Sel) -> Id
            );
            Self(retain(raw))
        }
    }

    pub fn post(&self, name: &str, object: Id) {
        unsafe {
            let name_ns = nsstring(name);
            msg_send!(
                self.0,
                sel!("postNotificationName:object:"),
                fn(Id, Sel, Id, Id) -> (),
                name_ns, object
            );
        }
    }

    pub fn post_with_info(&self, name: &str, object: Id, user_info: Id) {
        unsafe {
            let name_ns = nsstring(name);
            msg_send!(
                self.0,
                sel!("postNotificationName:object:userInfo:"),
                fn(Id, Sel, Id, Id, Id) -> (),
                name_ns, object, user_info
            );
        }
    }

    pub fn add_observer(&self, observer: Id, selector: Sel, name: &str, object: Id) {
        unsafe {
            let name_ns = nsstring(name);
            msg_send!(
                self.0,
                sel!("addObserver:selector:name:object:"),
                fn(Id, Sel, Id, Sel, Id, Id) -> (),
                observer, selector, name_ns, object
            );
        }
    }

    pub fn remove_observer(&self, observer: Id) {
        unsafe {
            msg_send!(
                self.0,
                sel!("removeObserver:"),
                fn(Id, Sel, Id) -> (),
                observer
            );
        }
    }

    pub fn remove_observer_for_name(&self, observer: Id, name: &str, object: Id) {
        unsafe {
            let name_ns = nsstring(name);
            msg_send!(
                self.0,
                sel!("removeObserver:name:object:"),
                fn(Id, Sel, Id, Id, Id) -> (),
                observer, name_ns, object
            );
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for NSNotificationCenter {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSNotificationCenter {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
