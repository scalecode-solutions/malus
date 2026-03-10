//! Enums and bitflags for CoreText.

/// Symbolic traits of a font (bitmask).
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct CTFontSymbolicTraits(pub u32);

#[allow(non_upper_case_globals)]
impl CTFontSymbolicTraits {
    pub const Bold: Self = Self(1 << 1);
    pub const Italic: Self = Self(1 << 0);
    pub const Expanded: Self = Self(1 << 5);
    pub const Condensed: Self = Self(1 << 6);
    pub const MonoSpace: Self = Self(1 << 10);
    pub const Vertical: Self = Self(1 << 11);
    pub const UIOptimized: Self = Self(1 << 12);
    pub const ColorGlyphs: Self = Self(1 << 13);
    pub const Composite: Self = Self(1 << 14);
}

impl CTFontSymbolicTraits {
    #[inline]
    pub fn contains(self, other: Self) -> bool {
        (self.0 & other.0) == other.0
    }
}

impl std::ops::BitOr for CTFontSymbolicTraits {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitAnd for CTFontSymbolicTraits {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

/// Text alignment for paragraph styles.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CTTextAlignment {
    Left = 0,
    Right = 1,
    Center = 2,
    Justified = 3,
    Natural = 4,
}

/// Line break modes for paragraph styles.
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CTLineBreakMode {
    WordWrapping = 0,
    CharWrapping = 1,
    Clipping = 2,
    TruncatingHead = 3,
    TruncatingTail = 4,
    TruncatingMiddle = 5,
}

/// Writing direction.
#[repr(i8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CTWritingDirection {
    Natural = -1,
    LeftToRight = 0,
    RightToLeft = 1,
}
