//! `VStack` / `HStack` — layout containers backed by NSStackView.

use crate::runtime::*;
use crate::views::View;
use crate::reconciler;

// ============================================================================
// Orientation
// ============================================================================

#[derive(Copy, Clone)]
pub(crate) enum Orientation {
    Vertical,   // NSUserInterfaceLayoutOrientationVertical = 1
    Horizontal, // NSUserInterfaceLayoutOrientationHorizontal = 0
}

// ============================================================================
// Stack (internal, shared between VStack and HStack)
// ============================================================================

pub(crate) struct Stack {
    pub orientation: Orientation,
    pub spacing: CGFloat,
    pub children: Vec<Box<dyn View>>,
}

impl View for Stack {
    unsafe fn build(&self) -> Id {
        let stack = alloc_init(cls!("NSStackView") as Id);

        // Set orientation
        let orient_val: NSInteger = match self.orientation {
            Orientation::Vertical => 1,
            Orientation::Horizontal => 0,
        };
        let _: () = msg_send!(
            stack,
            sel!("setOrientation:"),
            fn(Id, Sel, NSInteger) -> (),
            orient_val
        );

        // Set spacing
        let _: () = msg_send!(
            stack,
            sel!("setSpacing:"),
            fn(Id, Sel, CGFloat) -> (),
            self.spacing
        );

        // Distribution: fill proportionally
        let _: () = msg_send!(
            stack,
            sel!("setDistribution:"),
            fn(Id, Sel, NSInteger) -> (),
            1isize // NSStackViewDistributionFillProportionally
        );

        // Add children
        for child in &self.children {
            let child_view = reconciler::build_native(&**child);
            let _: () = msg_send!(
                stack,
                sel!("addArrangedSubview:"),
                fn(Id, Sel, Id) -> (),
                child_view
            );
        }

        // Auto layout
        let _: () = msg_send!(
            stack,
            sel!("setTranslatesAutoresizingMaskIntoConstraints:"),
            fn(Id, Sel, BOOL) -> (),
            no()
        );

        stack
    }
}

// ============================================================================
// VStack — public API
// ============================================================================

pub struct VStack {
    inner: Stack,
}

impl VStack {
    pub fn new(children: Vec<Box<dyn View>>) -> Self {
        Self {
            inner: Stack {
                orientation: Orientation::Vertical,
                spacing: 8.0,
                children,
            },
        }
    }

    pub fn spacing(mut self, s: f64) -> Self {
        self.inner.spacing = s as CGFloat;
        self
    }
}

impl View for VStack {
    unsafe fn build(&self) -> Id {
        self.inner.build()
    }
}

// ============================================================================
// HStack — public API
// ============================================================================

pub struct HStack {
    inner: Stack,
}

impl HStack {
    pub fn new(children: Vec<Box<dyn View>>) -> Self {
        Self {
            inner: Stack {
                orientation: Orientation::Horizontal,
                spacing: 8.0,
                children,
            },
        }
    }

    pub fn spacing(mut self, s: f64) -> Self {
        self.inner.spacing = s as CGFloat;
        self
    }
}

impl View for HStack {
    unsafe fn build(&self) -> Id {
        self.inner.build()
    }
}
