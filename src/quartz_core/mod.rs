//! QuartzCore (Core Animation) — layers, animations, and display timing.
//!
//! ```rust,no_run
//! use malus::quartz_core::*;
//!
//! CATransaction::begin();
//! CATransaction::set_animation_duration(0.25);
//! let layer = CALayer::new();
//! layer.set_corner_radius(8.0);
//! CATransaction::commit();
//! ```

pub mod layer;
pub mod metal_layer;
pub mod animation;
pub mod transaction;
pub mod display_link;
pub mod transform;

pub use layer::CALayer;
pub use metal_layer::{CAMetalLayer, CAMetalDrawable};
pub use animation::{CAAnimation, CABasicAnimation, CAKeyframeAnimation};
pub use transaction::CATransaction;
pub use display_link::CADisplayLink;
pub use transform::CATransform3D;
