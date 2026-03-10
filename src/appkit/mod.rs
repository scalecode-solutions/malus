//! AppKit — native macOS UI framework bindings.
//!
//! Layer 1 typed wrappers around AppKit classes. Every type is a newtype
//! around an ObjC `Id` with retain/release semantics.
//!
//! ```rust,no_run
//! use malus::appkit::*;
//!
//! let app = NSApplication::shared();
//! app.set_activation_policy(ActivationPolicy::Regular);
//!
//! let style = WindowStyle::TITLED | WindowStyle::CLOSABLE | WindowStyle::RESIZABLE;
//! let win = NSWindow::new(malus::core_graphics::CGRect::new(200.0, 200.0, 800.0, 600.0), style);
//! win.set_title("Hello");
//! win.make_key_and_order_front();
//!
//! app.run();
//! ```

// Core
pub mod app;
pub mod window;
pub mod view;
pub mod control;
pub mod responder;
pub mod event;
pub mod screen;
pub mod appearance;
pub mod cursor;

// Controls
pub mod text_field;
pub mod search_field;
pub mod text_view;
pub mod button;
pub mod slider;
pub mod popup_button;
pub mod combo_box;
pub mod color_well;
pub mod image_view;
pub mod image;
pub mod progress;
pub mod stepper;

// Container views
pub mod scroll_view;
pub mod split_view;
pub mod stack_view;
pub mod tab_view;
pub mod table_view;
pub mod outline_view;
pub mod collection_view;
pub mod box_;
pub mod visual_effect;
pub mod layout;

// Chrome
pub mod menu;
pub mod toolbar;

// Dialogs
pub mod alert;
pub mod open_panel;
pub mod save_panel;
pub mod font_panel;
pub mod color_panel;

// Services
pub mod pasteboard;
pub mod workspace;
pub mod user_defaults;
pub mod notification;
pub mod animation;

// Re-exports — the most commonly used types
pub use app::{NSApplication, ActivationPolicy};
pub use window::{NSWindow, WindowStyle};
pub use view::NSView;
pub use control::NSControl;
pub use event::{NSEvent, NSEventType, ModifierFlags};
pub use screen::NSScreen;
pub use appearance::NSAppearance;
pub use cursor::NSCursor;
pub use text_field::{NSTextField, NSTextAlignment, NSLineBreakMode};
pub use search_field::NSSearchField;
pub use text_view::NSTextView;
pub use button::{NSButton, NSButtonType, NSBezelStyle, NSControlStateValue};
pub use slider::NSSlider;
pub use popup_button::NSPopUpButton;
pub use combo_box::NSComboBox;
pub use color_well::NSColorWell;
pub use image_view::{NSImageView, NSImageScaling};
pub use image::NSImage;
pub use progress::{NSProgressIndicator, NSProgressIndicatorStyle};
pub use stepper::NSStepper;
pub use scroll_view::NSScrollView;
pub use split_view::NSSplitView;
pub use stack_view::{NSStackView, NSStackViewDistribution, NSEdgeInsets};
pub use tab_view::{NSTabView, NSTabViewItem};
pub use table_view::{NSTableView, NSTableColumn};
pub use outline_view::NSOutlineView;
pub use collection_view::NSCollectionView;
pub use box_::NSBox;
pub use visual_effect::{NSVisualEffectView, NSVisualEffectMaterial};
pub use layout::NSLayoutConstraint;
pub use menu::{NSMenu, NSMenuItem};
pub use toolbar::{NSToolbar, NSToolbarItem};
pub use alert::{NSAlert, NSAlertStyle, NSModalResponse};
pub use open_panel::NSOpenPanel;
pub use save_panel::NSSavePanel;
pub use font_panel::{NSFontPanel, NSFontManager};
pub use color_panel::NSColorPanel;
pub use pasteboard::NSPasteboard;
pub use workspace::NSWorkspace;
pub use user_defaults::NSUserDefaults;
pub use notification::NSNotificationCenter;
pub use animation::NSAnimationContext;
