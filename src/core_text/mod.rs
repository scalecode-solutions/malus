//! CoreText — font, text layout, and typesetting.
//!
//! ```rust,no_run
//! use malus::core_text::*;
//! use malus::core_foundation::CFString;
//!
//! let font = CTFont::new(&CFString::new("Helvetica"), 14.0);
//! let ascent = font.ascent();
//! ```

pub mod types;
pub mod string_attributes;
pub mod font_descriptor;
pub mod font;
pub mod font_collection;
pub mod font_manager;
pub mod run;
pub mod line;
pub mod frame;
pub mod paragraph;

pub use types::{CTFontSymbolicTraits, CTTextAlignment, CTLineBreakMode, CTWritingDirection};
pub use font_descriptor::CTFontDescriptor;
pub use font::{CTFont, CGGlyph};
pub use font_collection::CTFontCollection;
pub use font_manager::{register_fonts_for_url, unregister_fonts_for_url, CTFontManagerScope};
pub use run::{CTRun, CFRange};
pub use line::CTLine;
pub use frame::{CTFramesetter, CTFrame};
pub use paragraph::{CTParagraphStyle, CTParagraphStyleSetting, CTParagraphStyleSpecifier};
