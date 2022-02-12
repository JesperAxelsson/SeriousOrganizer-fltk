use fltk::app::{self, channel};
use fltk::enums::{Event, Key};
use fltk::{button::*, window::*};

use fltk::prelude::*;

use serious_organizer_lib::lens::Lens;
// use serious_organizer_lib::lens
use parking_lot::Mutex;
use std::sync::Arc;

use crate::label_filter::label_filter_list::LabelFilterList;

#[derive(Clone, Debug)]
pub enum LabelFilterMessage {
    ShowNewDialog,
    ShowEditDialog(i32),
    DeleteSelected(i32),
    ListChanged,
    ListSelected(i32),
    ExitDialog,
}

/// Shows all current label filters
pub struct LabelFilterDialog {
    lens: Arc<Mutex<Lens>>,
}

impl LabelFilterDialog {
    pub fn new(lens: Arc<Mutex<Lens>>) -> Self {
        LabelFilterDialog { lens }
    }

    pub fn show(&self) {
        // TODO: Maybe use refactor to use more sender instead.
        let (sender, reciever) = channel::<LabelFilterMessage>();

        let mut dialog = Window::new(300, 100, 310, 460, "Label filters");
        dialog.make_resizable(true);
        dialog.make_modal(true);

        let mut but_new = Button::new(10, 10, 60, 25, " New ");
        let mut but_edit = Button::new(10, 10, 60, 25, " Edit ");
        let mut but_delete = Button::new(80, 10, 60, 25, " Delete ");
        let mut but_exit = Button::new(80, 10, 60, 25, " Exit ");

        let mut lbl_table =
            LabelFilterList::new(10, 50, 200, 205, self.lens.clone(), sender.clone());

        // Button save callback
        // let lbl_table_c = lbl_table.clone();
        // let lens_c = self.lens.clone();
        let sender_c = sender.clone();
        but_new.set_callback(move |_| {
            sender_c.send(LabelFilterMessage::ShowNewDialog);
        });

        // let lens_c = self.lens.clone();
        let sender_c = sender.clone();
        but_edit.set_callback(move |_| {
            let label_auto_id = 1i32;
            sender_c.send(LabelFilterMessage::ShowEditDialog(label_auto_id));
        });

        // Button delete callback
        let sender_c = sender.clone();
        but_delete.set_callback(move |_| {
            // TODO: Delete label filter
            let label_auto_id = 1i32;

            sender_c.send(LabelFilterMessage::DeleteSelected(label_auto_id));
        });

        // Button exit callback
        let sender_c = sender.clone();
        but_exit.set_callback(move |_| {
            sender_c.send(LabelFilterMessage::ExitDialog);
        });

        let sender_c = sender.clone();
        dialog.handle(move |_, evt: Event| {
            if evt.contains(Event::Shortcut) && app::event_key() == Key::Escape {
                sender_c.send(LabelFilterMessage::ExitDialog);
            }

            false
        });

        dialog.end();
        dialog.show();
        dialog.make_current();

        let mut dialog_c = dialog.clone();
        while dialog.shown() {
            while fltk::app::wait() {
                if let Some(msg) = reciever.recv() {
                    match msg {
                        LabelFilterMessage::ShowNewDialog => {
                            println!("ShowNewDialog To be implemented")
                        }
                        LabelFilterMessage::ShowEditDialog(_) => {
                            println!("ShowEditDialog To be implemented")
                        }
                        LabelFilterMessage::DeleteSelected(_) => {
                            println!("DeleteSelected To be implemented")
                        }
                        LabelFilterMessage::ListChanged => lbl_table.redraw(),
                        LabelFilterMessage::ListSelected(_) => {
                            println!("ListSelected To be implemented")
                        }
                        LabelFilterMessage::ExitDialog => {
                            dialog_c.hide();
                            break;
                        }
                    }
                }
            }
        }

        println!("Exit labellist");
    }
}
