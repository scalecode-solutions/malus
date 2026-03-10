//! CTParagraphStyle — paragraph-level formatting.

use super::types::{CTTextAlignment, CTLineBreakMode};
use crate::core_foundation::base::{CFRef, CFTypeRef};
use std::ffi::c_void;
use std::fmt;

#[repr(C)]
pub struct __CTParagraphStyle(c_void);
pub type CTParagraphStyleRef = *const __CTParagraphStyle;

/// Specifier keys for paragraph style settings.
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CTParagraphStyleSpecifier {
    Alignment = 0,
    FirstLineHeadIndent = 1,
    HeadIndent = 2,
    TailIndent = 3,
    TabStops = 4,
    DefaultTabInterval = 5,
    LineBreakMode = 6,
    LineHeightMultiple = 7,
    MaximumLineHeight = 8,
    MinimumLineHeight = 9,
    LineSpacing = 10, // deprecated but still functional
    ParagraphSpacing = 11,
    ParagraphSpacingBefore = 12,
    BaseWritingDirection = 13,
    MaximumLineSpacing = 14,
    MinimumLineSpacing = 15,
    LineSpacingAdjustment = 16,
    LineBoundsOptions = 17,
}

/// A single paragraph style setting passed to `CTParagraphStyleCreate`.
#[repr(C)]
pub struct CTParagraphStyleSetting {
    pub spec: CTParagraphStyleSpecifier,
    pub value_size: usize,
    pub value: *const c_void,
}

extern "C" {
    fn CTParagraphStyleCreate(
        settings: *const CTParagraphStyleSetting,
        setting_count: usize,
    ) -> CTParagraphStyleRef;
}

// ============================================================================
// CTParagraphStyle newtype
// ============================================================================

pub struct CTParagraphStyle(CFRef<__CTParagraphStyle>);

impl CTParagraphStyle {
    /// Create a paragraph style from raw settings.
    ///
    /// # Safety
    /// The caller must ensure that each `CTParagraphStyleSetting` has a valid
    /// `value` pointer with the correct `value_size`.
    pub unsafe fn new(settings: &[CTParagraphStyleSetting]) -> Self {
        let raw = CTParagraphStyleCreate(settings.as_ptr(), settings.len());
        Self(CFRef::wrap(raw).expect("CTParagraphStyleCreate returned null"))
    }

    /// Create a default paragraph style (no custom settings).
    pub fn default_style() -> Self {
        unsafe {
            let raw = CTParagraphStyleCreate(std::ptr::null(), 0);
            Self(CFRef::wrap(raw).expect("CTParagraphStyleCreate returned null"))
        }
    }

    /// Convenience: create a paragraph style with just alignment.
    pub fn with_alignment(alignment: CTTextAlignment) -> Self {
        let setting = CTParagraphStyleSetting {
            spec: CTParagraphStyleSpecifier::Alignment,
            value_size: std::mem::size_of::<CTTextAlignment>(),
            value: &alignment as *const CTTextAlignment as *const c_void,
        };
        unsafe { Self::new(&[setting]) }
    }

    /// Convenience: create a paragraph style with alignment and line break mode.
    pub fn with_alignment_and_line_break(
        alignment: CTTextAlignment,
        line_break: CTLineBreakMode,
    ) -> Self {
        let settings = [
            CTParagraphStyleSetting {
                spec: CTParagraphStyleSpecifier::Alignment,
                value_size: std::mem::size_of::<CTTextAlignment>(),
                value: &alignment as *const CTTextAlignment as *const c_void,
            },
            CTParagraphStyleSetting {
                spec: CTParagraphStyleSpecifier::LineBreakMode,
                value_size: std::mem::size_of::<CTLineBreakMode>(),
                value: &line_break as *const CTLineBreakMode as *const c_void,
            },
        ];
        unsafe { Self::new(&settings) }
    }

    #[inline]
    pub unsafe fn wrap(ptr: CTParagraphStyleRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    #[inline]
    pub unsafe fn retain(ptr: CTParagraphStyleRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CTParagraphStyleRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CTParagraphStyle {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CTParagraphStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CTParagraphStyle({:?})", self.0)
    }
}
