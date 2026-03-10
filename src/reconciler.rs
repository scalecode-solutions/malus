//! Reconciler — turns the View trait tree into native NSView instances.
//!
//! For the initial implementation, this is a simple one-shot builder.
//! A future version could diff/reconcile, but for now: build once.

use crate::runtime::Id;
use crate::views::View;

/// Build a native NSView tree from a View trait object.
pub fn build_native(view: &dyn View) -> Id {
    unsafe { view.build() }
}
