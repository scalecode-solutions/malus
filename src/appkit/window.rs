//! NSWindow — a window on screen.

use crate::runtime::*;

/// Window style mask bitflags.
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct WindowStyle(pub NSUInteger);

impl WindowStyle {
    pub const BORDERLESS: Self = Self(0);
    pub const TITLED: Self = Self(1 << 0);
    pub const CLOSABLE: Self = Self(1 << 1);
    pub const MINIATURIZABLE: Self = Self(1 << 2);
    pub const RESIZABLE: Self = Self(1 << 3);
    pub const FULL_SCREEN: Self = Self(1 << 14);
}

impl std::ops::BitOr for WindowStyle {
    type Output = Self;
    fn bitor(self, rhs: Self) -> Self {
        Self(self.0 | rhs.0)
    }
}

impl std::ops::BitOrAssign for WindowStyle {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0;
    }
}

impl std::ops::BitAnd for WindowStyle {
    type Output = Self;
    fn bitand(self, rhs: Self) -> Self {
        Self(self.0 & rhs.0)
    }
}

/// A window.
pub struct NSWindow(pub(super) Id);

impl NSWindow {
    /// Return the raw ObjC pointer (does not transfer ownership).
    pub fn as_raw(&self) -> Id {
        self.0
    }

    /// Create a new window.
    ///
    /// Uses `NSBackingStoreBuffered` (2) and `defer = NO`.
    pub fn new(rect: CGRect, style: WindowStyle) -> Self {
        unsafe {
            let obj = alloc(cls!("NSWindow") as Id);
            let obj: Id = msg_send!(
                obj,
                sel!("initWithContentRect:styleMask:backing:defer:"),
                fn(Id, Sel, CGRect, NSUInteger, NSUInteger, BOOL) -> Id,
                rect,
                style.0,
                2 as NSUInteger, // NSBackingStoreBuffered
                no()
            );
            Self(obj)
        }
    }

    /// Set the window title.
    pub fn set_title(&self, title: &str) {
        unsafe {
            let ns = nsstring(title);
            msg_send!(self.0, sel!("setTitle:"), fn(Id, Sel, Id) -> (), ns);
        }
    }

    /// Get the window title.
    pub fn title(&self) -> String {
        unsafe {
            let ns: Id = msg_send!(self.0, sel!("title"), fn(Id, Sel) -> Id);
            from_nsstring(ns)
        }
    }

    /// Set the content view.
    pub fn set_content_view(&self, view: &super::view::NSView) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setContentView:"),
                fn(Id, Sel, Id) -> (),
                view.as_raw()
            );
        }
    }

    /// Get the content view.
    pub fn content_view(&self) -> super::view::NSView {
        unsafe {
            let raw: Id = msg_send!(self.0, sel!("contentView"), fn(Id, Sel) -> Id);
            super::view::NSView(retain(raw))
        }
    }

    /// Make the window key and bring it to front.
    pub fn make_key_and_order_front(&self) {
        unsafe {
            msg_send!(
                self.0,
                sel!("makeKeyAndOrderFront:"),
                fn(Id, Sel, Id) -> (),
                std::ptr::null_mut() as Id
            );
        }
    }

    /// Center the window on screen.
    pub fn center(&self) {
        unsafe {
            msg_send!(self.0, sel!("center"), fn(Id, Sel) -> ());
        }
    }

    /// Close the window.
    pub fn close(&self) {
        unsafe {
            msg_send!(self.0, sel!("close"), fn(Id, Sel) -> ());
        }
    }

    /// Set the window frame, optionally animating.
    pub fn set_frame(&self, rect: CGRect, animate: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setFrame:display:animate:"),
                fn(Id, Sel, CGRect, BOOL, BOOL) -> (),
                rect,
                yes(),
                from_bool(animate)
            );
        }
    }

    /// Get the window frame.
    pub fn frame(&self) -> CGRect {
        unsafe {
            msg_send!(self.0, sel!("frame"), fn(Id, Sel) -> CGRect)
        }
    }

    /// Set the minimum window size.
    pub fn set_min_size(&self, size: CGSize) {
        unsafe {
            msg_send!(self.0, sel!("setMinSize:"), fn(Id, Sel, CGSize) -> (), size);
        }
    }

    /// Set the maximum window size.
    pub fn set_max_size(&self, size: CGSize) {
        unsafe {
            msg_send!(self.0, sel!("setMaxSize:"), fn(Id, Sel, CGSize) -> (), size);
        }
    }

    /// Set the background color (pass a raw NSColor Id).
    pub fn set_background_color(&self, color_id: Id) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setBackgroundColor:"),
                fn(Id, Sel, Id) -> (),
                color_id
            );
        }
    }

    /// Set whether the window is opaque.
    pub fn set_opaque(&self, opaque: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setOpaque:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(opaque)
            );
        }
    }

    /// Set whether the window is movable by its background.
    pub fn set_movable_by_window_background(&self, movable: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setMovableByWindowBackground:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(movable)
            );
        }
    }

    /// Set the title visibility.
    ///
    /// `visible`: true = visible (0), false = hidden (1).
    pub fn set_title_visibility(&self, visible: bool) {
        let val: NSInteger = if visible { 0 } else { 1 };
        unsafe {
            msg_send!(
                self.0,
                sel!("setTitleVisibility:"),
                fn(Id, Sel, NSInteger) -> (),
                val
            );
        }
    }

    /// Set whether the titlebar appears transparent.
    pub fn set_titlebar_appears_transparent(&self, transparent: bool) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setTitlebarAppearsTransparent:"),
                fn(Id, Sel, BOOL) -> (),
                from_bool(transparent)
            );
        }
    }

    /// Set the window's appearance.
    pub fn set_appearance(&self, appearance: &super::appearance::NSAppearance) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setAppearance:"),
                fn(Id, Sel, Id) -> (),
                appearance.as_raw()
            );
        }
    }

    /// Set the toolbar (pass a raw NSToolbar Id).
    pub fn set_toolbar(&self, toolbar_id: Id) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setToolbar:"),
                fn(Id, Sel, Id) -> (),
                toolbar_id
            );
        }
    }

    /// Set the window delegate (pass a raw delegate Id).
    pub fn set_delegate(&self, delegate: Id) {
        unsafe {
            msg_send!(
                self.0,
                sel!("setDelegate:"),
                fn(Id, Sel, Id) -> (),
                delegate
            );
        }
    }

    /// Check whether this is the key window.
    pub fn is_key_window(&self) -> bool {
        unsafe {
            to_bool(msg_send!(self.0, sel!("isKeyWindow"), fn(Id, Sel) -> BOOL))
        }
    }

    /// Order the window to front (without making it key).
    pub fn order_front(&self) {
        unsafe {
            msg_send!(
                self.0,
                sel!("orderFront:"),
                fn(Id, Sel, Id) -> (),
                std::ptr::null_mut() as Id
            );
        }
    }

    /// Order the window out (hide it).
    pub fn order_out(&self) {
        unsafe {
            msg_send!(
                self.0,
                sel!("orderOut:"),
                fn(Id, Sel, Id) -> (),
                std::ptr::null_mut() as Id
            );
        }
    }
}

impl Clone for NSWindow {
    fn clone(&self) -> Self {
        Self(unsafe { retain(self.0) })
    }
}

impl Drop for NSWindow {
    fn drop(&mut self) {
        unsafe { release(self.0) }
    }
}
