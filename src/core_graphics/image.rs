//! CGImage — an immutable bitmap image.

use super::color_space::{CGColorSpace, CGColorSpaceRef};
use super::data_provider::{CGDataProvider, CGDataProviderRef};
use super::geometry::CGFloat;
use crate::core_foundation::base::{CFRef, CFTypeRef};
use std::ffi::c_void;
use std::fmt;

#[repr(C)]
pub struct __CGImage(c_void);
pub type CGImageRef = *const __CGImage;

/// Bitmap layout info — how pixel bytes are arranged.
pub type CGBitmapInfo = u32;

/// Alpha channel position and premultiplication.
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CGImageAlphaInfo {
    None = 0,
    PremultipliedLast = 1,
    PremultipliedFirst = 2,
    Last = 3,
    First = 4,
    NoneSkipLast = 5,
    NoneSkipFirst = 6,
    Only = 7,
}

/// Byte ordering of pixel components.
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CGImageByteOrderInfo {
    Default = 0,
    Order16Little = 1 << 12,
    Order32Little = 2 << 12,
    Order16Big = 3 << 12,
    Order32Big = 4 << 12,
}

/// Color rendering intent.
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CGColorRenderingIntent {
    Default = 0,
    AbsoluteColorimetric = 1,
    RelativeColorimetric = 2,
    Perceptual = 3,
    Saturation = 4,
}

extern "C" {
    fn CGImageCreate(
        width: usize,
        height: usize,
        bits_per_component: usize,
        bits_per_pixel: usize,
        bytes_per_row: usize,
        space: CGColorSpaceRef,
        bitmap_info: CGBitmapInfo,
        provider: CGDataProviderRef,
        decode: *const CGFloat,
        should_interpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> CGImageRef;

    fn CGImageGetWidth(image: CGImageRef) -> usize;
    fn CGImageGetHeight(image: CGImageRef) -> usize;
    fn CGImageGetBitsPerComponent(image: CGImageRef) -> usize;
    fn CGImageGetBitsPerPixel(image: CGImageRef) -> usize;
    fn CGImageGetBytesPerRow(image: CGImageRef) -> usize;
    fn CGImageGetColorSpace(image: CGImageRef) -> CGColorSpaceRef;
    fn CGImageGetAlphaInfo(image: CGImageRef) -> CGImageAlphaInfo;
    fn CGImageGetBitmapInfo(image: CGImageRef) -> CGBitmapInfo;
}

// ============================================================================
// CGImage newtype
// ============================================================================

pub struct CGImage(CFRef<__CGImage>);

impl CGImage {
    /// Create an image from raw pixel data.
    pub fn new(
        width: usize,
        height: usize,
        bits_per_component: usize,
        bits_per_pixel: usize,
        bytes_per_row: usize,
        color_space: &CGColorSpace,
        bitmap_info: CGBitmapInfo,
        provider: &CGDataProvider,
        should_interpolate: bool,
        intent: CGColorRenderingIntent,
    ) -> Self {
        unsafe {
            let raw = CGImageCreate(
                width,
                height,
                bits_per_component,
                bits_per_pixel,
                bytes_per_row,
                color_space.as_raw(),
                bitmap_info,
                provider.as_raw(),
                std::ptr::null(),
                should_interpolate,
                intent,
            );
            Self(CFRef::wrap(raw).expect("CGImageCreate returned null"))
        }
    }

    /// Create an 8-bit RGBA image from pixel data.
    pub fn from_rgba(width: usize, height: usize, data: &[u8]) -> Self {
        assert_eq!(data.len(), width * height * 4, "data length must be width * height * 4");
        let provider = CGDataProvider::from_bytes(data);
        let space = CGColorSpace::srgb();
        let info = CGImageAlphaInfo::PremultipliedLast as u32
            | CGImageByteOrderInfo::Default as u32;
        Self::new(width, height, 8, 32, width * 4, &space, info, &provider, true, CGColorRenderingIntent::Default)
    }

    pub fn width(&self) -> usize {
        unsafe { CGImageGetWidth(self.as_raw()) }
    }

    pub fn height(&self) -> usize {
        unsafe { CGImageGetHeight(self.as_raw()) }
    }

    pub fn bits_per_component(&self) -> usize {
        unsafe { CGImageGetBitsPerComponent(self.as_raw()) }
    }

    pub fn bits_per_pixel(&self) -> usize {
        unsafe { CGImageGetBitsPerPixel(self.as_raw()) }
    }

    pub fn bytes_per_row(&self) -> usize {
        unsafe { CGImageGetBytesPerRow(self.as_raw()) }
    }

    pub fn color_space(&self) -> CGColorSpace {
        unsafe {
            let ptr = CGImageGetColorSpace(self.as_raw());
            CGColorSpace::retain(ptr).expect("CGImageGetColorSpace returned null")
        }
    }

    pub fn alpha_info(&self) -> CGImageAlphaInfo {
        unsafe { CGImageGetAlphaInfo(self.as_raw()) }
    }

    pub fn bitmap_info(&self) -> CGBitmapInfo {
        unsafe { CGImageGetBitmapInfo(self.as_raw()) }
    }

    #[inline]
    pub unsafe fn wrap(ptr: CGImageRef) -> Option<Self> {
        CFRef::wrap(ptr).map(Self)
    }

    #[inline]
    pub unsafe fn retain(ptr: CGImageRef) -> Option<Self> {
        CFRef::retain(ptr).map(Self)
    }

    #[inline]
    pub fn as_raw(&self) -> CGImageRef {
        self.0.as_ptr()
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CGImage {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CGImage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CGImage({}x{}, {}bpp)", self.width(), self.height(), self.bits_per_pixel())
    }
}
