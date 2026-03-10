//! NSApplication + NSWindow bootstrap.
//!
//! `App::new("Title").window(...).run()` → native macOS app with an event loop.

use crate::runtime::*;
use crate::reconciler;
use crate::views::View;

pub struct Window {
    pub title: String,
    pub width: CGFloat,
    pub height: CGFloat,
    pub content: Box<dyn View>,
}

pub struct App {
    title: String,
    windows: Vec<Window>,
}

impl App {
    pub fn new(title: &str) -> Self {
        Self {
            title: title.to_string(),
            windows: Vec::new(),
        }
    }

    /// Add a window with a builder closure that returns a view tree.
    pub fn window<V: View + 'static>(
        mut self,
        title: &str,
        width: f64,
        height: f64,
        content: V,
    ) -> Self {
        self.windows.push(Window {
            title: title.to_string(),
            width: width as CGFloat,
            height: height as CGFloat,
            content: Box::new(content),
        });
        self
    }

    /// Start the macOS event loop. This function never returns.
    pub fn run(self) -> ! {
        unsafe {
            // NSApplication.sharedApplication
            let app: Id = msg_send!(
                cls!("NSApplication") as Id,
                sel!("sharedApplication"),
                fn(Id, Sel) -> Id
            );

            // Set activation policy to Regular (appears in Dock)
            let _: () = msg_send!(
                app,
                sel!("setActivationPolicy:"),
                fn(Id, Sel, NSInteger) -> (),
                0isize // NSApplicationActivationPolicyRegular
            );

            // Create windows
            for win in &self.windows {
                create_window(app, win);
            }

            // Activate and bring to front
            let _: () = msg_send!(
                app,
                sel!("activateIgnoringOtherApps:"),
                fn(Id, Sel, BOOL) -> (),
                yes()
            );

            // Run the event loop — blocks forever
            let _: () = msg_send!(app, sel!("run"), fn(Id, Sel) -> ());
        }

        // unreachable, but makes the return type `!`
        std::process::exit(0);
    }
}

unsafe fn create_window(app: Id, win: &Window) {
    let style_mask: NSUInteger = (1 << 0)  // titled
        | (1 << 1)  // closable
        | (1 << 2)  // miniaturizable
        | (1 << 3); // resizable

    let frame = CGRect::new(200.0, 200.0, win.width, win.height);

    // [[NSWindow alloc] initWithContentRect:styleMask:backing:defer:]
    let ns_window: Id = {
        let raw = alloc(cls!("NSWindow") as Id);
        msg_send!(
            raw,
            sel!("initWithContentRect:styleMask:backing:defer:"),
            fn(Id, Sel, CGRect, NSUInteger, NSUInteger, BOOL) -> Id,
            frame,
            style_mask,
            2usize, // NSBackingStoreBuffered
            no()
        )
    };

    // Set title
    let title_ns = nsstring(&win.title);
    let _: () = msg_send!(
        ns_window,
        sel!("setTitle:"),
        fn(Id, Sel, Id) -> (),
        title_ns
    );

    // Build the view tree into native NSViews
    let content_view = reconciler::build_native(&*win.content);

    // Set the content view
    let _: () = msg_send!(
        ns_window,
        sel!("setContentView:"),
        fn(Id, Sel, Id) -> (),
        content_view
    );

    // Center and show
    let _: () = msg_send!(ns_window, sel!("center"), fn(Id, Sel) -> ());
    let _: () = msg_send!(
        ns_window,
        sel!("makeKeyAndOrderFront:"),
        fn(Id, Sel, Id) -> (),
        std::ptr::null_mut::<objc_object>()
    );
}
