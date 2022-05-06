use fltk::frame::Frame;
use fltk::prelude::*;
use fltk::window::*;

pub struct LoadingDialog {
    dialog: Window,
}

impl LoadingDialog {
    pub fn new() -> Self {
        let mut dialog = Window::new(300, 325, 450, 120, "Loading!");
        dialog.make_modal(true);

        let mut output_name = Frame::new(10, 10, 390, 25, None);
        output_name.set_label("Loading...");
        output_name.set_label_size(20);

        dialog.end();

        LoadingDialog { dialog }
    }

    pub fn show(&mut self) {
        self.dialog.show();

        // while dialog.shown() {
        //     let _ = fltk::app::wait();
        // }
    }

    pub fn hide(&mut self) {
        self.dialog.hide();
    }
}
