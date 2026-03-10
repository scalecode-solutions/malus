//! `Text` — a non-editable label (NSTextField configured as a label).

use crate::runtime::*;
use crate::views::View;

pub struct Text {
    content: String,
    font_size: Option<CGFloat>,
}

impl Text {
    pub fn new(s: &str) -> Self {
        Self {
            content: s.to_string(),
            font_size: None,
        }
    }

    pub fn font_size(mut self, size: f64) -> Self {
        self.font_size = Some(size as CGFloat);
        self
    }
}

impl View for Text {
    unsafe fn build(&self) -> Id {
        let field = alloc_init(cls!("NSTextField") as Id);

        // Set string value
        let ns = nsstring(&self.content);
        let _: () = msg_send!(field, sel!("setStringValue:"), fn(Id, Sel, Id) -> (), ns);

        // Make it a label (non-editable, no border, no background)
        let _: () = msg_send!(field, sel!("setEditable:"), fn(Id, Sel, BOOL) -> (), no());
        let _: () = msg_send!(field, sel!("setBezeled:"), fn(Id, Sel, BOOL) -> (), no());
        let _: () = msg_send!(field, sel!("setDrawsBackground:"), fn(Id, Sel, BOOL) -> (), no());
        let _: () = msg_send!(field, sel!("setSelectable:"), fn(Id, Sel, BOOL) -> (), no());

        // Font size
        if let Some(size) = self.font_size {
            let font: Id = msg_send!(
                cls!("NSFont") as Id,
                sel!("systemFontOfSize:"),
                fn(Id, Sel, CGFloat) -> Id,
                size
            );
            let _: () = msg_send!(field, sel!("setFont:"), fn(Id, Sel, Id) -> (), font);
        }

        // Use auto layout
        let _: () = msg_send!(
            field,
            sel!("setTranslatesAutoresizingMaskIntoConstraints:"),
            fn(Id, Sel, BOOL) -> (),
            no()
        );

        field
    }
}
