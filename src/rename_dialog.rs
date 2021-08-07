use fltk::frame::Frame;
use fltk::{button::*, input::*, window::*};
use fltk::{enums::*, prelude::*};
use parking_lot::Mutex;
use serious_organizer_lib::lens::Lens;
use serious_organizer_lib::models::Entry;
use std::path::Path;
use std::sync::Arc;

use crate::error_dialog::ErrorDialog;

pub struct RenameDialog {
    lens: Arc<Mutex<Lens>>,
    label: Arc<Mutex<Option<String>>>,
    entry: Entry,
}

impl RenameDialog {
    pub fn new(lens: Arc<Mutex<Lens>>, entry: Entry) -> Self {
        RenameDialog {
            lens,
            label: Arc::new(Mutex::new(None)),
            entry,
        }
    }

    pub fn show(&self) {
        // let title = format!("Rename Entry: {}", self.entry.path).as_str();
        let mut dialog = Window::new(300, 325, 450, 120, "Rename Entry");
        dialog.make_modal(true);

        let path = Path::new(&self.entry.path);
        let name = path.file_name().unwrap().to_string_lossy();

        let mut output_name = Frame::new(60, 10, 390, 25, None);
        output_name.set_label(&name);
        output_name.set_label_size(10);
        let mut input_name = Input::new(60, 45, 390, 25, "Name");
        input_name.set_value(&name);
        input_name.set_label_size(10);
        input_name.set_text_size(10);
        let mut but_save = Button::new(10, 80, 60, 25, "Save");
        let mut but_delete = Button::new(80, 80, 60, 25, "Cancel");

        // Button save callback
        let lens_c = self.lens.clone();
        let label_c = self.label.clone();
        let entry_c = self.entry.clone();

        let mut dialog_c = dialog.clone();
        but_save.set_callback(move |_| {
            let lbl = label_c.lock();
            if let Some(ref name) = *lbl {
                {
                    println!("Rename from {} to {}", entry_c.name, &name);
                    let mut lens = lens_c.lock();
                    let result = lens.rename_entry(entry_c.clone(), &name);
                    if let Err(err) = result {
                        println!("Error while renaming file: {:?}", err);
                        let err_dialog = ErrorDialog::new(err.to_string());
                        err_dialog.show();
                    }
                }
                dialog_c.hide();
            }
        });
        but_save.deactivate();

        // Button cancel callback
        let mut dialog_c = dialog.clone();
        but_delete.set_callback(move |_| {
            dialog_c.hide();
        });

        // Name changed
        let label_c = self.label.clone();
        let mut but_c = but_save.clone();
        input_name.set_trigger(CallbackTrigger::Changed);
        input_name.set_callback(move |input_c: &mut Input| {
            let name = input_c.value();
            let mut lbl = label_c.lock();
            if !name.is_empty() {
                (*lbl) = Some(name);
                but_c.activate();
            } else {
                (*lbl) = None;
                but_c.deactivate();
            }
        });

        dialog.end();
        dialog.show();

        while dialog.shown() {
            let _ = fltk::app::wait();
        }
    }
}
