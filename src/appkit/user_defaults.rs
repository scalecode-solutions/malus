//! NSUserDefaults — persistent storage of user preferences.

use crate::runtime::*;

// ============================================================================
// NSUserDefaults
// ============================================================================

pub struct NSUserDefaults(pub(super) Id);

impl NSUserDefaults {
    pub fn standard() -> Self {
        unsafe {
            let raw: Id = msg_send!(
                cls!("NSUserDefaults") as Id,
                sel!("standardUserDefaults"),
                fn(Id, Sel) -> Id
            );
            Self(retain(raw))
        }
    }

    pub fn string_for_key(&self, key: &str) -> Option<String> {
        unsafe {
            let key_ns = nsstring(key);
            let raw: Id = msg_send!(
                self.0,
                sel!("stringForKey:"),
                fn(Id, Sel, Id) -> Id,
                key_ns
            );
            if raw.is_null() {
                None
            } else {
                Some(from_nsstring(raw))
            }
        }
    }

    pub fn set_string(&self, val: &str, key: &str) {
        unsafe {
            let val_ns = nsstring(val);
            let key_ns = nsstring(key);
            msg_send!(
                self.0,
                sel!("setObject:forKey:"),
                fn(Id, Sel, Id, Id) -> (),
                val_ns, key_ns
            );
        }
    }

    pub fn integer_for_key(&self, key: &str) -> i64 {
        unsafe {
            let key_ns = nsstring(key);
            msg_send!(
                self.0,
                sel!("integerForKey:"),
                fn(Id, Sel, Id) -> i64,
                key_ns
            )
        }
    }

    pub fn set_integer(&self, val: i64, key: &str) {
        unsafe {
            let key_ns = nsstring(key);
            msg_send!(
                self.0,
                sel!("setInteger:forKey:"),
                fn(Id, Sel, i64, Id) -> (),
                val, key_ns
            );
        }
    }

    pub fn bool_for_key(&self, key: &str) -> bool {
        unsafe {
            let key_ns = nsstring(key);
            let result: BOOL = msg_send!(
                self.0,
                sel!("boolForKey:"),
                fn(Id, Sel, Id) -> BOOL,
                key_ns
            );
            to_bool(result)
        }
    }

    pub fn set_bool(&self, val: bool, key: &str) {
        unsafe {
            let key_ns = nsstring(key);
            msg_send!(
                self.0,
                sel!("setBool:forKey:"),
                fn(Id, Sel, BOOL, Id) -> (),
                from_bool(val), key_ns
            );
        }
    }

    pub fn double_for_key(&self, key: &str) -> f64 {
        unsafe {
            let key_ns = nsstring(key);
            msg_send!(
                self.0,
                sel!("doubleForKey:"),
                fn(Id, Sel, Id) -> f64,
                key_ns
            )
        }
    }

    pub fn set_double(&self, val: f64, key: &str) {
        unsafe {
            let key_ns = nsstring(key);
            msg_send!(
                self.0,
                sel!("setDouble:forKey:"),
                fn(Id, Sel, f64, Id) -> (),
                val, key_ns
            );
        }
    }

    pub fn remove_object(&self, key: &str) {
        unsafe {
            let key_ns = nsstring(key);
            msg_send!(
                self.0,
                sel!("removeObjectForKey:"),
                fn(Id, Sel, Id) -> (),
                key_ns
            );
        }
    }

    pub fn synchronize(&self) -> bool {
        unsafe {
            to_bool(msg_send!(self.0, sel!("synchronize"), fn(Id, Sel) -> BOOL))
        }
    }

    pub fn object_for_key(&self, key: &str) -> Id {
        unsafe {
            let key_ns = nsstring(key);
            msg_send!(
                self.0,
                sel!("objectForKey:"),
                fn(Id, Sel, Id) -> Id,
                key_ns
            )
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for NSUserDefaults {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSUserDefaults {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
