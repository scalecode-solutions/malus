//! MTLBuffer — GPU-accessible memory.

use crate::runtime::*;
use std::ffi::c_void;

pub struct MTLBuffer(Id);

impl MTLBuffer {
    /// Pointer to the buffer's contents (CPU-accessible for shared/managed storage).
    pub fn contents(&self) -> *mut c_void {
        unsafe {
            msg_send!(self.0, sel!("contents"), fn(Id, Sel) -> *mut c_void)
        }
    }

    /// The length of the buffer in bytes.
    pub fn length(&self) -> usize {
        unsafe {
            msg_send!(self.0, sel!("length"), fn(Id, Sel) -> NSUInteger)
        }
    }

    /// Get the label.
    pub fn label(&self) -> String {
        unsafe {
            let ns_str: Id = msg_send!(self.0, sel!("label"), fn(Id, Sel) -> Id);
            from_nsstring(ns_str)
        }
    }

    /// Set the label.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns_str = nsstring(label);
            msg_send!(self.0, sel!("setLabel:"), fn(Id, Sel, Id) -> (), ns_str);
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

impl Clone for MTLBuffer {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLBuffer {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

unsafe impl Send for MTLBuffer {}
