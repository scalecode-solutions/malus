//! The `View` trait and all built-in view types.

mod text;
mod textfield;
mod button;
mod stack;
mod spacer;

pub use text::Text;
pub use textfield::TextField;
pub use button::Button;
pub use stack::{VStack, HStack};
pub use spacer::Spacer;

use crate::runtime::Id;

/// A declarative UI element that can be materialized into a native NSView.
pub trait View {
    /// Build this view (and its children) into a native NSView tree.
    /// The returned Id is a retained NSView (or subclass).
    unsafe fn build(&self) -> Id;
}

// Blanket: Box<dyn View> delegates to the inner type
impl View for Box<dyn View> {
    unsafe fn build(&self) -> Id {
        (**self).build()
    }
}
