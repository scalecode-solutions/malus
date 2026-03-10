//! MTLVertexDescriptor and related types.

use crate::runtime::*;
use super::types::{MTLVertexFormat, MTLVertexStepFunction};

// ============================================================================
// MTLVertexDescriptor
// ============================================================================

pub struct MTLVertexDescriptor(Id);

impl MTLVertexDescriptor {
    /// Create a new vertex descriptor.
    pub fn new() -> Self {
        unsafe {
            let cls_id = cls!("MTLVertexDescriptor") as Id;
            let raw: Id = msg_send!(cls_id, sel!("vertexDescriptor"), fn(Id, Sel) -> Id);
            assert!(!raw.is_null(), "vertexDescriptor returned null");
            retain(raw);
            Self(raw)
        }
    }

    /// Get the attribute descriptor at the given index.
    pub fn attribute(&self, index: usize) -> MTLVertexAttributeDescriptor {
        unsafe {
            let attrs: Id = msg_send!(self.0, sel!("attributes"), fn(Id, Sel) -> Id);
            let attr: Id = msg_send!(
                attrs,
                sel!("objectAtIndexedSubscript:"),
                fn(Id, Sel, NSUInteger) -> Id,
                index as NSUInteger
            );
            MTLVertexAttributeDescriptor(attr)
        }
    }

    /// Get the buffer layout descriptor at the given index.
    pub fn layout(&self, index: usize) -> MTLVertexBufferLayoutDescriptor {
        unsafe {
            let layouts: Id = msg_send!(self.0, sel!("layouts"), fn(Id, Sel) -> Id);
            let layout: Id = msg_send!(
                layouts,
                sel!("objectAtIndexedSubscript:"),
                fn(Id, Sel, NSUInteger) -> Id,
                index as NSUInteger
            );
            MTLVertexBufferLayoutDescriptor(layout)
        }
    }

    #[inline]
    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for MTLVertexDescriptor {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLVertexDescriptor {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

impl Default for MTLVertexDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MTLVertexAttributeDescriptor (borrowed — not individually retained)
// ============================================================================

/// A borrowed reference to an attribute descriptor inside a vertex descriptor.
/// These are not individually retained; the parent MTLVertexDescriptor owns them.
pub struct MTLVertexAttributeDescriptor(Id);

impl MTLVertexAttributeDescriptor {
    pub fn set_format(&self, format: MTLVertexFormat) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setFormat:"),
                fn(Id, Sel, NSUInteger) -> (),
                format as NSUInteger
            );
        }
    }

    pub fn set_offset(&self, offset: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setOffset:"),
                fn(Id, Sel, NSUInteger) -> (),
                offset as NSUInteger
            );
        }
    }

    pub fn set_buffer_index(&self, index: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setBufferIndex:"),
                fn(Id, Sel, NSUInteger) -> (),
                index as NSUInteger
            );
        }
    }

    #[inline]
    pub fn as_raw(&self) -> Id {
        self.0
    }
}

// ============================================================================
// MTLVertexBufferLayoutDescriptor (borrowed — not individually retained)
// ============================================================================

/// A borrowed reference to a buffer layout descriptor inside a vertex descriptor.
pub struct MTLVertexBufferLayoutDescriptor(Id);

impl MTLVertexBufferLayoutDescriptor {
    pub fn set_stride(&self, stride: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setStride:"),
                fn(Id, Sel, NSUInteger) -> (),
                stride as NSUInteger
            );
        }
    }

    pub fn set_step_function(&self, function: MTLVertexStepFunction) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setStepFunction:"),
                fn(Id, Sel, NSUInteger) -> (),
                function as NSUInteger
            );
        }
    }

    pub fn set_step_rate(&self, rate: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setStepRate:"),
                fn(Id, Sel, NSUInteger) -> (),
                rate as NSUInteger
            );
        }
    }

    #[inline]
    pub fn as_raw(&self) -> Id {
        self.0
    }
}
