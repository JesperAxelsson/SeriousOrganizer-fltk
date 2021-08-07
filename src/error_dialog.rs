use fltk::frame::Frame;
use fltk::prelude::*;
use fltk::{button::*, window::*};

pub struct ErrorDialog {
    error_text: String,
}

impl ErrorDialog {
    pub fn new(error_text: String) -> Self {
        ErrorDialog { error_text }
    }

    pub fn show(&self) {
        let mut dialog = Window::new(300, 325, 450, 120, "Error!");
        dialog.make_modal(true);

        let mut output_name = Frame::new(10, 10, 390, 25, None);
        output_name.set_label(&self.error_text);
        output_name.set_label_size(10);

        let mut btn = Button::new(10, 45, 60, 25, "Ok");
        let mut dialog_c = dialog.clone();
        btn.set_callback(move |_| {
            dialog_c.hide();
        });

        dialog.end();
        dialog.show();

        while dialog.shown() {
            let _ = fltk::app::wait();
        }
    }
}
