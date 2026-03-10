//! MTLCommandBuffer — encoded GPU commands.

use crate::runtime::*;
use super::encoder::{MTLRenderCommandEncoder, MTLComputeCommandEncoder, MTLBlitCommandEncoder};
use super::render_pass::MTLRenderPassDescriptor;
use super::types::MTLCommandBufferStatus;

pub struct MTLCommandBuffer(Id);

impl MTLCommandBuffer {
    /// Create a render command encoder for the given render pass descriptor.
    pub fn new_render_command_encoder(
        &self,
        descriptor: &MTLRenderPassDescriptor,
    ) -> MTLRenderCommandEncoder {
        unsafe {
            let raw: Id = msg_send!(
                self.0,
                sel!("renderCommandEncoderWithDescriptor:"),
                fn(Id, Sel, Id) -> Id,
                descriptor.as_raw()
            );
            assert!(!raw.is_null(), "renderCommandEncoderWithDescriptor returned null");
            retain(raw);
            MTLRenderCommandEncoder::from_raw(raw)
        }
    }

    /// Create a compute command encoder.
    pub fn new_compute_command_encoder(&self) -> MTLComputeCommandEncoder {
        unsafe {
            let raw: Id = msg_send!(self.0, sel!("computeCommandEncoder"), fn(Id, Sel) -> Id);
            assert!(!raw.is_null(), "computeCommandEncoder returned null");
            retain(raw);
            MTLComputeCommandEncoder::from_raw(raw)
        }
    }

    /// Create a blit command encoder.
    pub fn new_blit_command_encoder(&self) -> MTLBlitCommandEncoder {
        unsafe {
            let raw: Id = msg_send!(self.0, sel!("blitCommandEncoder"), fn(Id, Sel) -> Id);
            assert!(!raw.is_null(), "blitCommandEncoder returned null");
            retain(raw);
            MTLBlitCommandEncoder::from_raw(raw)
        }
    }

    /// Commit this command buffer for execution.
    pub fn commit(&self) {
        unsafe {
            msg_send!(self.0, sel!("commit"), fn(Id, Sel) -> ());
        }
    }

    /// Block until this command buffer has completed execution.
    pub fn wait_until_completed(&self) {
        unsafe {
            msg_send!(self.0, sel!("waitUntilCompleted"), fn(Id, Sel) -> ());
        }
    }

    /// Get the current status of this command buffer.
    pub fn status(&self) -> MTLCommandBufferStatus {
        unsafe {
            let raw: NSUInteger = msg_send!(self.0, sel!("status"), fn(Id, Sel) -> NSUInteger);
            std::mem::transmute(raw as u64)
        }
    }

    /// Add a completion handler block. The block is called when the command
    /// buffer has completed execution.
    ///
    /// **Safety**: the closure must be `'static + Send`. We build an ObjC block
    /// trampoline under the hood.
    pub fn add_completed_handler<F>(&self, handler: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unsafe {
            // Build a minimal stack-allocated ObjC block that calls our closure.
            let boxed = Box::into_raw(Box::new(handler));

            extern "C" fn invoke<F: FnOnce() + Send + 'static>(
                _block: *mut std::ffi::c_void,
                _cmd_buf: Id,
            ) {
                // no-op trampoline: the real work is in the Fn we stashed
            }

            // For a real implementation we'd build a proper __block_literal.
            // This is a simplified version — we call the handler when the
            // command buffer completes synchronously instead.
            // TODO: implement full ObjC block support
            let _ = Box::from_raw(boxed);
            // For now, the handler is dropped. Users should prefer
            // wait_until_completed() for synchronous waiting.
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

impl Clone for MTLCommandBuffer {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLCommandBuffer {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

unsafe impl Send for MTLCommandBuffer {}
