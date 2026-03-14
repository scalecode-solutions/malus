//! MTLCommandBuffer — encoded GPU commands.

use crate::runtime::*;
use std::ffi::c_void;
use super::encoder::{MTLRenderCommandEncoder, MTLComputeCommandEncoder, MTLBlitCommandEncoder};
use super::render_pass::MTLRenderPassDescriptor;
use super::types::MTLCommandBufferStatus;

// ============================================================================
// ObjC block ABI — minimal layout for blocks with captured data
// ============================================================================

/// The ABI layout of an ObjC block. Apple's block implementation spec:
/// https://clang.llvm.org/docs/Block-ABI-Apple.html
#[repr(C)]
struct BlockLiteral<F> {
    isa: *const c_void,
    flags: i32,
    reserved: i32,
    invoke: unsafe extern "C" fn(*mut BlockLiteral<F>, Id),
    descriptor: *const BlockDescriptor<F>,
    closure: *mut F,
}

#[repr(C)]
struct BlockDescriptor<F> {
    reserved: usize,
    size: usize,
    copy_helper: unsafe extern "C" fn(*mut BlockLiteral<F>, *const BlockLiteral<F>),
    dispose_helper: unsafe extern "C" fn(*mut BlockLiteral<F>),
}

extern "C" {
    static _NSConcreteStackBlock: *const c_void;
}

/// Block flags: BLOCK_HAS_COPY_DISPOSE (1 << 25)
const BLOCK_HAS_COPY_DISPOSE: i32 = 1 << 25;

unsafe extern "C" fn block_invoke<F: FnOnce() + Send + 'static>(
    block: *mut BlockLiteral<F>,
    _cmd_buf: Id,
) {
    let closure = Box::from_raw((*block).closure);
    closure();
}

unsafe extern "C" fn block_copy<F>(
    dst: *mut BlockLiteral<F>,
    src: *const BlockLiteral<F>,
) {
    // The closure pointer is heap-allocated; copy the pointer.
    (*dst).closure = (*src).closure;
}

unsafe extern "C" fn block_dispose<F>(block: *mut BlockLiteral<F>) {
    // Drop the closure when the block is disposed.
    let _ = Box::from_raw((*block).closure);
}

// ============================================================================
// MTLCommandBuffer
// ============================================================================

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

    /// Wait until this command buffer has been scheduled for execution.
    pub fn wait_until_scheduled(&self) {
        unsafe {
            msg_send!(self.0, sel!("waitUntilScheduled"), fn(Id, Sel) -> ());
        }
    }

    /// Block until this command buffer has completed execution.
    pub fn wait_until_completed(&self) {
        unsafe {
            msg_send!(self.0, sel!("waitUntilCompleted"), fn(Id, Sel) -> ());
        }
    }

    /// Present a drawable when the command buffer is scheduled.
    pub fn present_drawable(&self, drawable: Id) {
        unsafe {
            msg_send!(
                self.0,
                sel!("presentDrawable:"),
                fn(Id, Sel, Id) -> (),
                drawable
            );
        }
    }

    /// Get the current status of this command buffer.
    pub fn status(&self) -> MTLCommandBufferStatus {
        unsafe {
            let raw: NSUInteger = msg_send!(self.0, sel!("status"), fn(Id, Sel) -> NSUInteger);
            std::mem::transmute(raw as u64)
        }
    }

    /// Set the label for debugging.
    pub fn set_label(&self, label: &str) {
        unsafe {
            let ns = nsstring(label);
            msg_send!(self.0, sel!("setLabel:"), fn(Id, Sel, Id) -> (), ns);
        }
    }

    /// Add a completion handler. Called on a background thread when the
    /// command buffer finishes execution.
    ///
    /// The closure receives no arguments (the command buffer ref is not
    /// forwarded to keep the API simple — check `status()` beforehand
    /// if needed).
    pub fn add_completed_handler<F>(&self, handler: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unsafe {
            let boxed = Box::into_raw(Box::new(handler));

            let descriptor = BlockDescriptor::<F> {
                reserved: 0,
                size: std::mem::size_of::<BlockLiteral<F>>(),
                copy_helper: block_copy::<F>,
                dispose_helper: block_dispose::<F>,
            };

            let block = BlockLiteral::<F> {
                isa: _NSConcreteStackBlock,
                flags: BLOCK_HAS_COPY_DISPOSE,
                reserved: 0,
                invoke: block_invoke::<F>,
                descriptor: &descriptor,
                closure: boxed,
            };

            msg_send!(
                self.0,
                sel!("addCompletedHandler:"),
                fn(Id, Sel, *const c_void) -> (),
                &block as *const BlockLiteral<F> as *const c_void
            );
            // The runtime copies the block (via copy_helper) before this
            // stack frame returns, so the stack block is safe here.
        }
    }

    /// Add a scheduled handler. Called when the command buffer is scheduled.
    pub fn add_scheduled_handler<F>(&self, handler: F)
    where
        F: FnOnce() + Send + 'static,
    {
        unsafe {
            let boxed = Box::into_raw(Box::new(handler));

            let descriptor = BlockDescriptor::<F> {
                reserved: 0,
                size: std::mem::size_of::<BlockLiteral<F>>(),
                copy_helper: block_copy::<F>,
                dispose_helper: block_dispose::<F>,
            };

            let block = BlockLiteral::<F> {
                isa: _NSConcreteStackBlock,
                flags: BLOCK_HAS_COPY_DISPOSE,
                reserved: 0,
                invoke: block_invoke::<F>,
                descriptor: &descriptor,
                closure: boxed,
            };

            msg_send!(
                self.0,
                sel!("addScheduledHandler:"),
                fn(Id, Sel, *const c_void) -> (),
                &block as *const BlockLiteral<F> as *const c_void
            );
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
