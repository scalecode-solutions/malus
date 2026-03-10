//! Metal enum and struct types.

use crate::runtime::NSUInteger;

// ============================================================================
// Pixel formats
// ============================================================================

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MTLPixelFormat {
    Invalid = 0,
    A8Unorm = 1,
    R8Unorm = 10,
    R8Snorm = 12,
    R8Uint = 13,
    R8Sint = 14,
    R16Unorm = 20,
    R16Snorm = 22,
    R16Uint = 23,
    R16Sint = 24,
    R16Float = 25,
    RG8Unorm = 30,
    RG8Snorm = 32,
    RG8Uint = 33,
    RG8Sint = 34,
    R32Uint = 53,
    R32Sint = 54,
    R32Float = 55,
    RG16Unorm = 60,
    RG16Snorm = 62,
    RG16Uint = 63,
    RG16Sint = 64,
    RG16Float = 65,
    RGBA8Unorm = 70,
    RGBA8UnormSRGB = 71,
    RGBA8Snorm = 72,
    RGBA8Uint = 73,
    RGBA8Sint = 74,
    BGRA8Unorm = 80,
    BGRA8UnormSRGB = 81,
    RGB10A2Unorm = 90,
    RG11B10Float = 92,
    RGB9E5Float = 93,
    RG32Uint = 103,
    RG32Sint = 104,
    RG32Float = 105,
    RGBA16Unorm = 110,
    RGBA16Snorm = 112,
    RGBA16Uint = 113,
    RGBA16Sint = 114,
    RGBA16Float = 115,
    RGBA32Uint = 123,
    RGBA32Sint = 124,
    RGBA32Float = 125,
    Depth16Unorm = 250,
    Depth32Float = 252,
    Stencil8 = 253,
    Depth24UnormStencil8 = 255,
    Depth32FloatStencil8 = 260,
}

// ============================================================================
// Primitive types
// ============================================================================

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MTLPrimitiveType {
    Point = 0,
    Line = 1,
    LineStrip = 2,
    Triangle = 3,
    TriangleStrip = 4,
}

// ============================================================================
// Load / Store actions
// ============================================================================

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MTLLoadAction {
    DontCare = 0,
    Load = 1,
    Clear = 2,
}

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MTLStoreAction {
    DontCare = 0,
    Store = 1,
    MultisampleResolve = 2,
    StoreAndMultisampleResolve = 3,
    Unknown = 4,
    CustomSampleDepthStore = 5,
}

// ============================================================================
// Resource options
// ============================================================================

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MTLResourceOptions {
    StorageModeShared = 0 << 4,
    StorageModeManaged = 1 << 4,
    StorageModePrivate = 2 << 4,
}

impl MTLResourceOptions {
    pub fn bits(self) -> NSUInteger {
        self as NSUInteger
    }
}

// ============================================================================
// Compare function (depth/stencil)
// ============================================================================

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MTLCompareFunction {
    Never = 0,
    Less = 1,
    Equal = 2,
    LessEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterEqual = 6,
    Always = 7,
}

// ============================================================================
// Vertex format
// ============================================================================

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MTLVertexFormat {
    Invalid = 0,
    UChar2 = 1,
    UChar3 = 2,
    UChar4 = 3,
    Char2 = 4,
    Char3 = 5,
    Char4 = 6,
    UChar2Normalized = 7,
    UChar3Normalized = 8,
    UChar4Normalized = 9,
    Char2Normalized = 10,
    Char3Normalized = 11,
    Char4Normalized = 12,
    UShort2 = 13,
    UShort3 = 14,
    UShort4 = 15,
    Short2 = 16,
    Short3 = 17,
    Short4 = 18,
    UShort2Normalized = 19,
    UShort3Normalized = 20,
    UShort4Normalized = 21,
    Short2Normalized = 22,
    Short3Normalized = 23,
    Short4Normalized = 24,
    Half2 = 25,
    Half3 = 26,
    Half4 = 27,
    Float = 28,
    Float2 = 29,
    Float3 = 30,
    Float4 = 31,
    Int = 32,
    Int2 = 33,
    Int3 = 34,
    Int4 = 35,
    UInt = 36,
    UInt2 = 37,
    UInt3 = 38,
    UInt4 = 39,
}

// ============================================================================
// Vertex step function
// ============================================================================

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MTLVertexStepFunction {
    Constant = 0,
    PerVertex = 1,
    PerInstance = 2,
    PerPatch = 3,
    PerPatchControlPoint = 4,
}

// ============================================================================
// Index type
// ============================================================================

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MTLIndexType {
    UInt16 = 0,
    UInt32 = 1,
}

// ============================================================================
// Clear color
// ============================================================================

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct MTLClearColor {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
    pub alpha: f64,
}

impl MTLClearColor {
    pub fn new(red: f64, green: f64, blue: f64, alpha: f64) -> Self {
        Self { red, green, blue, alpha }
    }
}

impl Default for MTLClearColor {
    fn default() -> Self {
        Self { red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0 }
    }
}

// ============================================================================
// Origin / Size / Region
// ============================================================================

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct MTLOrigin {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

impl MTLOrigin {
    pub fn new(x: usize, y: usize, z: usize) -> Self {
        Self { x, y, z }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct MTLSize {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
}

impl MTLSize {
    pub fn new(width: usize, height: usize, depth: usize) -> Self {
        Self { width, height, depth }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Default)]
pub struct MTLRegion {
    pub origin: MTLOrigin,
    pub size: MTLSize,
}

impl MTLRegion {
    pub fn new_2d(x: usize, y: usize, width: usize, height: usize) -> Self {
        Self {
            origin: MTLOrigin::new(x, y, 0),
            size: MTLSize::new(width, height, 1),
        }
    }
}

// ============================================================================
// GPU family
// ============================================================================

#[repr(i64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MTLGPUFamily {
    Common1 = 3001,
    Common2 = 3002,
    Common3 = 3003,
    Apple1 = 1001,
    Apple2 = 1002,
    Apple3 = 1003,
    Apple4 = 1004,
    Apple5 = 1005,
    Apple6 = 1006,
    Apple7 = 1007,
    Apple8 = 1008,
    Apple9 = 1009,
    Mac2 = 2002,
    Metal3 = 5001,
}

// ============================================================================
// Command buffer status
// ============================================================================

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MTLCommandBufferStatus {
    NotEnqueued = 0,
    Enqueued = 1,
    Committed = 2,
    Scheduled = 3,
    Completed = 4,
    Error = 5,
}

// ============================================================================
// Sampler enums
// ============================================================================

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MTLSamplerMinMagFilter {
    Nearest = 0,
    Linear = 1,
}

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MTLSamplerMipFilter {
    NotMipmapped = 0,
    Nearest = 1,
    Linear = 2,
}

#[repr(u64)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MTLSamplerAddressMode {
    ClampToEdge = 0,
    MirrorClampToEdge = 1,
    Repeat = 2,
    MirrorRepeat = 3,
    ClampToZero = 4,
    ClampToBorderColor = 5,
}
