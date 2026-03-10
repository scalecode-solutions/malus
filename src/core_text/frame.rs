//! CTFramesetter + CTFrame — multi-line text layout.

use super::line::{CFAttributedStringRef, CTLine, CTLineRef};
use super::run::CFRange;
use crate::core_foundation::array::CFArrayRef;
use crate::core_foundation::base::{CFRef, CFTypeRef};
use crate::core_graphics::geometry::CGPoint;
use crate::core_graphics::path::CGPathRef;
use std::ffi::c_void;
use std::fmt;

// ============================================================================
// CTFramesetter
// ============================================================================

#[repr(C)]
pub struct __CTFramesetter(c_void);
pub type CTFramesetterRef = *const __CTFramesetter;

#[repr(C)]
pub struct __CTFrame(c_void);
pub type CTFrameRef = *const __CTFrame;

extern "C" {
    fn CTFramesetterCreateWithAttributedString(
        attr_string: CFAttributedStringRef,
    ) -> CTFramesetterRef;

    fn CTFramesetterCreateFrame(
        framesetter: CTFramesetterRef,
        string_range: CFRange,
        path: CGPathRef,
        frame_attributes: *const c_void, // CFDictionaryRef, nullable
    ) -> CTFrameRef;

    fn CTFrameGetLines(frame: CTFrameRef) -> CFArrayRef;

    fn CTFrameGetLineOrigins(
        frame: CTFrameRef,
        range: CFRange,
        origins: *mut CGPoint,
    );
}

pub struct CTFramesetter(CFRef<__CTFramesetter>);

impl CTFramesetter {
    /// Create a framesetter from an attributed string.
    pub fn new(attr_string: CFAttributedStringRef) -> Self {
        unsafe {
            let raw = CTFramesetterCreateWithAttributedString(attr_string);
            Self(CFRef::wrap(raw).expect("CTFramesetterCreateWithAttributedString returned null"))
        }
    }

    /// Create a frame for the given path. Pass `CFRange { location: 0, length: 0 }`
    /// to typeset the entire string.
    pub fn create_frame(
        &self,
        string_range: CFRange,
        path: &crate::core_graphics::path::CGPath,
    ) -> CTFrame {
        unsafe {
            let raw = CTFramesetterCreateFrame(
                self.as_raw(),
                string_range,
                path.as_raw(),
                std::ptr::null(),
            );
            CTFrame(CFRef::wrap(raw).expect("CTFramesetterCreateFrame returned null"))
        }
    }

    #[inline]
    pub unsafe fn wrap(ptr: CTFramesetterRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    #[inline]
    pub unsafe fn retain(ptr: CTFramesetterRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CTFramesetterRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CTFramesetter {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CTFramesetter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CTFramesetter({:?})", self.0)
    }
}

// ============================================================================
// CTFrame
// ============================================================================

pub struct CTFrame(CFRef<__CTFrame>);

impl CTFrame {
    /// Get the lines in this frame.
    pub fn lines(&self) -> Vec<CTLine> {
        unsafe {
            let arr = CTFrameGetLines(self.as_raw());
            if arr.is_null() {
                return Vec::new();
            }
            let array = crate::core_foundation::array::CFArray::retain(arr);
            match array {
                None => Vec::new(),
                Some(array) => {
                    let count = array.len();
                    let mut lines = Vec::with_capacity(count);
                    for i in 0..count {
                        let line_ref = array.get(i) as CTLineRef;
                        if let Some(line) = CTLine::retain(line_ref) {
                            lines.push(line);
                        }
                    }
                    lines
                }
            }
        }
    }

    /// Get the origin point for each line. Pass `CFRange { location: 0, length: 0 }`
    /// to get origins for all lines.
    pub fn line_origins(&self, range: CFRange) -> Vec<CGPoint> {
        let lines = self.lines();
        let count = if range.length == 0 { lines.len() } else { range.length as usize };
        let mut origins = vec![CGPoint { x: 0.0, y: 0.0 }; count];
        unsafe { CTFrameGetLineOrigins(self.as_raw(), range, origins.as_mut_ptr()) };
        origins
    }

    #[inline]
    pub unsafe fn wrap(ptr: CTFrameRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    #[inline]
    pub unsafe fn retain(ptr: CTFrameRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CTFrameRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CTFrame {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CTFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CTFrame({:?})", self.0)
    }
}
