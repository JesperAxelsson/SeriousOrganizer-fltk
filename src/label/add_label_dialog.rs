use fltk::app::Sender;
use fltk::{button::*, input::*, window::*};
use fltk::{enums::*, prelude::*};
use serious_organizer_lib::lens::Lens;
// use serious_organizer_lib::lens
use parking_lot::Mutex;
use std::sync::Arc;

use crate::model::message::Message;

pub struct AddLabelDialog {
    lens: Arc<Mutex<Lens>>,
    label: Arc<Mutex<Option<String>>>,
    sender: Sender<Message>,
}

impl AddLabelDialog {
    pub fn new(lens: Arc<Mutex<Lens>>, sender: Sender<Message>) -> Self {
        AddLabelDialog {
            lens,
            label: Arc::new(Mutex::new(None)),
            sender,
        }
    }

    pub fn show(&self) {
        let mut dialog = Window::new(300, 100, 150, 85, "Add Label");
        dialog.make_modal(true);

        let mut input_name = Input::new(60, 10, 80, 25, "Name");
        let mut but_save = Button::new(10, 45, 60, 25, "Save");
        let mut but_delete = Button::new(80, 45, 60, 25, "Cancel");

        // Button save callback
        let lens_c = self.lens.clone();
        let label_c = self.label.clone();
        let sender_c = self.sender.clone();
        // let on_update = self.on_update.clone();
        let mut dialog_c = dialog.clone();
        but_save.set_callback(move |_| {
            let lbl = label_c.lock();
            if let Some(ref name) = *lbl {
                {
                    let mut lens = lens_c.lock();
                    lens.add_label(&name);
                }
                dialog_c.hide();
                sender_c.send(Message::LabelTableInvalidated);
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
