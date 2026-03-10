//! Malus — Native Apple UI framework for Rust.
//!
//! Write pure Rust, get native macOS apps. No Swift, no Xcode.
//!
//! ```rust,no_run
//! use malus::*;
//!
//! fn main() {
//!     App::new("Hello")
//!         .window("Hello Malus", 400.0, 300.0, {
//!             vstack![
//!                 text("Welcome to Malus!"),
//!                 textfield("Type here..."),
//!                 button("Click me"),
//!             ]
//!         })
//!         .run();
//! }
//! ```

// Internal modules
#[macro_use]
pub(crate) mod runtime;
pub(crate) mod events;
pub(crate) mod reconciler;

// Framework bindings (feature-gated)
#[cfg(feature = "core-foundation")]
pub mod core_foundation;
#[cfg(feature = "core-graphics")]
pub mod core_graphics;
#[cfg(feature = "metal")]
pub mod metal;
#[cfg(feature = "core-text")]
pub mod core_text;
#[cfg(feature = "quartz-core")]
pub mod quartz_core;
#[cfg(feature = "appkit")]
pub mod appkit;

// Layer 3 declarative UI (uses raw msg_send for now, will migrate to appkit bindings)
#[cfg(feature = "appkit")]
pub mod views;
#[cfg(feature = "appkit")]
#[path = "app.rs"]
mod app_bootstrap;

// Public re-exports — the user-facing API
#[cfg(feature = "appkit")]
pub use app_bootstrap::App;
#[cfg(feature = "appkit")]
pub use views::{View, Text, TextField, Button, VStack, HStack, Spacer};

// ============================================================================
// Convenience constructors — lowercase function-style like SwiftUI
// ============================================================================

#[cfg(feature = "appkit")]
/// Create a `Text` label.
pub fn text(s: &str) -> Text {
    Text::new(s)
}

#[cfg(feature = "appkit")]
/// Create an editable `TextField`.
pub fn textfield(placeholder: &str) -> TextField {
    TextField::new(placeholder)
}

#[cfg(feature = "appkit")]
/// Create a `Button`.
pub fn button(label: &str) -> Button {
    Button::new(label)
}

#[cfg(feature = "appkit")]
/// Create a vertical stack of views.
pub fn vstack(children: Vec<Box<dyn View>>) -> VStack {
    VStack::new(children)
}

#[cfg(feature = "appkit")]
/// Create a horizontal stack of views.
pub fn hstack(children: Vec<Box<dyn View>>) -> HStack {
    HStack::new(children)
}

#[cfg(feature = "appkit")]
/// Create a flexible spacer.
pub fn spacer() -> Spacer {
    Spacer::new()
}

// ============================================================================
// vstack! / hstack! macros — ergonomic child lists
// ============================================================================

/// Build a `VStack` with a list of view expressions.
///
/// ```rust,no_run
/// # use malus::*;
/// vstack![
///     text("Hello"),
///     button("OK"),
/// ];
/// ```
#[cfg(feature = "appkit")]
#[macro_export]
macro_rules! vstack {
    ( $( $child:expr ),* $(,)? ) => {
        $crate::vstack(vec![ $( Box::new($child) as Box<dyn $crate::View> ),* ])
    };
}

/// Build an `HStack` with a list of view expressions.
#[cfg(feature = "appkit")]
#[macro_export]
macro_rules! hstack {
    ( $( $child:expr ),* $(,)? ) => {
        $crate::hstack(vec![ $( Box::new($child) as Box<dyn $crate::View> ),* ])
    };
}
