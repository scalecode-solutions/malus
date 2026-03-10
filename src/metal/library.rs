//! MTLLibrary and MTLFunction.

use crate::runtime::*;

// ============================================================================
// MTLFunction
// ============================================================================

pub struct MTLFunction(Id);

impl MTLFunction {
    /// Get the function name.
    pub fn name(&self) -> String {
        unsafe {
            let ns_str: Id = msg_send!(self.0, sel!("name"), fn(Id, Sel) -> Id);
            from_nsstring(ns_str)
        }
    }

    #[inline]
    pub fn as_raw(&self) -> Id {
        self.0
    }

    #[inline]
    pub(crate) unsafe fn from_raw(ptr: Id) -> Self {
        Self(ptr)
    }
}

impl Clone for MTLFunction {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLFunction {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

unsafe impl Send for MTLFunction {}

// ============================================================================
// MTLLibrary
// ============================================================================

pub struct MTLLibrary(Id);

impl MTLLibrary {
    /// Get a function by name from this library.
    pub fn new_function_with_name(&self, name: &str) -> Option<MTLFunction> {
        unsafe {
            let ns_name = nsstring(name);
            let raw: Id = msg_send!(
                self.0,
                sel!("newFunctionWithName:"),
                fn(Id, Sel, Id) -> Id,
                ns_name
            );
            if raw.is_null() {
                None
            } else {
                Some(MTLFunction::from_raw(raw))
            }
        }
    }

    /// Get all function names in this library.
    pub fn function_names(&self) -> Vec<String> {
        unsafe {
            let ns_array: Id = msg_send!(self.0, sel!("functionNames"), fn(Id, Sel) -> Id);
            if ns_array.is_null() {
                return Vec::new();
            }
            let count: NSUInteger = msg_send!(ns_array, sel!("count"), fn(Id, Sel) -> NSUInteger);
            let mut names = Vec::with_capacity(count);
            for i in 0..count {
                let ns_str: Id = msg_send!(
                    ns_array,
                    sel!("objectAtIndex:"),
                    fn(Id, Sel, NSUInteger) -> Id,
                    i as NSUInteger
                );
                names.push(from_nsstring(ns_str));
            }
            names
        }
    }

    #[inline]
    pub fn as_raw(&self) -> Id {
        self.0
    }

    #[inline]
    pub(crate) unsafe fn from_raw(ptr: Id) -> Self {
        Self(ptr)
    }
}

impl Clone for MTLLibrary {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLLibrary {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

unsafe impl Send for MTLLibrary {}
