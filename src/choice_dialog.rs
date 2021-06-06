use fltk::frame::Frame;
use fltk::prelude::*;
use fltk::{button::*, window::*};
use parking_lot::Mutex;
use std::sync::Arc;

pub struct ChoiceDialog {
    text: String,
    choice: Arc<Mutex<Option<u32>>>,
    choices: Vec<String>,
}

impl ChoiceDialog {
    pub fn new(text: String, choices: Vec<String>) -> Self {
        ChoiceDialog {
            text,
            choice: Arc::new(Mutex::new(None)),
            choices,
        }
    }

    pub fn result(&self) -> i32 {
        if let Some(res) = *self.choice.lock() {
            res as i32
        } else {
            -1
        }
    }

    pub fn show(&self) {
        // let title = format!("Rename Entry: {}", self.entry.path).as_str();
        let mut dialog = Window::new(300, 325, 450, 120, "Choose");
        dialog.make_modal(true);

        let mut output_name = Frame::new(60, 10, 390, 25, None);
        output_name.set_label(&self.text);
        output_name.set_label_size(10);

        for (ix, ch) in self.choices.iter().enumerate() {
            let x = 10 + (70 * ix);

            let mut btn = Button::new(x as i32, 45, 60, 25, None);
            btn.set_label(ch);

            let mut dialog_c = dialog.clone();
            let choice_c = self.choice.clone();

            btn.set_callback(move |_| {
                let mut choice = choice_c.lock();
                *choice = Some(ix as u32);
                println!("Choice: {:?}", choice);
                dialog_c.hide();
            });
        }

        dialog.end();
        dialog.show();

        while dialog.shown() {
            let _ = fltk::app::wait();
        }
    }
}
