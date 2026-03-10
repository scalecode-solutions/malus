//! CTLine — a line of laid-out glyphs.

use super::run::{CFRange, CTRun, CTRunRef};
use crate::core_foundation::array::CFArrayRef;
use crate::core_foundation::base::{CFRef, CFTypeRef, __CFAttributedString};
use crate::core_graphics::context::CGContextRef;
use crate::core_graphics::geometry::CGFloat;
use std::ffi::c_void;
use std::fmt;

pub type CFAttributedStringRef = *const __CFAttributedString;

#[repr(C)]
pub struct __CTLine(c_void);
pub type CTLineRef = *const __CTLine;

extern "C" {
    fn CTLineCreateWithAttributedString(
        attr_string: CFAttributedStringRef,
    ) -> CTLineRef;

    fn CTLineGetGlyphRuns(line: CTLineRef) -> CFArrayRef;
    fn CTLineGetStringRange(line: CTLineRef) -> CFRange;

    fn CTLineGetTypographicBounds(
        line: CTLineRef,
        ascent: *mut CGFloat,
        descent: *mut CGFloat,
        leading: *mut CGFloat,
    ) -> f64;

    fn CTLineDraw(line: CTLineRef, context: CGContextRef);
}

// ============================================================================
// CTLine newtype
// ============================================================================

pub struct CTLine(CFRef<__CTLine>);

impl CTLine {
    /// Create a line from a CFAttributedString.
    pub fn new(attr_string: CFAttributedStringRef) -> Self {
        unsafe {
            let raw = CTLineCreateWithAttributedString(attr_string);
            Self(CFRef::wrap(raw).expect("CTLineCreateWithAttributedString returned null"))
        }
    }

    /// Get the glyph runs as a CFArray of CTRunRef.
    /// The array is owned by the line; callers should retain individual runs.
    pub fn glyph_runs(&self) -> Vec<CTRun> {
        unsafe {
            let arr = CTLineGetGlyphRuns(self.as_raw());
            if arr.is_null() {
                return Vec::new();
            }
            // Retain the array to read it
            let array = crate::core_foundation::array::CFArray::retain(arr);
            match array {
                None => Vec::new(),
                Some(array) => {
                    let count = array.len();
                    let mut runs = Vec::with_capacity(count);
                    for i in 0..count {
                        let run_ref = array.get(i) as CTRunRef;
                        if let Some(run) = CTRun::retain(run_ref) {
                            runs.push(run);
                        }
                    }
                    runs
                }
            }
        }
    }

    /// Get the string range this line covers.
    pub fn string_range(&self) -> CFRange {
        unsafe { CTLineGetStringRange(self.as_raw()) }
    }

    /// Get typographic bounds: returns (width, ascent, descent, leading).
    pub fn typographic_bounds(&self) -> (f64, CGFloat, CGFloat, CGFloat) {
        unsafe {
            let mut ascent: CGFloat = 0.0;
            let mut descent: CGFloat = 0.0;
            let mut leading: CGFloat = 0.0;
            let width = CTLineGetTypographicBounds(
                self.as_raw(),
                &mut ascent,
                &mut descent,
                &mut leading,
            );
            (width, ascent, descent, leading)
        }
    }

    /// Draw the line into a CGContext at the current text position.
    pub fn draw(&self, context: &crate::core_graphics::context::CGContext) {
        unsafe { CTLineDraw(self.as_raw(), context.as_raw()) }
    }

    #[inline]
    pub unsafe fn wrap(ptr: CTLineRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    #[inline]
    pub unsafe fn retain(ptr: CTLineRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CTLineRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CTLine {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CTLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CTLine({:?})", self.0)
    }
}
