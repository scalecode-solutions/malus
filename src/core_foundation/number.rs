//! CFNumber — CoreFoundation boxed numeric value.

use super::base::*;
use std::ffi::c_void;
use std::fmt;

pub type CFNumberRef = *const __CFNumber;

/// CFNumberType constants — which C type is stored inside.
#[repr(isize)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CFNumberType {
    SInt8 = 1,
    SInt16 = 2,
    SInt32 = 3,
    SInt64 = 4,
    Float32 = 5,
    Float64 = 6,
    Char = 7,
    Short = 8,
    Int = 9,
    Long = 10,
    LongLong = 11,
    Float = 12,
    Double = 13,
    CFIndex = 14,
    NSInteger = 15,
    CGFloat = 16,
}

extern "C" {
    fn CFNumberCreate(
        allocator: CFAllocatorRef,
        the_type: CFNumberType,
        value_ptr: *const c_void,
    ) -> CFNumberRef;

    fn CFNumberGetValue(
        number: CFNumberRef,
        the_type: CFNumberType,
        value_ptr: *mut c_void,
    ) -> Boolean;

    fn CFNumberGetType(number: CFNumberRef) -> CFNumberType;
}

// ============================================================================
// CFNumber newtype
// ============================================================================

pub struct CFNumber(CFRef<__CFNumber>);

impl CFNumber {
    pub fn from_i32(v: i32) -> Self {
        unsafe {
            let raw = CFNumberCreate(
                std::ptr::null(),
                CFNumberType::SInt32,
                &v as *const i32 as *const c_void,
            );
            Self(CFRef::wrap(raw).expect("CFNumberCreate returned null"))
        }
    }

    pub fn from_i64(v: i64) -> Self {
        unsafe {
            let raw = CFNumberCreate(
                std::ptr::null(),
                CFNumberType::SInt64,
                &v as *const i64 as *const c_void,
            );
            Self(CFRef::wrap(raw).expect("CFNumberCreate returned null"))
        }
    }

    pub fn from_f32(v: f32) -> Self {
        unsafe {
            let raw = CFNumberCreate(
                std::ptr::null(),
                CFNumberType::Float32,
                &v as *const f32 as *const c_void,
            );
            Self(CFRef::wrap(raw).expect("CFNumberCreate returned null"))
        }
    }

    pub fn from_f64(v: f64) -> Self {
        unsafe {
            let raw = CFNumberCreate(
                std::ptr::null(),
                CFNumberType::Float64,
                &v as *const f64 as *const c_void,
            );
            Self(CFRef::wrap(raw).expect("CFNumberCreate returned null"))
        }
    }

    pub fn to_i32(&self) -> Option<i32> {
        let mut v: i32 = 0;
        unsafe {
            if CFNumberGetValue(
                self.as_raw(),
                CFNumberType::SInt32,
                &mut v as *mut i32 as *mut c_void,
            ) != 0
            {
                Some(v)
            } else {
                None
            }
        }
    }

    pub fn to_i64(&self) -> Option<i64> {
        let mut v: i64 = 0;
        unsafe {
            if CFNumberGetValue(
                self.as_raw(),
                CFNumberType::SInt64,
                &mut v as *mut i64 as *mut c_void,
            ) != 0
            {
                Some(v)
            } else {
                None
            }
        }
    }

    pub fn to_f32(&self) -> Option<f32> {
        let mut v: f32 = 0.0;
        unsafe {
            if CFNumberGetValue(
                self.as_raw(),
                CFNumberType::Float32,
                &mut v as *mut f32 as *mut c_void,
            ) != 0
            {
                Some(v)
            } else {
                None
            }
        }
    }

    pub fn to_f64(&self) -> Option<f64> {
        let mut v: f64 = 0.0;
        unsafe {
            if CFNumberGetValue(
                self.as_raw(),
                CFNumberType::Float64,
                &mut v as *mut f64 as *mut c_void,
            ) != 0
            {
                Some(v)
            } else {
                None
            }
        }
    }

    /// The CFNumberType stored internally.
    pub fn number_type(&self) -> CFNumberType {
        unsafe { CFNumberGetType(self.as_raw()) }
    }

    /// Wrap a CFNumberRef you own (from a Create/Copy function).
    #[inline]
    pub unsafe fn wrap(ptr: CFNumberRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    /// Wrap a borrowed CFNumberRef (from a Get function) by retaining it.
    #[inline]
    pub unsafe fn retain(ptr: CFNumberRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CFNumberRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CFNumber {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl PartialEq for CFNumber {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl fmt::Debug for CFNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(v) = self.to_f64() {
            write!(f, "CFNumber({v})")
        } else {
            write!(f, "CFNumber(?)")
        }
    }
}

impl From<i32> for CFNumber {
    fn from(v: i32) -> Self {
        Self::from_i32(v)
    }
}

impl From<i64> for CFNumber {
    fn from(v: i64) -> Self {
        Self::from_i64(v)
    }
}

impl From<f32> for CFNumber {
    fn from(v: f32) -> Self {
        Self::from_f32(v)
    }
}

impl From<f64> for CFNumber {
    fn from(v: f64) -> Self {
        Self::from_f64(v)
    }
}
