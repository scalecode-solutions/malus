//! NSImage — a high-level image object.

use crate::runtime::*;

pub struct NSImage(pub(super) Id);

impl NSImage {
    /// Create a new empty image with the given size.
    pub fn new(size: CGSize) -> Self {
        unsafe {
            let obj = alloc(cls!("NSImage") as Id);
            let obj: Id = msg_send!(obj, sel!("initWithSize:"), fn(Id, Sel, CGSize) -> Id, size);
            Self(obj)
        }
    }

    /// Load an image from a file path. Returns None if the file cannot be loaded.
    pub fn from_file(path: &str) -> Option<Self> {
        unsafe {
            let ns = nsstring(path);
            let obj = alloc(cls!("NSImage") as Id);
            let obj: Id = msg_send!(
                obj,
                sel!("initWithContentsOfFile:"),
                fn(Id, Sel, Id) -> Id,
                ns
            );
            if obj.is_null() { None } else { Some(Self(obj)) }
        }
    }

    /// Look up a named system image. Returns None if not found.
    pub fn named(name: &str) -> Option<Self> {
        unsafe {
            let ns = nsstring(name);
            let raw: Id = msg_send!(
                cls!("NSImage") as Id,
                sel!("imageNamed:"),
                fn(Id, Sel, Id) -> Id,
                ns
            );
            if raw.is_null() {
                None
            } else {
                Some(Self(retain(raw)))
            }
        }
    }

    /// Create an NSImage from a CGImageRef with the given size.
    pub fn from_cg_image(cg_image: Id, size: CGSize) -> Self {
        unsafe {
            let obj = alloc(cls!("NSImage") as Id);
            let obj: Id = msg_send!(
                obj,
                sel!("initWithCGImage:size:"),
                fn(Id, Sel, Id, CGSize) -> Id,
                cg_image,
                size
            );
            Self(obj)
        }
    }

    pub fn as_raw(&self) -> Id {
        self.0
    }

    pub fn set_size(&self, size: CGSize) {
        unsafe {
            msg_send!(self.0, sel!("setSize:"), fn(Id, Sel, CGSize) -> (), size)
        }
    }

    pub fn size(&self) -> CGSize {
        unsafe { msg_send!(self.0, sel!("size"), fn(Id, Sel) -> CGSize) }
    }

    pub fn set_template(&self, flag: bool) {
        unsafe {
            msg_send!(self.0, sel!("setTemplate:"), fn(Id, Sel, BOOL) -> (), from_bool(flag))
        }
    }

    pub fn is_template(&self) -> bool {
        unsafe { to_bool(msg_send!(self.0, sel!("isTemplate"), fn(Id, Sel) -> BOOL)) }
    }
}

impl Clone for NSImage {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSImage {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
