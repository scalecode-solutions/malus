//! CFBoolean — CoreFoundation boolean singleton values.

use super::base::*;
use std::fmt;

pub type CFBooleanRef = *const __CFBoolean;

extern "C" {
    static kCFBooleanTrue: CFBooleanRef;
    static kCFBooleanFalse: CFBooleanRef;
    fn CFBooleanGetValue(boolean: CFBooleanRef) -> Boolean;
}

// ============================================================================
// CFBoolean newtype
// ============================================================================

/// Wrapper around kCFBooleanTrue / kCFBooleanFalse.
///
/// Unlike most CF types, booleans are singletons — they are never deallocated.
/// Clone is cheap (just a retain on an immortal object).
pub struct CFBoolean(CFRef<__CFBoolean>);

impl CFBoolean {
    pub fn value(&self) -> bool {
        unsafe { CFBooleanGetValue(self.as_raw()) != 0 }
    }

    pub fn from_bool(v: bool) -> Self {
        unsafe {
            let ptr = if v { kCFBooleanTrue } else { kCFBooleanFalse };
            // Retain because CFRef will release on drop.
            Self(CFRef::retain(ptr).unwrap())
        }
    }

    #[inline]
    pub fn as_raw(&self) -> CFBooleanRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CFBoolean {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CFBoolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CFBoolean({})", self.value())
    }
}

impl From<bool> for CFBoolean {
    fn from(v: bool) -> Self {
        Self::from_bool(v)
    }
}

impl From<CFBoolean> for bool {
    fn from(b: CFBoolean) -> bool {
        b.value()
    }
}
