//! MTLDevice — the GPU abstraction.

use crate::runtime::*;
use std::ffi::c_void;

use super::buffer::MTLBuffer;
use super::command_queue::MTLCommandQueue;
use super::library::MTLLibrary;
use super::types::{MTLGPUFamily, MTLResourceOptions};

extern "C" {
    fn MTLCreateSystemDefaultDevice() -> Id;
}

/// A GPU device (wraps the `MTLDevice` protocol object).
pub struct MTLDevice(Id);

impl MTLDevice {
    /// Get the system default Metal device, if available.
    pub fn system_default() -> Option<Self> {
        unsafe {
            let raw = MTLCreateSystemDefaultDevice();
            if raw.is_null() {
                None
            } else {
                Some(Self(raw))
            }
        }
    }

    /// The device name.
    pub fn name(&self) -> String {
        unsafe {
            let ns_str: Id = msg_send!(self.0, sel!("name"), fn(Id, Sel) -> Id);
            from_nsstring(ns_str)
        }
    }

    /// Create a new command queue.
    pub fn new_command_queue(&self) -> MTLCommandQueue {
        unsafe {
            let raw: Id = msg_send!(self.0, sel!("newCommandQueue"), fn(Id, Sel) -> Id);
            assert!(!raw.is_null(), "newCommandQueue returned null");
            MTLCommandQueue::from_raw(raw)
        }
    }

    /// Compile a Metal library from source code.
    pub fn new_library_with_source(&self, source: &str) -> Result<MTLLibrary, String> {
        unsafe {
            let ns_src = nsstring(source);
            let options: Id = std::ptr::null_mut(); // nil options = defaults
            let mut error: Id = std::ptr::null_mut();
            let raw: Id = msg_send!(
                self.0,
                sel!("newLibraryWithSource:options:error:"),
                fn(Id, Sel, Id, Id, *mut Id) -> Id,
                ns_src, options, &mut error as *mut Id
            );
            if raw.is_null() {
                let desc: Id = msg_send!(error, sel!("localizedDescription"), fn(Id, Sel) -> Id);
                Err(from_nsstring(desc))
            } else {
                Ok(MTLLibrary::from_raw(raw))
            }
        }
    }

    /// Create a buffer with initial data.
    pub fn new_buffer_with_bytes(
        &self,
        data: &[u8],
        options: MTLResourceOptions,
    ) -> MTLBuffer {
        unsafe {
            let raw: Id = msg_send!(
                self.0,
                sel!("newBufferWithBytes:length:options:"),
                fn(Id, Sel, *const c_void, NSUInteger, NSUInteger) -> Id,
                data.as_ptr() as *const c_void,
                data.len() as NSUInteger,
                options as NSUInteger
            );
            assert!(!raw.is_null(), "newBufferWithBytes returned null");
            MTLBuffer::from_raw(raw)
        }
    }

    /// Create an empty buffer of the given length.
    pub fn new_buffer_with_length(
        &self,
        length: usize,
        options: MTLResourceOptions,
    ) -> MTLBuffer {
        unsafe {
            let raw: Id = msg_send!(
                self.0,
                sel!("newBufferWithLength:options:"),
                fn(Id, Sel, NSUInteger, NSUInteger) -> Id,
                length as NSUInteger,
                options as NSUInteger
            );
            assert!(!raw.is_null(), "newBufferWithLength returned null");
            MTLBuffer::from_raw(raw)
        }
    }

    /// Check whether the device supports a given GPU family.
    pub fn supports_family(&self, family: MTLGPUFamily) -> bool {
        unsafe {
            let result: BOOL = msg_send!(
                self.0,
                sel!("supportsFamily:"),
                fn(Id, Sel, NSInteger) -> BOOL,
                family as NSInteger
            );
            to_bool(result)
        }
    }

    /// Raw pointer access.
    #[inline]
    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for MTLDevice {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLDevice {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

impl std::fmt::Debug for MTLDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MTLDevice({:?}, \"{}\")", self.0, self.name())
    }
}

unsafe impl Send for MTLDevice {}
