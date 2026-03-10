//! CoreText attributed-string keys (extern CFStringRef constants).

use crate::core_foundation::string::CFStringRef;

extern "C" {
    /// Value: CTFontRef
    pub static kCTFontAttributeName: CFStringRef;
    /// Value: CGColorRef
    pub static kCTForegroundColorAttributeName: CFStringRef;
    /// Value: CFNumberRef (CGFloat)
    pub static kCTKernAttributeName: CFStringRef;
    /// Value: CTParagraphStyleRef
    pub static kCTParagraphStyleAttributeName: CFStringRef;
    /// Value: CFNumberRef (float, 0.0 - 1.0)
    pub static kCTStrokeWidthAttributeName: CFStringRef;
    /// Value: CGColorRef
    pub static kCTStrokeColorAttributeName: CFStringRef;
    /// Value: CFNumberRef (int32, CTUnderlineStyle)
    pub static kCTUnderlineStyleAttributeName: CFStringRef;
    /// Value: CGColorRef
    pub static kCTUnderlineColorAttributeName: CFStringRef;
    /// Value: CGColorRef
    pub static kCTForegroundColorFromContextAttributeName: CFStringRef;
    /// Value: CFNumberRef (int32, CTSuperscriptStyle)
    pub static kCTSuperscriptAttributeName: CFStringRef;
    /// Value: CFNumberRef (CGFloat)
    pub static kCTBaselineOffsetAttributeName: CFStringRef;
    /// Value: CFNumberRef (int32, CTLigatureStyle)
    pub static kCTLigatureAttributeName: CFStringRef;
}
