//! MTLTexture and MTLTextureDescriptor.

use crate::runtime::*;
use std::ffi::c_void;

use super::types::{MTLPixelFormat, MTLRegion};

// ============================================================================
// MTLTextureDescriptor
// ============================================================================

pub struct MTLTextureDescriptor(Id);

impl MTLTextureDescriptor {
    /// Create a descriptor for a 2D texture.
    pub fn texture_2d(
        pixel_format: MTLPixelFormat,
        width: usize,
        height: usize,
        mipmapped: bool,
    ) -> Self {
        unsafe {
            let cls_id = cls!("MTLTextureDescriptor") as Id;
            let raw: Id = msg_send!(
                cls_id,
                sel!("texture2DDescriptorWithPixelFormat:width:height:mipmapped:"),
                fn(Id, Sel, NSUInteger, NSUInteger, NSUInteger, BOOL) -> Id,
                pixel_format as NSUInteger,
                width as NSUInteger,
                height as NSUInteger,
                from_bool(mipmapped)
            );
            assert!(!raw.is_null(), "texture2DDescriptor returned null");
            retain(raw);
            Self(raw)
        }
    }

    /// Set the pixel format.
    pub fn set_pixel_format(&self, format: MTLPixelFormat) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setPixelFormat:"),
                fn(Id, Sel, NSUInteger) -> (),
                format as NSUInteger
            );
        }
    }

    /// Set the width.
    pub fn set_width(&self, width: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setWidth:"),
                fn(Id, Sel, NSUInteger) -> (),
                width as NSUInteger
            );
        }
    }

    /// Set the height.
    pub fn set_height(&self, height: usize) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setHeight:"),
                fn(Id, Sel, NSUInteger) -> (),
                height as NSUInteger
            );
        }
    }

    /// Set the usage flags (raw NSUInteger bitmask).
    pub fn set_usage(&self, usage: NSUInteger) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setUsage:"),
                fn(Id, Sel, NSUInteger) -> (),
                usage
            );
        }
    }

    #[inline]
    pub fn as_raw(&self) -> Id {
        self.0
    }
}

impl Clone for MTLTextureDescriptor {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLTextureDescriptor {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

// ============================================================================
// MTLTexture
// ============================================================================

pub struct MTLTexture(Id);

impl MTLTexture {
    /// Width in pixels.
    pub fn width(&self) -> usize {
        unsafe {
            msg_send!(self.0, sel!("width"), fn(Id, Sel) -> NSUInteger)
        }
    }

    /// Height in pixels.
    pub fn height(&self) -> usize {
        unsafe {
            msg_send!(self.0, sel!("height"), fn(Id, Sel) -> NSUInteger)
        }
    }

    /// The pixel format.
    pub fn pixel_format(&self) -> MTLPixelFormat {
        unsafe {
            let raw: NSUInteger = msg_send!(self.0, sel!("pixelFormat"), fn(Id, Sel) -> NSUInteger);
            std::mem::transmute(raw as u64)
        }
    }

    /// Replace a region of the texture with new pixel data.
    pub fn replace_region(
        &self,
        region: MTLRegion,
        mipmap_level: usize,
        bytes: *const c_void,
        bytes_per_row: usize,
    ) {
        unsafe {
            msg_send!(
                self.0,
                sel!("replaceRegion:mipmapLevel:withBytes:bytesPerRow:"),
                fn(Id, Sel, MTLRegion, NSUInteger, *const c_void, NSUInteger) -> (),
                region,
                mipmap_level as NSUInteger,
                bytes,
                bytes_per_row as NSUInteger
            );
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

    #[inline]
    pub unsafe fn from_raw(ptr: Id) -> Self {
        Self(ptr)
    }
}

impl Clone for MTLTexture {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for MTLTexture {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

unsafe impl Send for MTLTexture {}
