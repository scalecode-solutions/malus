//! MTLRenderPipelineDescriptor and MTLRenderPipelineState.

use crate::runtime::*;
use super::device::MTLDevice;
use super::library::MTLFunction;
use super::types::MTLPixelFormat;
use super::vertex::MTLVertexDescriptor;

// ============================================================================
// MTLRenderPipelineDescriptor
// ============================================================================

pub struct MTLRenderPipelineDescriptor(Id);

impl MTLRenderPipelineDescriptor {
    /// Create a new, empty pipeline descriptor.
    pub fn new() -> Self {
        unsafe {
            let raw = alloc_init(cls!("MTLRenderPipelineDescriptor") as Id);
            assert!(!raw.is_null(), "failed to create MTLRenderPipelineDescriptor");
            Self(raw)
        }
    }

    /// Set the vertex function.
    pub fn set_vertex_function(&self, function: &MTLFunction) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setVertexFunction:"),
                fn(Id, Sel, Id) -> (),
                function.as_raw()
            );
        }
    }

    /// Set the fragment function.
    pub fn set_fragment_function(&self, function: &MTLFunction) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setFragmentFunction:"),
                fn(Id, Sel, Id) -> (),
                function.as_raw()
            );
        }
    }

    /// Set the pixel format of a color attachment at the given index.
    pub fn set_color_attachment_pixel_format(&self, index: usize, format: MTLPixelFormat) {
        unsafe {
            let attachments: Id = msg_send!(
                self.0,
                sel!("colorAttachments"),
                fn(Id, Sel) -> Id
            );
            let attachment: Id = msg_send!(
                attachments,
                sel!("objectAtIndexedSubscript:"),
                fn(Id, Sel, NSUInteger) -> Id,
                index as NSUInteger
            );
            msg_send!(
                attachment,
                sel!("setPixelFormat:"),
                fn(Id, Sel, NSUInteger) -> (),
                format as NSUInteger
            );
        }
    }

    /// Set the depth attachment pixel format.
    pub fn set_depth_attachment_pixel_format(&self, format: MTLPixelFormat) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setDepthAttachmentPixelFormat:"),
                fn(Id, Sel, NSUInteger) -> (),
                format as NSUInteger
            );
        }
    }

    /// Set the stencil attachment pixel format.
    pub fn set_stencil_attachment_pixel_format(&self, format: MTLPixelFormat) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setStencilAttachmentPixelFormat:"),
                fn(Id, Sel, NSUInteger) -> (),
                format as NSUInteger
            );
        }
    }

    /// Set the vertex descriptor.
    pub fn set_vertex_descriptor(&self, desc: &MTLVertexDescriptor) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setVertexDescriptor:"),
                fn(Id, Sel, Id) -> (),
                desc.as_raw()
            );
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
}

impl Clone for MTLRenderPipelineDescriptor {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLRenderPipelineDescriptor {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

impl Default for MTLRenderPipelineDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MTLRenderPipelineState
// ============================================================================

pub struct MTLRenderPipelineState(Id);

impl MTLRenderPipelineState {
    /// Create a pipeline state from a device and descriptor.
    /// Returns an error string if compilation fails.
    pub fn new(
        device: &MTLDevice,
        descriptor: &MTLRenderPipelineDescriptor,
    ) -> Result<Self, String> {
        unsafe {
            let mut error: Id = std::ptr::null_mut();
            let raw: Id = msg_send!(
                device.as_raw(),
                sel!("newRenderPipelineStateWithDescriptor:error:"),
                fn(Id, Sel, Id, *mut Id) -> Id,
                descriptor.as_raw(),
                &mut error as *mut Id
            );
            if raw.is_null() {
                let desc: Id = msg_send!(error, sel!("localizedDescription"), fn(Id, Sel) -> Id);
                Err(from_nsstring(desc))
            } else {
                Ok(Self(raw))
            }
        }
    }

    /// Get the label.
    pub fn label(&self) -> String {
        unsafe {
            let ns_str: Id = msg_send!(self.0, sel!("label"), fn(Id, Sel) -> Id);
            from_nsstring(ns_str)
        }
    }

    #[inline]
    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for MTLRenderPipelineState {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLRenderPipelineState {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

unsafe impl Send for MTLRenderPipelineState {}
unsafe impl Sync for MTLRenderPipelineState {}
