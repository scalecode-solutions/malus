//! MTLCommandQueue — serialized command buffer submission.

use crate::runtime::*;
use super::command_buffer::MTLCommandBuffer;

pub struct MTLCommandQueue(Id);

impl MTLCommandQueue {
    /// Create a new command buffer from this queue.
    pub fn new_command_buffer(&self) -> MTLCommandBuffer {
        unsafe {
            let raw: Id = msg_send!(self.0, sel!("commandBuffer"), fn(Id, Sel) -> Id);
            assert!(!raw.is_null(), "commandBuffer returned null");
            // Command buffers from commandBuffer are autoreleased; retain.
            retain(raw);
            MTLCommandBuffer::from_raw(raw)
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

    /// Construct from a raw pointer. Takes ownership (no extra retain).
    #[inline]
    pub(crate) unsafe fn from_raw(ptr: Id) -> Self {
        Self(ptr)
    }
}

impl Clone for MTLCommandQueue {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLCommandQueue {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

unsafe impl Send for MTLCommandQueue {}
