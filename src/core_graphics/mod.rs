//! CoreGraphics — 2D drawing, color, images, and display info.
//!
//! ```rust,no_run
//! use malus::core_graphics::*;
//!
//! // Create a bitmap context and draw into it
//! let ctx = CGContext::bitmap_rgba(100, 100);
//! ctx.set_fill_color(&CGColor::rgb(1.0, 0.0, 0.0));
//! ctx.fill_rect(CGRect::new(0.0, 0.0, 100.0, 100.0));
//! let image = ctx.create_image();
//! ```

pub mod geometry;
pub mod color_space;
pub mod color;
pub mod data_provider;
pub mod image;
pub mod context;
pub mod path;
pub mod gradient;
pub mod display;

pub use geometry::{CGFloat, CGPoint, CGSize, CGRect, CGAffineTransform};
pub use color_space::CGColorSpace;
pub use color::CGColor;
pub use data_provider::CGDataProvider;
pub use image::{CGImage, CGImageAlphaInfo, CGImageByteOrderInfo, CGColorRenderingIntent, CGBitmapInfo};
pub use context::{CGContext, CGLineCap, CGLineJoin, CGBlendMode, CGTextDrawingMode};
pub use path::{CGPath, CGMutablePath};
pub use gradient::{CGGradient, CGGradientDrawingOptions};
pub use display::CGDisplay;
