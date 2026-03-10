//! Spacer — flexible space in a stack (like SwiftUI's Spacer).

use crate::runtime::*;
use crate::views::View;

pub struct Spacer;

impl Spacer {
    pub fn new() -> Self {
        Spacer
    }
}

impl View for Spacer {
    unsafe fn build(&self) -> Id {
        // A transparent NSView that wants to expand
        let view = alloc_init(cls!("NSView") as Id);

        let _: () = msg_send!(
            view,
            sel!("setTranslatesAutoresizingMaskIntoConstraints:"),
            fn(Id, Sel, BOOL) -> (),
            no()
        );

        // Set low hugging priority so it expands to fill space
        // NSLayoutPriorityDefaultLow = 250
        // Orientation: both horizontal (0) and vertical (1)
        let _: () = msg_send!(
            view,
            sel!("setContentHuggingPriority:forOrientation:"),
            fn(Id, Sel, f32, NSInteger) -> (),
            1.0f32, // very low priority — wants to expand
            0isize  // horizontal
        );
        let _: () = msg_send!(
            view,
            sel!("setContentHuggingPriority:forOrientation:"),
            fn(Id, Sel, f32, NSInteger) -> (),
            1.0f32,
            1isize  // vertical
        );

        view
    }
}
