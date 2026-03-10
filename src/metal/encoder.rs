//! Metal command encoders — render, compute, and blit.

use crate::runtime::*;
use std::ffi::c_void;

use super::buffer::MTLBuffer;
use super::pipeline::MTLRenderPipelineState;
use super::sampler::MTLSamplerState;
use super::texture::MTLTexture;
use super::types::{MTLIndexType, MTLPrimitiveType};

// ============================================================================
// MTLRenderCommandEncoder
// ============================================================================

pub struct MTLRenderCommandEncoder(Id);

impl MTLRenderCommandEncoder {
    /// Set the render pipeline state.
    pub fn set_render_pipeline_state(&self, state: &MTLRenderPipelineState) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setRenderPipelineState:"),
                fn(Id, Sel, Id) -> (),
                state.as_raw()
            );
        }
    }

    /// Bind a vertex buffer at the given index.
    pub fn set_vertex_buffer(&self, buffer: &MTLBuffer, offset: usize, index: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setVertexBuffer:offset:atIndex:"),
                fn(Id, Sel, Id, NSUInteger, NSUInteger) -> (),
                buffer.as_raw(),
                offset as NSUInteger,
                index as NSUInteger
            );
        }
    }

    /// Bind a fragment buffer at the given index.
    pub fn set_fragment_buffer(&self, buffer: &MTLBuffer, offset: usize, index: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setFragmentBuffer:offset:atIndex:"),
                fn(Id, Sel, Id, NSUInteger, NSUInteger) -> (),
                buffer.as_raw(),
                offset as NSUInteger,
                index as NSUInteger
            );
        }
    }

    /// Set vertex bytes directly (small data, no buffer needed).
    pub fn set_vertex_bytes(&self, bytes: &[u8], index: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setVertexBytes:length:atIndex:"),
                fn(Id, Sel, *const c_void, NSUInteger, NSUInteger) -> (),
                bytes.as_ptr() as *const c_void,
                bytes.len() as NSUInteger,
                index as NSUInteger
            );
        }
    }

    /// Set fragment bytes directly (small data, no buffer needed).
    pub fn set_fragment_bytes(&self, bytes: &[u8], index: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setFragmentBytes:length:atIndex:"),
                fn(Id, Sel, *const c_void, NSUInteger, NSUInteger) -> (),
                bytes.as_ptr() as *const c_void,
                bytes.len() as NSUInteger,
                index as NSUInteger
            );
        }
    }

    /// Bind a texture to the fragment shader at the given index.
    pub fn set_fragment_texture(&self, texture: &MTLTexture, index: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setFragmentTexture:atIndex:"),
                fn(Id, Sel, Id, NSUInteger) -> (),
                texture.as_raw(),
                index as NSUInteger
            );
        }
    }

    /// Bind a sampler state to the fragment shader at the given index.
    pub fn set_fragment_sampler_state(&self, sampler: &MTLSamplerState, index: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setFragmentSamplerState:atIndex:"),
                fn(Id, Sel, Id, NSUInteger) -> (),
                sampler.as_raw(),
                index as NSUInteger
            );
        }
    }

    /// Draw non-indexed primitives.
    pub fn draw_primitives(
        &self,
        primitive_type: MTLPrimitiveType,
        vertex_start: usize,
        vertex_count: usize,
    ) {
        unsafe {
            msg_send!(
                self.0,
                sel!("drawPrimitives:vertexStart:vertexCount:"),
                fn(Id, Sel, NSUInteger, NSUInteger, NSUInteger) -> (),
                primitive_type as NSUInteger,
                vertex_start as NSUInteger,
                vertex_count as NSUInteger
            );
        }
    }

    /// Draw indexed primitives.
    pub fn draw_indexed_primitives(
        &self,
        primitive_type: MTLPrimitiveType,
        index_count: usize,
        index_type: MTLIndexType,
        index_buffer: &MTLBuffer,
        index_buffer_offset: usize,
    ) {
        unsafe {
            msg_send!(
                self.0,
                sel!("drawIndexedPrimitives:indexCount:indexType:indexBuffer:indexBufferOffset:"),
                fn(Id, Sel, NSUInteger, NSUInteger, NSUInteger, Id, NSUInteger) -> (),
                primitive_type as NSUInteger,
                index_count as NSUInteger,
                index_type as NSUInteger,
                index_buffer.as_raw(),
                index_buffer_offset as NSUInteger
            );
        }
    }

    /// Signal the end of encoding for this encoder.
    pub fn end_encoding(&self) {
        unsafe {
            msg_send!(self.0, sel!("endEncoding"), fn(Id, Sel) -> ());
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

impl Clone for MTLRenderCommandEncoder {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLRenderCommandEncoder {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

unsafe impl Send for MTLRenderCommandEncoder {}

// ============================================================================
// MTLComputeCommandEncoder
// ============================================================================

pub struct MTLComputeCommandEncoder(Id);

impl MTLComputeCommandEncoder {
    /// Signal the end of encoding.
    pub fn end_encoding(&self) {
        unsafe {
            msg_send!(self.0, sel!("endEncoding"), fn(Id, Sel) -> ());
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

impl Clone for MTLComputeCommandEncoder {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLComputeCommandEncoder {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

unsafe impl Send for MTLComputeCommandEncoder {}

// ============================================================================
// MTLBlitCommandEncoder
// ============================================================================

pub struct MTLBlitCommandEncoder(Id);

impl MTLBlitCommandEncoder {
    /// Signal the end of encoding.
    pub fn end_encoding(&self) {
        unsafe {
            msg_send!(self.0, sel!("endEncoding"), fn(Id, Sel) -> ());
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

impl Clone for MTLBlitCommandEncoder {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLBlitCommandEncoder {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

unsafe impl Send for MTLBlitCommandEncoder {}
