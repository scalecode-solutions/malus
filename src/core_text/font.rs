//! CTFont — a specific font instance for layout and rendering.

use super::font_descriptor::{CTFontDescriptor, CTFontDescriptorRef};
use crate::core_foundation::base::{CFRef, CFTypeRef, CFIndex};
use crate::core_foundation::string::CFStringRef;
use crate::core_graphics::context::CGContextRef;
use crate::core_graphics::geometry::*;
use std::ffi::c_void;
use std::fmt;

#[repr(C)]
pub struct __CTFont(c_void);
pub type CTFontRef = *const __CTFont;

/// A glyph identifier (same as CGGlyph).
pub type CGGlyph = u16;

extern "C" {
    fn CTFontCreateWithName(
        name: CFStringRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
    ) -> CTFontRef;

    fn CTFontCreateWithFontDescriptor(
        descriptor: CTFontDescriptorRef,
        size: CGFloat,
        matrix: *const CGAffineTransform,
    ) -> CTFontRef;

    fn CTFontGetAscent(font: CTFontRef) -> CGFloat;
    fn CTFontGetDescent(font: CTFontRef) -> CGFloat;
    fn CTFontGetLeading(font: CTFontRef) -> CGFloat;
    fn CTFontGetUnitsPerEm(font: CTFontRef) -> u32;
    fn CTFontGetBoundingBox(font: CTFontRef) -> CGRect;
    fn CTFontGetUnderlinePosition(font: CTFontRef) -> CGFloat;
    fn CTFontGetUnderlineThickness(font: CTFontRef) -> CGFloat;
    fn CTFontGetSize(font: CTFontRef) -> CGFloat;

    fn CTFontGetGlyphsForCharacters(
        font: CTFontRef,
        characters: *const u16,
        glyphs: *mut CGGlyph,
        count: CFIndex,
    ) -> bool;

    fn CTFontDrawGlyphs(
        font: CTFontRef,
        glyphs: *const CGGlyph,
        positions: *const CGPoint,
        count: usize,
        context: CGContextRef,
    );

    fn CTFontGetAdvancesForGlyphs(
        font: CTFontRef,
        orientation: u32, // CTFontOrientation
        glyphs: *const CGGlyph,
        advances: *mut CGSize,
        count: CFIndex,
    ) -> f64;

    fn CTFontCopyFontDescriptor(font: CTFontRef) -> CTFontDescriptorRef;
    fn CTFontCopyDisplayName(font: CTFontRef) -> CFStringRef;
    fn CTFontCopyFullName(font: CTFontRef) -> CFStringRef;
    fn CTFontCopyFamilyName(font: CTFontRef) -> CFStringRef;
    fn CTFontCopyPostScriptName(font: CTFontRef) -> CFStringRef;
}

// ============================================================================
// CTFont newtype
// ============================================================================

pub struct CTFont(CFRef<__CTFont>);

impl CTFont {
    /// Create a font from a PostScript name and point size.
    pub fn new(name: &crate::core_foundation::string::CFString, size: CGFloat) -> Self {
        unsafe {
            let raw = CTFontCreateWithName(name.as_raw(), size, std::ptr::null());
            Self(CFRef::wrap(raw).expect("CTFontCreateWithName returned null"))
        }
    }

    /// Create a font from a descriptor and point size.
    /// Pass `0.0` for size to use the descriptor's size.
    pub fn from_descriptor(descriptor: &CTFontDescriptor, size: CGFloat) -> Self {
        unsafe {
            let raw = CTFontCreateWithFontDescriptor(descriptor.as_raw(), size, std::ptr::null());
            Self(CFRef::wrap(raw).expect("CTFontCreateWithFontDescriptor returned null"))
        }
    }

    // ===== Metrics =====

    pub fn ascent(&self) -> CGFloat {
        unsafe { CTFontGetAscent(self.as_raw()) }
    }

    pub fn descent(&self) -> CGFloat {
        unsafe { CTFontGetDescent(self.as_raw()) }
    }

    pub fn leading(&self) -> CGFloat {
        unsafe { CTFontGetLeading(self.as_raw()) }
    }

    pub fn units_per_em(&self) -> u32 {
        unsafe { CTFontGetUnitsPerEm(self.as_raw()) }
    }

    pub fn bounding_box(&self) -> CGRect {
        unsafe { CTFontGetBoundingBox(self.as_raw()) }
    }

    pub fn underline_position(&self) -> CGFloat {
        unsafe { CTFontGetUnderlinePosition(self.as_raw()) }
    }

    pub fn underline_thickness(&self) -> CGFloat {
        unsafe { CTFontGetUnderlineThickness(self.as_raw()) }
    }

    pub fn size(&self) -> CGFloat {
        unsafe { CTFontGetSize(self.as_raw()) }
    }

    // ===== Glyphs =====

    /// Get glyphs for UTF-16 characters. Returns `true` if all characters were mapped.
    pub fn glyphs_for_characters(&self, characters: &[u16], glyphs: &mut [CGGlyph]) -> bool {
        assert_eq!(characters.len(), glyphs.len());
        unsafe {
            CTFontGetGlyphsForCharacters(
                self.as_raw(),
                characters.as_ptr(),
                glyphs.as_mut_ptr(),
                characters.len() as CFIndex,
            )
        }
    }

    /// Draw glyphs at the given positions into a CGContext.
    pub fn draw_glyphs(
        &self,
        glyphs: &[CGGlyph],
        positions: &[CGPoint],
        context: &crate::core_graphics::context::CGContext,
    ) {
        assert_eq!(glyphs.len(), positions.len());
        unsafe {
            CTFontDrawGlyphs(
                self.as_raw(),
                glyphs.as_ptr(),
                positions.as_ptr(),
                glyphs.len(),
                context.as_raw(),
            );
        }
    }

    /// Get the advance width for a single glyph (horizontal orientation).
    pub fn advance_for_glyph(&self, glyph: CGGlyph) -> CGFloat {
        unsafe {
            let glyphs = [glyph];
            let mut advances = [CGSize { width: 0.0, height: 0.0 }];
            CTFontGetAdvancesForGlyphs(
                self.as_raw(),
                0, // kCTFontOrientationHorizontal
                glyphs.as_ptr(),
                advances.as_mut_ptr(),
                1,
            );
            advances[0].width
        }
    }

    // ===== Names =====

    pub fn copy_descriptor(&self) -> CTFontDescriptor {
        unsafe {
            let raw = CTFontCopyFontDescriptor(self.as_raw());
            CTFontDescriptor::wrap(raw).expect("CTFontCopyFontDescriptor returned null")
        }
    }

    pub fn display_name(&self) -> Option<crate::core_foundation::string::CFString> {
        unsafe { crate::core_foundation::string::CFString::wrap(CTFontCopyDisplayName(self.as_raw())) }
    }

    pub fn full_name(&self) -> Option<crate::core_foundation::string::CFString> {
        unsafe { crate::core_foundation::string::CFString::wrap(CTFontCopyFullName(self.as_raw())) }
    }

    pub fn family_name(&self) -> Option<crate::core_foundation::string::CFString> {
        unsafe { crate::core_foundation::string::CFString::wrap(CTFontCopyFamilyName(self.as_raw())) }
    }

    pub fn postscript_name(&self) -> Option<crate::core_foundation::string::CFString> {
        unsafe { crate::core_foundation::string::CFString::wrap(CTFontCopyPostScriptName(self.as_raw())) }
    }

    // ===== Raw access =====

    #[inline]
    pub unsafe fn wrap(ptr: CTFontRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    #[inline]
    pub unsafe fn retain(ptr: CTFontRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CTFontRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CTFont {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CTFont {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CTFont({:?})", self.0)
    }
}
