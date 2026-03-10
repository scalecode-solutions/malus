//! CFTypeRef, CFRef<T> RAII wrapper, and shared CoreFoundation primitives.

use std::ffi::c_void;
use std::fmt;
use std::ptr::NonNull;

// ============================================================================
// Opaque CF type stubs — one per framework type
// ============================================================================

#[repr(C)]
pub struct __CFString(c_void);
#[repr(C)]
pub struct __CFNumber(c_void);
#[repr(C)]
pub struct __CFBoolean(c_void);
#[repr(C)]
pub struct __CFData(c_void);
#[repr(C)]
pub struct __CFArray(c_void);
#[repr(C)]
pub struct __CFDictionary(c_void);
#[repr(C)]
pub struct __CFURL(c_void);
#[repr(C)]
pub struct __CFError(c_void);
#[repr(C)]
pub struct __CFAttributedString(c_void);

// ============================================================================
// Primitive type aliases
// ============================================================================

pub type CFTypeRef = *const c_void;
pub type CFAllocatorRef = *const c_void;
pub type CFIndex = isize;
pub type CFTypeID = usize;
pub type CFOptionFlags = usize;
pub type CFHashCode = usize;

/// CoreFoundation Boolean (unsigned char, NOT ObjC BOOL).
pub type Boolean = u8;

// ============================================================================
// Raw extern functions
// ============================================================================

extern "C" {
    pub fn CFRetain(cf: CFTypeRef) -> CFTypeRef;
    pub fn CFRelease(cf: CFTypeRef);
    pub fn CFGetTypeID(cf: CFTypeRef) -> CFTypeID;
    pub fn CFEqual(cf1: CFTypeRef, cf2: CFTypeRef) -> Boolean;
    pub fn CFHash(cf: CFTypeRef) -> CFHashCode;
}

// ============================================================================
// CFRef<T> — generic RAII wrapper for any CoreFoundation type
// ============================================================================

/// Owned, reference-counted handle to a CoreFoundation object.
///
/// Calls `CFRelease` on drop. Clone calls `CFRetain`.
///
/// Ownership convention:
/// - `CFRef::wrap()` — takes ownership of an already-retained pointer
///   (from functions with Create/Copy in the name).
/// - `CFRef::retain()` — retains a borrowed pointer before wrapping
///   (from functions with Get in the name).
pub struct CFRef<T> {
    ptr: NonNull<T>,
}

impl<T> CFRef<T> {
    /// Wrap a pointer you already own (from Create/Copy functions).
    /// Returns `None` if the pointer is null.
    #[inline]
    pub unsafe fn wrap(ptr: *const T) -> Option<Self> {
        NonNull::new(ptr as *mut T).map(|ptr| Self { ptr })
    }

    /// Wrap a borrowed pointer (from Get functions) by retaining it.
    /// Returns `None` if the pointer is null.
    #[inline]
    pub unsafe fn retain(ptr: *const T) -> Option<Self> {
        if ptr.is_null() {
            return None;
        }
        CFRetain(ptr as CFTypeRef);
        Some(Self {
            ptr: NonNull::new_unchecked(ptr as *mut T),
        })
    }

    /// Get the raw pointer. Does not transfer ownership.
    #[inline]
    pub fn as_ptr(&self) -> *const T {
        self.ptr.as_ptr()
    }

    /// Get the pointer as a generic CFTypeRef.
    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.ptr.as_ptr() as CFTypeRef
    }
}

impl<T> Clone for CFRef<T> {
    fn clone(&self) -> Self {
        unsafe {
            CFRetain(self.as_type_ref());
        }
        Self { ptr: self.ptr }
    }
}

impl<T> Drop for CFRef<T> {
    fn drop(&mut self) {
        unsafe {
            CFRelease(self.as_type_ref());
        }
    }
}

impl<T> fmt::Debug for CFRef<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CFRef({:?})", self.ptr)
    }
}

impl<T> PartialEq for CFRef<T> {
    fn eq(&self, other: &Self) -> bool {
        unsafe { CFEqual(self.as_type_ref(), other.as_type_ref()) != 0 }
    }
}

// CF objects are refcounted and safe to send across threads.
// Individual types can opt out if needed.
unsafe impl<T> Send for CFRef<T> {}
