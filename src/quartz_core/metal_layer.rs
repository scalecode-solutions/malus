//! CAMetalLayer / CAMetalDrawable — Metal-backed layer for GPU rendering.

use crate::runtime::*;

/// A Core Animation layer that Metal can render into.
pub struct CAMetalLayer(Id);

impl CAMetalLayer {
    /// Create a new Metal layer.
    pub fn new() -> Self {
        unsafe { Self(alloc_init(cls!("CAMetalLayer") as Id)) }
    }

    /// Wrap an existing retained pointer.
    ///
    /// # Safety
    /// `ptr` must be a valid, retained `CAMetalLayer` instance.
    pub unsafe fn from_raw(ptr: Id) -> Self {
        Self(ptr)
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }

    // -- Device --------------------------------------------------------------

    /// Set the Metal device (pass a raw `MTLDevice` pointer).
    pub fn set_device(&self, device: Id) {
        unsafe {
            msg_send!(self.0, sel!("setDevice:"), fn(Id, Sel, Id) -> (), device)
        }
    }

    /// Get the current Metal device.
    pub fn device(&self) -> Id {
        unsafe {
            msg_send!(self.0, sel!("device"), fn(Id, Sel) -> Id)
        }
    }

    // -- Pixel format --------------------------------------------------------

    /// Set the pixel format (NSUInteger). Common value: `BGRA8Unorm = 80`.
    pub fn set_pixel_format(&self, format: NSUInteger) {
        unsafe {
            msg_send!(self.0, sel!("setPixelFormat:"), fn(Id, Sel, NSUInteger) -> (), format)
        }
    }

    pub fn pixel_format(&self) -> NSUInteger {
        unsafe {
            msg_send!(self.0, sel!("pixelFormat"), fn(Id, Sel) -> NSUInteger)
        }
    }

    // -- Framebuffer ---------------------------------------------------------

    pub fn set_framebuffer_only(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setFramebufferOnly:"), fn(Id, Sel, BOOL) -> (), from_bool(flag))
        }
    }

    // -- Drawable size -------------------------------------------------------

    pub fn set_drawable_size(&self, size: CGSize) {
        unsafe {
            msg_send!(self.0, sel!("setDrawableSize:"), fn(Id, Sel, CGSize) -> (), size)
        }
    }

    pub fn drawable_size(&self) -> CGSize {
        unsafe {
            msg_send!(self.0, sel!("drawableSize"), fn(Id, Sel) -> CGSize)
        }
    }

    // -- Next drawable -------------------------------------------------------

    /// Get the next drawable for rendering. Returns `None` if unavailable.
    pub fn next_drawable(&self) -> Option<CAMetalDrawable> {
        unsafe {
            let obj: Id = msg_send!(self.0, sel!("nextDrawable"), fn(Id, Sel) -> Id);
            if obj.is_null() {
                None
            } else {
                // nextDrawable returns an autoreleased object; retain for ownership.
                Some(CAMetalDrawable(retain(obj)))
            }
        }
    }

    // -- Frame / scale (inherited from CALayer) ------------------------------

    pub fn set_frame(&self, frame: CGRect) {
        unsafe {
            msg_send!(self.0, sel!("setFrame:"), fn(Id, Sel, CGRect) -> (), frame)
        }
    }

    pub fn set_contents_scale(&self, scale: CGFloat) {
        unsafe {
            msg_send!(self.0, sel!("setContentsScale:"), fn(Id, Sel, CGFloat) -> (), scale)
        }
    }
}

impl Clone for CAMetalLayer {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for CAMetalLayer {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}

// ============================================================================
// CAMetalDrawable
// ============================================================================

/// A displayable resource obtained from a `CAMetalLayer`.
pub struct CAMetalDrawable(Id);

impl CAMetalDrawable {
    pub fn as_raw(&self) -> Id {
        self.0
    }

    /// Return the Metal texture backing this drawable (`MTLTexture`).
    pub fn texture(&self) -> Id {
        unsafe {
            msg_send!(self.0, sel!("texture"), fn(Id, Sel) -> Id)
        }
    }

    /// Return the layer that produced this drawable.
    pub fn layer(&self) -> Id {
        unsafe {
            msg_send!(self.0, sel!("layer"), fn(Id, Sel) -> Id)
        }
    }
}

impl Clone for CAMetalDrawable {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for CAMetalDrawable {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
