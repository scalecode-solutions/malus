//! MTLRenderPassDescriptor and color attachment configuration.

use crate::runtime::*;
use super::texture::MTLTexture;
use super::types::{MTLClearColor, MTLLoadAction, MTLStoreAction};

// ============================================================================
// MTLRenderPassDescriptor
// ============================================================================

pub struct MTLRenderPassDescriptor(Id);

impl MTLRenderPassDescriptor {
    /// Create a new render pass descriptor.
    pub fn new() -> Self {
        unsafe {
            let cls_id = cls!("MTLRenderPassDescriptor") as Id;
            let raw: Id = msg_send!(cls_id, sel!("renderPassDescriptor"), fn(Id, Sel) -> Id);
            assert!(!raw.is_null(), "renderPassDescriptor returned null");
            retain(raw);
            Self(raw)
        }
    }

    /// Get the color attachment at the given index.
    pub fn color_attachment(&self, index: usize) -> MTLRenderPassColorAttachmentDescriptor {
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
            MTLRenderPassColorAttachmentDescriptor(attachment)
        }
    }

    /// Set the depth attachment texture.
    pub fn set_depth_attachment_texture(&self, texture: &MTLTexture) {
        unsafe {
            let depth: Id = msg_send!(self.0, sel!("depthAttachment"), fn(Id, Sel) -> Id);
            msg_send!(
                depth,
                sel!("setTexture:"),
                fn(Id, Sel, Id) -> (),
                texture.as_raw()
            );
        }
    }

    /// Set the stencil attachment texture.
    pub fn set_stencil_attachment_texture(&self, texture: &MTLTexture) {
        unsafe {
            let stencil: Id = msg_send!(self.0, sel!("stencilAttachment"), fn(Id, Sel) -> Id);
            msg_send!(
                stencil,
                sel!("setTexture:"),
                fn(Id, Sel, Id) -> (),
                texture.as_raw()
            );
        }
    }

    #[inline]
    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for MTLRenderPassDescriptor {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLRenderPassDescriptor {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

impl Default for MTLRenderPassDescriptor {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// MTLRenderPassColorAttachmentDescriptor (borrowed from parent)
// ============================================================================

/// Borrowed reference to a color attachment within a render pass descriptor.
pub struct MTLRenderPassColorAttachmentDescriptor(Id);

impl MTLRenderPassColorAttachmentDescriptor {
    /// Set the texture for this color attachment.
    pub fn set_texture(&self, texture: &MTLTexture) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setTexture:"),
                fn(Id, Sel, Id) -> (),
                texture.as_raw()
            );
        }
    }

    /// Set the clear color.
    pub fn set_clear_color(&self, color: MTLClearColor) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setClearColor:"),
                fn(Id, Sel, MTLClearColor) -> (),
                color
            );
        }
    }

    /// Get the clear color.
    pub fn clear_color(&self) -> MTLClearColor {
        unsafe {
            msg_send!(
                self.0,
                sel!("clearColor"),
                fn(Id, Sel) -> MTLClearColor
            )
        }
    }

    /// Set the load action.
    pub fn set_load_action(&self, action: MTLLoadAction) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setLoadAction:"),
                fn(Id, Sel, NSUInteger) -> (),
                action as NSUInteger
            );
        }
    }

    /// Get the load action.
    pub fn load_action(&self) -> MTLLoadAction {
        unsafe {
            let raw: NSUInteger = msg_send!(self.0, sel!("loadAction"), fn(Id, Sel) -> NSUInteger);
            std::mem::transmute(raw as u64)
        }
    }

    /// Set the store action.
    pub fn set_store_action(&self, action: MTLStoreAction) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setStoreAction:"),
                fn(Id, Sel, NSUInteger) -> (),
                action as NSUInteger
            );
        }
    }

    /// Get the store action.
    pub fn store_action(&self) -> MTLStoreAction {
        unsafe {
            let raw: NSUInteger = msg_send!(self.0, sel!("storeAction"), fn(Id, Sel) -> NSUInteger);
            std::mem::transmute(raw as u64)
        }
    }

    #[inline]
    pub fn as_raw(&self) -> Id {
        self.0
    }
}
