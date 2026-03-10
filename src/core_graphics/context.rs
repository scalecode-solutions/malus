//! CGContext — 2D drawing destination (bitmap, PDF, window).

use super::color::{CGColor, CGColorRef};
use super::color_space::CGColorSpaceRef;
use super::geometry::*;
use super::image::{CGBitmapInfo, CGImage, CGImageRef, __CGImage};
use crate::core_foundation::base::{CFRef, CFTypeRef};
use std::ffi::c_void;
use std::fmt;

#[repr(C)]
pub struct __CGContext(c_void);
pub type CGContextRef = *mut __CGContext;

/// Line cap styles.
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CGLineCap {
    Butt = 0,
    Round = 1,
    Square = 2,
}

/// Line join styles.
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CGLineJoin {
    Miter = 0,
    Round = 1,
    Bevel = 2,
}

/// Text drawing modes.
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CGTextDrawingMode {
    Fill = 0,
    Stroke = 1,
    FillStroke = 2,
    Invisible = 3,
    FillClip = 4,
    StrokeClip = 5,
    FillStrokeClip = 6,
    Clip = 7,
}

/// Blend modes for compositing.
#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum CGBlendMode {
    Normal = 0,
    Multiply = 1,
    Screen = 2,
    Overlay = 3,
    Darken = 4,
    Lighten = 5,
    ColorDodge = 6,
    ColorBurn = 7,
    SoftLight = 8,
    HardLight = 9,
    Difference = 10,
    Exclusion = 11,
    Hue = 12,
    Saturation = 13,
    Color = 14,
    Luminosity = 15,
    Clear = 16,
    Copy = 17,
    SourceIn = 18,
    SourceOut = 19,
    SourceAtop = 20,
    DestinationOver = 21,
    DestinationIn = 22,
    DestinationOut = 23,
    DestinationAtop = 24,
    Xor = 25,
    PlusDarker = 26,
    PlusLighter = 27,
}

extern "C" {
    // Bitmap context
    fn CGBitmapContextCreate(
        data: *mut c_void,
        width: usize,
        height: usize,
        bits_per_component: usize,
        bytes_per_row: usize,
        space: CGColorSpaceRef,
        bitmap_info: CGBitmapInfo,
    ) -> CGContextRef;
    fn CGBitmapContextCreateImage(context: CGContextRef) -> CGImageRef;
    fn CGBitmapContextGetData(context: CGContextRef) -> *mut c_void;
    fn CGBitmapContextGetWidth(context: CGContextRef) -> usize;
    fn CGBitmapContextGetHeight(context: CGContextRef) -> usize;

    // State
    fn CGContextSaveGState(c: CGContextRef);
    fn CGContextRestoreGState(c: CGContextRef);

    // Transform
    fn CGContextTranslateCTM(c: CGContextRef, tx: CGFloat, ty: CGFloat);
    fn CGContextScaleCTM(c: CGContextRef, sx: CGFloat, sy: CGFloat);
    fn CGContextRotateCTM(c: CGContextRef, angle: CGFloat);
    fn CGContextConcatCTM(c: CGContextRef, transform: CGAffineTransform);
    fn CGContextGetCTM(c: CGContextRef) -> CGAffineTransform;

    // Color
    fn CGContextSetFillColorWithColor(c: CGContextRef, color: CGColorRef);
    fn CGContextSetStrokeColorWithColor(c: CGContextRef, color: CGColorRef);
    fn CGContextSetAlpha(c: CGContextRef, alpha: CGFloat);
    fn CGContextSetBlendMode(c: CGContextRef, mode: CGBlendMode);

    // Stroke settings
    fn CGContextSetLineWidth(c: CGContextRef, width: CGFloat);
    fn CGContextSetLineCap(c: CGContextRef, cap: CGLineCap);
    fn CGContextSetLineJoin(c: CGContextRef, join: CGLineJoin);
    fn CGContextSetMiterLimit(c: CGContextRef, limit: CGFloat);
    fn CGContextSetLineDash(
        c: CGContextRef,
        phase: CGFloat,
        lengths: *const CGFloat,
        count: usize,
    );

    // Path construction
    fn CGContextBeginPath(c: CGContextRef);
    fn CGContextClosePath(c: CGContextRef);
    fn CGContextMoveToPoint(c: CGContextRef, x: CGFloat, y: CGFloat);
    fn CGContextAddLineToPoint(c: CGContextRef, x: CGFloat, y: CGFloat);
    fn CGContextAddCurveToPoint(
        c: CGContextRef,
        cp1x: CGFloat, cp1y: CGFloat,
        cp2x: CGFloat, cp2y: CGFloat,
        x: CGFloat, y: CGFloat,
    );
    fn CGContextAddQuadCurveToPoint(
        c: CGContextRef,
        cpx: CGFloat, cpy: CGFloat,
        x: CGFloat, y: CGFloat,
    );
    fn CGContextAddRect(c: CGContextRef, rect: CGRect);
    fn CGContextAddEllipseInRect(c: CGContextRef, rect: CGRect);
    fn CGContextAddArc(
        c: CGContextRef,
        x: CGFloat, y: CGFloat,
        radius: CGFloat,
        start_angle: CGFloat, end_angle: CGFloat,
        clockwise: i32,
    );

    // Path drawing
    fn CGContextStrokePath(c: CGContextRef);
    fn CGContextFillPath(c: CGContextRef);
    fn CGContextFillRect(c: CGContextRef, rect: CGRect);
    fn CGContextStrokeRect(c: CGContextRef, rect: CGRect);
    fn CGContextStrokeRectWithWidth(c: CGContextRef, rect: CGRect, width: CGFloat);
    fn CGContextFillEllipseInRect(c: CGContextRef, rect: CGRect);
    fn CGContextStrokeEllipseInRect(c: CGContextRef, rect: CGRect);
    fn CGContextClearRect(c: CGContextRef, rect: CGRect);

    // Clipping
    fn CGContextClip(c: CGContextRef);
    fn CGContextClipToRect(c: CGContextRef, rect: CGRect);
    fn CGContextGetClipBoundingBox(c: CGContextRef) -> CGRect;

    // Image drawing
    fn CGContextDrawImage(c: CGContextRef, rect: CGRect, image: CGImageRef);

    // Text
    fn CGContextSetTextPosition(c: CGContextRef, x: CGFloat, y: CGFloat);
    fn CGContextGetTextPosition(c: CGContextRef) -> CGPoint;
    fn CGContextSetTextDrawingMode(c: CGContextRef, mode: CGTextDrawingMode);
    fn CGContextSetTextMatrix(c: CGContextRef, t: CGAffineTransform);
    fn CGContextGetTextMatrix(c: CGContextRef) -> CGAffineTransform;
}

// ============================================================================
// CGContext newtype
// ============================================================================

pub struct CGContext(CFRef<__CGContext>);

impl CGContext {
    /// Create a bitmap context. Pass `None` for data to let CG allocate it.
    pub fn bitmap(
        width: usize,
        height: usize,
        bits_per_component: usize,
        bytes_per_row: usize,
        color_space: &super::color_space::CGColorSpace,
        bitmap_info: CGBitmapInfo,
    ) -> Self {
        unsafe {
            let raw = CGBitmapContextCreate(
                std::ptr::null_mut(),
                width,
                height,
                bits_per_component,
                bytes_per_row,
                color_space.as_raw(),
                bitmap_info,
            );
            Self(CFRef::wrap(raw as *const __CGContext)
                .expect("CGBitmapContextCreate returned null"))
        }
    }

    /// Create an 8-bit RGBA bitmap context (the common case).
    pub fn bitmap_rgba(width: usize, height: usize) -> Self {
        use super::image::CGImageAlphaInfo;
        let space = super::color_space::CGColorSpace::srgb();
        Self::bitmap(
            width,
            height,
            8,
            width * 4,
            &space,
            CGImageAlphaInfo::PremultipliedLast as u32,
        )
    }

    /// Create a CGImage from this bitmap context's current contents.
    pub fn create_image(&self) -> CGImage {
        unsafe {
            let raw = CGBitmapContextCreateImage(self.as_raw_mut());
            CGImage::wrap(raw).expect("CGBitmapContextCreateImage returned null")
        }
    }

    /// Pointer to the raw bitmap data (if this is a bitmap context).
    pub fn data(&self) -> *mut c_void {
        unsafe { CGBitmapContextGetData(self.as_raw_mut()) }
    }

    /// Pixel data as a mutable byte slice (bitmap context only).
    pub fn data_as_bytes(&self) -> &mut [u8] {
        unsafe {
            let ptr = self.data() as *mut u8;
            let w = CGBitmapContextGetWidth(self.as_raw_mut());
            let h = CGBitmapContextGetHeight(self.as_raw_mut());
            std::slice::from_raw_parts_mut(ptr, w * h * 4)
        }
    }

    // ===== State =====

    pub fn save(&self) {
        unsafe { CGContextSaveGState(self.as_raw_mut()) }
    }

    pub fn restore(&self) {
        unsafe { CGContextRestoreGState(self.as_raw_mut()) }
    }

    // ===== Transform =====

    pub fn translate(&self, tx: CGFloat, ty: CGFloat) {
        unsafe { CGContextTranslateCTM(self.as_raw_mut(), tx, ty) }
    }

    pub fn scale(&self, sx: CGFloat, sy: CGFloat) {
        unsafe { CGContextScaleCTM(self.as_raw_mut(), sx, sy) }
    }

    pub fn rotate(&self, angle: CGFloat) {
        unsafe { CGContextRotateCTM(self.as_raw_mut(), angle) }
    }

    pub fn concat_ctm(&self, transform: CGAffineTransform) {
        unsafe { CGContextConcatCTM(self.as_raw_mut(), transform) }
    }

    pub fn ctm(&self) -> CGAffineTransform {
        unsafe { CGContextGetCTM(self.as_raw_mut()) }
    }

    // ===== Color =====

    pub fn set_fill_color(&self, color: &CGColor) {
        unsafe { CGContextSetFillColorWithColor(self.as_raw_mut(), color.as_raw()) }
    }

    pub fn set_stroke_color(&self, color: &CGColor) {
        unsafe { CGContextSetStrokeColorWithColor(self.as_raw_mut(), color.as_raw()) }
    }

    pub fn set_alpha(&self, alpha: CGFloat) {
        unsafe { CGContextSetAlpha(self.as_raw_mut(), alpha) }
    }

    pub fn set_blend_mode(&self, mode: CGBlendMode) {
        unsafe { CGContextSetBlendMode(self.as_raw_mut(), mode) }
    }

    // ===== Stroke =====

    pub fn set_line_width(&self, width: CGFloat) {
        unsafe { CGContextSetLineWidth(self.as_raw_mut(), width) }
    }

    pub fn set_line_cap(&self, cap: CGLineCap) {
        unsafe { CGContextSetLineCap(self.as_raw_mut(), cap) }
    }

    pub fn set_line_join(&self, join: CGLineJoin) {
        unsafe { CGContextSetLineJoin(self.as_raw_mut(), join) }
    }

    pub fn set_miter_limit(&self, limit: CGFloat) {
        unsafe { CGContextSetMiterLimit(self.as_raw_mut(), limit) }
    }

    pub fn set_line_dash(&self, phase: CGFloat, lengths: &[CGFloat]) {
        unsafe { CGContextSetLineDash(self.as_raw_mut(), phase, lengths.as_ptr(), lengths.len()) }
    }

    // ===== Path construction =====

    pub fn begin_path(&self) {
        unsafe { CGContextBeginPath(self.as_raw_mut()) }
    }

    pub fn close_path(&self) {
        unsafe { CGContextClosePath(self.as_raw_mut()) }
    }

    pub fn move_to(&self, x: CGFloat, y: CGFloat) {
        unsafe { CGContextMoveToPoint(self.as_raw_mut(), x, y) }
    }

    pub fn line_to(&self, x: CGFloat, y: CGFloat) {
        unsafe { CGContextAddLineToPoint(self.as_raw_mut(), x, y) }
    }

    pub fn curve_to(
        &self,
        cp1x: CGFloat, cp1y: CGFloat,
        cp2x: CGFloat, cp2y: CGFloat,
        x: CGFloat, y: CGFloat,
    ) {
        unsafe { CGContextAddCurveToPoint(self.as_raw_mut(), cp1x, cp1y, cp2x, cp2y, x, y) }
    }

    pub fn quad_curve_to(&self, cpx: CGFloat, cpy: CGFloat, x: CGFloat, y: CGFloat) {
        unsafe { CGContextAddQuadCurveToPoint(self.as_raw_mut(), cpx, cpy, x, y) }
    }

    pub fn add_rect(&self, rect: CGRect) {
        unsafe { CGContextAddRect(self.as_raw_mut(), rect) }
    }

    pub fn add_ellipse(&self, rect: CGRect) {
        unsafe { CGContextAddEllipseInRect(self.as_raw_mut(), rect) }
    }

    pub fn add_arc(
        &self,
        x: CGFloat, y: CGFloat,
        radius: CGFloat,
        start_angle: CGFloat, end_angle: CGFloat,
        clockwise: bool,
    ) {
        unsafe {
            CGContextAddArc(
                self.as_raw_mut(),
                x, y, radius, start_angle, end_angle,
                if clockwise { 1 } else { 0 },
            )
        }
    }

    // ===== Path drawing =====

    pub fn stroke_path(&self) {
        unsafe { CGContextStrokePath(self.as_raw_mut()) }
    }

    pub fn fill_path(&self) {
        unsafe { CGContextFillPath(self.as_raw_mut()) }
    }

    pub fn fill_rect(&self, rect: CGRect) {
        unsafe { CGContextFillRect(self.as_raw_mut(), rect) }
    }

    pub fn stroke_rect(&self, rect: CGRect) {
        unsafe { CGContextStrokeRect(self.as_raw_mut(), rect) }
    }

    pub fn stroke_rect_with_width(&self, rect: CGRect, width: CGFloat) {
        unsafe { CGContextStrokeRectWithWidth(self.as_raw_mut(), rect, width) }
    }

    pub fn fill_ellipse(&self, rect: CGRect) {
        unsafe { CGContextFillEllipseInRect(self.as_raw_mut(), rect) }
    }

    pub fn stroke_ellipse(&self, rect: CGRect) {
        unsafe { CGContextStrokeEllipseInRect(self.as_raw_mut(), rect) }
    }

    pub fn clear_rect(&self, rect: CGRect) {
        unsafe { CGContextClearRect(self.as_raw_mut(), rect) }
    }

    // ===== Clipping =====

    pub fn clip(&self) {
        unsafe { CGContextClip(self.as_raw_mut()) }
    }

    pub fn clip_to_rect(&self, rect: CGRect) {
        unsafe { CGContextClipToRect(self.as_raw_mut(), rect) }
    }

    pub fn clip_bounding_box(&self) -> CGRect {
        unsafe { CGContextGetClipBoundingBox(self.as_raw_mut()) }
    }

    // ===== Image drawing =====

    pub fn draw_image(&self, rect: CGRect, image: &CGImage) {
        unsafe { CGContextDrawImage(self.as_raw_mut(), rect, image.as_raw()) }
    }

    // ===== Text =====

    pub fn set_text_position(&self, x: CGFloat, y: CGFloat) {
        unsafe { CGContextSetTextPosition(self.as_raw_mut(), x, y) }
    }

    pub fn text_position(&self) -> CGPoint {
        unsafe { CGContextGetTextPosition(self.as_raw_mut()) }
    }

    pub fn set_text_drawing_mode(&self, mode: CGTextDrawingMode) {
        unsafe { CGContextSetTextDrawingMode(self.as_raw_mut(), mode) }
    }

    pub fn set_text_matrix(&self, t: CGAffineTransform) {
        unsafe { CGContextSetTextMatrix(self.as_raw_mut(), t) }
    }

    pub fn text_matrix(&self) -> CGAffineTransform {
        unsafe { CGContextGetTextMatrix(self.as_raw_mut()) }
    }

    // ===== Raw access =====

    #[inline]
    pub unsafe fn wrap(ptr: CGContextRef) -> Option<Self> {
        CFRef::wrap(ptr as *const __CGContext).map(Self)
    }

    #[inline]
    pub unsafe fn retain(ptr: CGContextRef) -> Option<Self> {
        CFRef::retain(ptr as *const __CGContext).map(Self)
    }

    #[inline]
    fn as_raw_mut(&self) -> CGContextRef {
        self.0.as_ptr() as CGContextRef
    }

    #[inline]
    pub fn as_raw(&self) -> CGContextRef {
        self.0.as_ptr() as CGContextRef
    }

    #[inline]
    pub fn as_type_ref(&self) -> CFTypeRef {
        self.0.as_type_ref()
    }
}

impl Clone for CGContext {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl fmt::Debug for CGContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CGContext({:?})", self.0)
    }
}
