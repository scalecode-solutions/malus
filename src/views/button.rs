//! `Button` — a clickable button (NSButton).

use crate::events;
use crate::runtime::*;
use crate::views::View;

pub struct Button {
    label: String,
    on_click: Option<Box<dyn Fn() + Send + 'static>>,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            on_click: None,
        }
    }

    pub fn on_click(mut self, f: impl Fn() + Send + 'static) -> Self {
        self.on_click = Some(Box::new(f));
        self
    }
}

impl View for Button {
    unsafe fn build(&self) -> Id {
        // Use buttonWithTitle:target:action: for a standard push button
        let title_ns = nsstring(&self.label);
        let btn: Id = msg_send!(
            cls!("NSButton") as Id,
            sel!("buttonWithTitle:target:action:"),
            fn(Id, Sel, Id, Id, Sel) -> Id,
            title_ns,
            std::ptr::null_mut::<objc_object>(),
            std::ptr::null::<objc_selector>() as Sel
        );

        // Auto layout
        let _: () = msg_send!(
            btn,
            sel!("setTranslatesAutoresizingMaskIntoConstraints:"),
            fn(Id, Sel, BOOL) -> (),
            no()
        );

        // Note: on_click wiring happens in build_owned since we need ownership
        btn
    }
}

impl Button {
    pub(crate) unsafe fn build_owned(self) -> Id {
        let title_ns = nsstring(&self.label);
        let btn: Id = msg_send!(
            cls!("NSButton") as Id,
            sel!("buttonWithTitle:target:action:"),
            fn(Id, Sel, Id, Id, Sel) -> Id,
            title_ns,
            std::ptr::null_mut::<objc_object>(),
            std::ptr::null::<objc_selector>() as Sel
        );

        let _: () = msg_send!(
            btn,
            sel!("setTranslatesAutoresizingMaskIntoConstraints:"),
            fn(Id, Sel, BOOL) -> (),
            no()
        );

        if let Some(cb) = self.on_click {
            let cb_id = events::store_action(cb);
            events::wire_action(btn, cb_id);
        }

        btn
    }
}
