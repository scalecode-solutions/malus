//! CTRun — a glyph run within a CTLine.

use crate::core_foundation::base::{CFRef, CFTypeRef, CFIndex};
use crate::core_graphics::geometry::{CGPoint, CGSize};
use std::ffi::c_void;
use std::fmt;

#[repr(C)]
pub struct __CTRun(c_void);
pub type CTRunRef = *const __CTRun;

/// CFRange — location + length pair used by CoreText.
#[repr(C)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CFRange {
    pub location: CFIndex,
    pub length: CFIndex,
}

extern "C" {
    fn CTRunGetGlyphCount(run: CTRunRef) -> CFIndex;
    fn CTRunGetStringRange(run: CTRunRef) -> CFRange;

    fn CTRunGetPositions(
        run: CTRunRef,
        range: CFRange,
        buffer: *mut CGPoint,
    );

    fn CTRunGetAdvances(
        run: CTRunRef,
        range: CFRange,
        buffer: *mut CGSize,
    );

    fn CTRunGetGlyphs(
        run: CTRunRef,
        range: CFRange,
        buffer: *mut super::font::CGGlyph,
    );
}

// ============================================================================
// CTRun newtype
// ============================================================================

pub struct CTRun(CFRef<__CTRun>);

impl CTRun {
    /// Number of glyphs in this run.
    pub fn glyph_count(&self) -> usize {
        unsafe { CTRunGetGlyphCount(self.as_raw()) as usize }
    }

    /// The string range that this run covers.
    pub fn string_range(&self) -> CFRange {
        unsafe { CTRunGetStringRange(self.as_raw()) }
    }

    /// Get glyph positions. Pass an empty range (0, 0) to get all.
    pub fn positions(&self) -> Vec<CGPoint> {
        let count = self.glyph_count();
        let mut buf = vec![CGPoint { x: 0.0, y: 0.0 }; count];
        let range = CFRange { location: 0, length: 0 };
        unsafe { CTRunGetPositions(self.as_raw(), range, buf.as_mut_ptr()) };
        buf
    }

    /// Get glyph advances. Pass an empty range (0, 0) to get all.
    pub fn advances(&self) -> Vec<CGSize> {
        let count = self.glyph_count();
        let mut buf = vec![CGSize { width: 0.0, height: 0.0 }; count];
        let range = CFRange { location: 0, length: 0 };
        unsafe { CTRunGetAdvances(self.as_raw(), range, buf.as_mut_ptr()) };
        buf
    }

    /// Get the glyphs in this run.
    pub fn glyphs(&self) -> Vec<super::font::CGGlyph> {
        let count = self.glyph_count();
        let mut buf = vec![0u16; count];
        let range = CFRange { location: 0, length: 0 };
        unsafe { CTRunGetGlyphs(self.as_raw(), range, buf.as_mut_ptr()) };
        buf
    }

    #[inline]
    pub unsafe fn wrap(ptr: CTRunRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    #[inline]
    pub unsafe fn retain(ptr: CTRunRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CTRunRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CTRun {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CTRun {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CTRun(glyphs={})", self.glyph_count())
    }
}
