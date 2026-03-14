//! Metal command encoders — render, compute, and blit.

use crate::runtime::*;
use std::ffi::c_void;

use super::buffer::MTLBuffer;
use super::pipeline::{MTLRenderPipelineState, MTLComputePipelineState};
use super::sampler::MTLSamplerState;
use super::texture::MTLTexture;
use super::types::{MTLIndexType, MTLOrigin, MTLPrimitiveType, MTLSize};

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

    /// Bind a texture to the vertex shader at the given index.
    pub fn set_vertex_texture(&self, texture: &MTLTexture, index: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setVertexTexture:atIndex:"),
                fn(Id, Sel, Id, NSUInteger) -> (),
                texture.as_raw(),
                index as NSUInteger
            );
        }
    }

    /// Bind a sampler state to the vertex shader at the given index.
    pub fn set_vertex_sampler_state(&self, sampler: &MTLSamplerState, index: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setVertexSamplerState:atIndex:"),
                fn(Id, Sel, Id, NSUInteger) -> (),
                sampler.as_raw(),
                index as NSUInteger
            );
        }
    }

    /// Set the viewport.
    pub fn set_viewport(&self, x: f64, y: f64, width: f64, height: f64, z_near: f64, z_far: f64) {
        #[repr(C)]
        struct MTLViewport {
            x: f64, y: f64, width: f64, height: f64, z_near: f64, z_far: f64,
        }
        unsafe {
            let vp = MTLViewport { x, y, width, height, z_near, z_far };
            msg_send!(
                self.0,
                sel!("setViewport:"),
                fn(Id, Sel, MTLViewport) -> (),
                vp
            );
        }
    }

    /// Set the scissor rect.
    pub fn set_scissor_rect(&self, x: usize, y: usize, width: usize, height: usize) {
        #[repr(C)]
        struct MTLScissorRect {
            x: NSUInteger, y: NSUInteger, width: NSUInteger, height: NSUInteger,
        }
        unsafe {
            let rect = MTLScissorRect {
                x: x as NSUInteger, y: y as NSUInteger,
                width: width as NSUInteger, height: height as NSUInteger,
            };
            msg_send!(
                self.0,
                sel!("setScissorRect:"),
                fn(Id, Sel, MTLScissorRect) -> (),
                rect
            );
        }
    }

    /// Set the front-facing winding order (0 = clockwise, 1 = counter-clockwise).
    pub fn set_front_facing_winding(&self, winding: NSUInteger) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setFrontFacingWinding:"),
                fn(Id, Sel, NSUInteger) -> (),
                winding
            );
        }
    }

    /// Set the cull mode (0 = none, 1 = front, 2 = back).
    pub fn set_cull_mode(&self, mode: NSUInteger) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setCullMode:"),
                fn(Id, Sel, NSUInteger) -> (),
                mode
            );
        }
    }

    /// Draw non-indexed instanced primitives.
    pub fn draw_primitives_instanced(
        &self,
        primitive_type: MTLPrimitiveType,
        vertex_start: usize,
        vertex_count: usize,
        instance_count: usize,
    ) {
        unsafe {
            msg_send!(
                self.0,
                sel!("drawPrimitives:vertexStart:vertexCount:instanceCount:"),
                fn(Id, Sel, NSUInteger, NSUInteger, NSUInteger, NSUInteger) -> (),
                primitive_type as NSUInteger,
                vertex_start as NSUInteger,
                vertex_count as NSUInteger,
                instance_count as NSUInteger
            );
        }
    }

    /// Set the blend color.
    pub fn set_blend_color(&self, red: f32, green: f32, blue: f32, alpha: f32) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setBlendColorRed:green:blue:alpha:"),
                fn(Id, Sel, f32, f32, f32, f32) -> (),
                red, green, blue, alpha
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
    /// Set the compute pipeline state.
    pub fn set_compute_pipeline_state(&self, state: &MTLComputePipelineState) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setComputePipelineState:"),
                fn(Id, Sel, Id) -> (),
                state.as_raw()
            );
        }
    }

    /// Bind a buffer at the given index.
    pub fn set_buffer(&self, buffer: &MTLBuffer, offset: usize, index: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setBuffer:offset:atIndex:"),
                fn(Id, Sel, Id, NSUInteger, NSUInteger) -> (),
                buffer.as_raw(),
                offset as NSUInteger,
                index as NSUInteger
            );
        }
    }

    /// Set bytes directly at the given index.
    pub fn set_bytes(&self, bytes: &[u8], index: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setBytes:length:atIndex:"),
                fn(Id, Sel, *const c_void, NSUInteger, NSUInteger) -> (),
                bytes.as_ptr() as *const c_void,
                bytes.len() as NSUInteger,
                index as NSUInteger
            );
        }
    }

    /// Bind a texture at the given index.
    pub fn set_texture(&self, texture: &MTLTexture, index: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setTexture:atIndex:"),
                fn(Id, Sel, Id, NSUInteger) -> (),
                texture.as_raw(),
                index as NSUInteger
            );
        }
    }

    /// Bind a sampler state at the given index.
    pub fn set_sampler_state(&self, sampler: &MTLSamplerState, index: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setSamplerState:atIndex:"),
                fn(Id, Sel, Id, NSUInteger) -> (),
                sampler.as_raw(),
                index as NSUInteger
            );
        }
    }

    /// Dispatch threadgroups with threads per threadgroup.
    pub fn dispatch_threadgroups(
        &self,
        threadgroups_per_grid: MTLSize,
        threads_per_threadgroup: MTLSize,
    ) {
        unsafe {
            msg_send!(
                self.0,
                sel!("dispatchThreadgroups:threadsPerThreadgroup:"),
                fn(Id, Sel, MTLSize, MTLSize) -> (),
                threadgroups_per_grid,
                threads_per_threadgroup
            );
        }
    }

    /// Dispatch threads directly (non-uniform threadgroup sizes).
    pub fn dispatch_threads(
        &self,
        threads_per_grid: MTLSize,
        threads_per_threadgroup: MTLSize,
    ) {
        unsafe {
            msg_send!(
                self.0,
                sel!("dispatchThreads:threadsPerThreadgroup:"),
                fn(Id, Sel, MTLSize, MTLSize) -> (),
                threads_per_grid,
                threads_per_threadgroup
            );
        }
    }

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
    /// Copy from one texture to another.
    pub fn copy_from_texture(
        &self,
        source: &MTLTexture,
        source_slice: usize,
        source_level: usize,
        source_origin: MTLOrigin,
        source_size: MTLSize,
        destination: &MTLTexture,
        destination_slice: usize,
        destination_level: usize,
        destination_origin: MTLOrigin,
    ) {
        unsafe {
            msg_send!(
                self.0,
                sel!("copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:"),
                fn(Id, Sel, Id, NSUInteger, NSUInteger, MTLOrigin, MTLSize, Id, NSUInteger, NSUInteger, MTLOrigin) -> (),
                source.as_raw(),
                source_slice as NSUInteger,
                source_level as NSUInteger,
                source_origin,
                source_size,
                destination.as_raw(),
                destination_slice as NSUInteger,
                destination_level as NSUInteger,
                destination_origin
            );
        }
    }

    /// Copy from a buffer to a texture.
    pub fn copy_from_buffer_to_texture(
        &self,
        source: &MTLBuffer,
        source_offset: usize,
        source_bytes_per_row: usize,
        source_bytes_per_image: usize,
        source_size: MTLSize,
        destination: &MTLTexture,
        destination_slice: usize,
        destination_level: usize,
        destination_origin: MTLOrigin,
    ) {
        unsafe {
            msg_send!(
                self.0,
                sel!("copyFromBuffer:sourceOffset:sourceBytesPerRow:sourceBytesPerImage:sourceSize:toTexture:destinationSlice:destinationLevel:destinationOrigin:"),
                fn(Id, Sel, Id, NSUInteger, NSUInteger, NSUInteger, MTLSize, Id, NSUInteger, NSUInteger, MTLOrigin) -> (),
                source.as_raw(),
                source_offset as NSUInteger,
                source_bytes_per_row as NSUInteger,
                source_bytes_per_image as NSUInteger,
                source_size,
                destination.as_raw(),
                destination_slice as NSUInteger,
                destination_level as NSUInteger,
                destination_origin
            );
        }
    }

    /// Copy from a texture to a buffer.
    pub fn copy_from_texture_to_buffer(
        &self,
        source: &MTLTexture,
        source_slice: usize,
        source_level: usize,
        source_origin: MTLOrigin,
        source_size: MTLSize,
        destination: &MTLBuffer,
        destination_offset: usize,
        destination_bytes_per_row: usize,
        destination_bytes_per_image: usize,
    ) {
        unsafe {
            msg_send!(
                self.0,
                sel!("copyFromTexture:sourceSlice:sourceLevel:sourceOrigin:sourceSize:toBuffer:destinationOffset:destinationBytesPerRow:destinationBytesPerImage:"),
                fn(Id, Sel, Id, NSUInteger, NSUInteger, MTLOrigin, MTLSize, Id, NSUInteger, NSUInteger, NSUInteger) -> (),
                source.as_raw(),
                source_slice as NSUInteger,
                source_level as NSUInteger,
                source_origin,
                source_size,
                destination.as_raw(),
                destination_offset as NSUInteger,
                destination_bytes_per_row as NSUInteger,
                destination_bytes_per_image as NSUInteger
            );
        }
    }

    /// Generate mipmaps for a texture.
    pub fn generate_mipmaps(&self, texture: &MTLTexture) {
        unsafe {
            msg_send!(
                self.0,
                sel!("generateMipmapsForTexture:"),
                fn(Id, Sel, Id) -> (),
                texture.as_raw()
            );
        }
    }

    /// Fill a buffer with a constant byte value.
    pub fn fill_buffer(&self, buffer: &MTLBuffer, range_offset: usize, range_length: usize, value: u8) {
        #[repr(C)]
        struct NSRange { location: NSUInteger, length: NSUInteger }
        unsafe {
            msg_send!(
                self.0,
                sel!("fillBuffer:range:value:"),
                fn(Id, Sel, Id, NSRange, u8) -> (),
                buffer.as_raw(),
                NSRange { location: range_offset as NSUInteger, length: range_length as NSUInteger },
                value
            );
        }
    }

    /// Copy from one buffer to another.
    pub fn copy_from_buffer(
        &self,
        source: &MTLBuffer,
        source_offset: usize,
        destination: &MTLBuffer,
        destination_offset: usize,
        size: usize,
    ) {
        unsafe {
            msg_send!(
                self.0,
                sel!("copyFromBuffer:sourceOffset:toBuffer:destinationOffset:size:"),
                fn(Id, Sel, Id, NSUInteger, Id, NSUInteger, NSUInteger) -> (),
                source.as_raw(),
                source_offset as NSUInteger,
                destination.as_raw(),
                destination_offset as NSUInteger,
                size as NSUInteger
            );
        }
    }

    /// Synchronize a managed resource (macOS only, for StorageModeManaged).
    pub fn synchronize_resource(&self, resource: Id) {
        unsafe {
            msg_send!(
                self.0,
                sel!("synchronizeResource:"),
                fn(Id, Sel, Id) -> (),
                resource
            );
        }
    }

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
