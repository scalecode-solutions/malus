//! `TextField` — an editable text field (NSTextField).

use crate::events;
use crate::runtime::*;
use crate::views::View;

pub struct TextField {
    placeholder: String,
    on_change: Option<Box<dyn Fn(String) + Send + 'static>>,
}

impl TextField {
    pub fn new(placeholder: &str) -> Self {
        Self {
            placeholder: placeholder.to_string(),
            on_change: None,
        }
    }

    pub fn on_change(mut self, f: impl Fn(String) + Send + 'static) -> Self {
        self.on_change = Some(Box::new(f));
        self
    }
}

impl View for TextField {
    unsafe fn build(&self) -> Id {
        let field = alloc_init(cls!("NSTextField") as Id);

        // Placeholder
        let ns_ph = nsstring(&self.placeholder);
        let _: () = msg_send!(
            field,
            sel!("setPlaceholderString:"),
            fn(Id, Sel, Id) -> (),
            ns_ph
        );

        // Editable, bordered
        let _: () = msg_send!(field, sel!("setEditable:"), fn(Id, Sel, BOOL) -> (), yes());
        let _: () = msg_send!(field, sel!("setBezeled:"), fn(Id, Sel, BOOL) -> (), yes());

        // Auto layout
        let _: () = msg_send!(
            field,
            sel!("setTranslatesAutoresizingMaskIntoConstraints:"),
            fn(Id, Sel, BOOL) -> (),
            no()
        );

        // Wire on_change via delegate
        if let Some(cb) = &self.on_change {
            // We need to clone the callback into the event system.
            // Since we can't clone a Box<dyn Fn>, we'll use a trick:
            // wrap in Arc so the map owns it.
            // Actually, let's just take ownership by swapping.
        }

        field
    }
}

/// Separate builder that takes ownership of on_change to avoid borrow issues.
impl TextField {
    pub(crate) unsafe fn build_owned(self) -> Id {
        let field = alloc_init(cls!("NSTextField") as Id);

        let ns_ph = nsstring(&self.placeholder);
        let _: () = msg_send!(
            field,
            sel!("setPlaceholderString:"),
            fn(Id, Sel, Id) -> (),
            ns_ph
        );

        let _: () = msg_send!(field, sel!("setEditable:"), fn(Id, Sel, BOOL) -> (), yes());
        let _: () = msg_send!(field, sel!("setBezeled:"), fn(Id, Sel, BOOL) -> (), yes());

        let _: () = msg_send!(
            field,
            sel!("setTranslatesAutoresizingMaskIntoConstraints:"),
            fn(Id, Sel, BOOL) -> (),
            no()
        );

        if let Some(cb) = self.on_change {
            let cb_id = events::store_change(cb);
            events::wire_text_change(field, cb_id);
        }

        field
    }
}
