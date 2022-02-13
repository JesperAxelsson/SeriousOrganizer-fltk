use fltk::app::{self, channel};
use fltk::enums::{Event, Key};
use fltk::group::Flex;
use fltk::{button::*, dialog, window::*};

use fltk::prelude::*;

use serious_organizer_lib::lens::Lens;
// use serious_organizer_lib::lens
use parking_lot::Mutex;
use serious_organizer_lib::models::LabelAutoFilter;
use std::sync::Arc;

use crate::label_filter::label_filter_edit_dialog::LabelFilterEditDialog;
use crate::label_filter::label_filter_list::LabelFilterList;

#[derive(Clone, Debug)]
pub enum LabelFilterMessage {
    RunFilters,
    ShowNewDialog,
    ShowEditDialog,
    DeleteSelected,
    ListChanged,
    ListSelected(Option<LabelAutoFilter>),
    ExitDialog,
}

/// Shows all current label filters
pub struct LabelFilterDialog {
    lens: Arc<Mutex<Lens>>,
    selected_label_filter: Arc<Mutex<Option<LabelAutoFilter>>>,
}

impl LabelFilterDialog {
    pub fn new(lens: Arc<Mutex<Lens>>) -> Self {
        LabelFilterDialog {
            lens,
            selected_label_filter: Arc::new(Mutex::new(None)),
        }
    }

    pub fn show(&self) {
        let (sender, reciever) = channel::<LabelFilterMessage>();

        let mut dialog = Window::new(300, 100, 310, 460, "Label filters");
        dialog.make_resizable(true);
        dialog.make_modal(true);

        let mut col = Flex::default_fill().column();
        col.set_margin(10);

        let row = Flex::default_fill().row();

        Button::default()
            .with_label("Run")
            .with_size(60, 25)
            .emit(sender.clone(), LabelFilterMessage::RunFilters);
        Button::new(10, 10, 60, 25, "New").emit(sender.clone(), LabelFilterMessage::ShowNewDialog);
        let mut but_edit = Button::new(10, 10, 60, 25, "Edit");
        let mut but_delete = Button::new(80, 10, 80, 25, "Delete");
        Button::new(80, 10, 60, 25, "Exit").emit(sender.clone(), LabelFilterMessage::ExitDialog);

        row.end();
        col.set_size(&row, 25);

        let mut lbl_table =
            LabelFilterList::new(10, 50, 200, 205, self.lens.clone(), sender.clone());

        col.end();

        dialog.end();
        dialog.show();
        dialog.make_current();

        but_edit.deactivate();
        but_delete.deactivate();

        // Button new
        but_edit.emit(sender.clone(), LabelFilterMessage::ShowEditDialog);
        but_delete.emit(sender.clone(), LabelFilterMessage::DeleteSelected);

        let sender_c = sender.clone();
        dialog.handle(move |_, evt: Event| {
            if evt.contains(Event::Shortcut) && app::event_key() == Key::Escape {
                sender_c.send(LabelFilterMessage::ExitDialog);
                println!("Return from dialog");
                return true;
            }

            false
        });

        while dialog.shown() {
            while fltk::app::wait() {
                if let Some(msg) = reciever.recv() {
                    println!("Filter got message {:?}", msg);

                    match msg {
                        LabelFilterMessage::RunFilters => self.run_filters(),
                        LabelFilterMessage::ShowNewDialog => {
                            let lens_c = self.lens.clone();
                            let dialog = LabelFilterEditDialog::new(lens_c.clone());
                            dialog.show_new();
                            println!("Show done");
                            sender.send(LabelFilterMessage::ListChanged);
                            println!("Message sent");
                        }
                        LabelFilterMessage::ShowEditDialog => {
                            if let Some(label_filter) = &*self.selected_label_filter.lock() {
                                let dialog = LabelFilterEditDialog::new(self.lens.clone());
                                dialog.show_edit(label_filter);
                                sender.send(LabelFilterMessage::ListChanged);
                            } else {
                                println!("ShowEditDialog got no label filter selected!");
                            }
                        }
                        LabelFilterMessage::DeleteSelected => {
                            if let Some(label_filter) = &*self.selected_label_filter.lock() {
                                let choice = dialog::choice_default(
                                    &format!("Would you like to delete '{}'", label_filter.name),
                                    "No",
                                    "Yes",
                                    "",
                                );
                                if choice == 1{
                                    let mut lens_c = self.lens.lock();
                                    lens_c.delete_label_filter(label_filter);
                                    
                                    sender.send(LabelFilterMessage::ListSelected(None));
                                    sender.send(LabelFilterMessage::ListChanged);
                                }
                            } else {
                                println!("ShowEditDialog got no label filter selected!");
                            }
                        }
                        LabelFilterMessage::ListChanged => lbl_table.update(),
                        LabelFilterMessage::ListSelected(label_filter) => {
                            if label_filter.is_some() {
                                but_edit.activate();
                                but_delete.activate();
                            } else {
                                but_edit.deactivate();
                                but_delete.deactivate();
                            }
                            println!("Got selected label filter {:?}", label_filter);
                            *self.selected_label_filter.lock() = label_filter;
                        }
                        LabelFilterMessage::ExitDialog => {
                            println!("Filter dialog got exit");
                            dialog.hide();
                            break;
                        }
                    }
                }
            }
        }

        println!("Exit label filter dialog");
    }

    fn run_filters(&self) {
        let mut lens = self.lens.lock();
        let label_filters = lens.get_label_filters();
        let mut labels = Vec::with_capacity(1);

        for filter in label_filters.iter() {
            let matching_entries = lens.get_entries_for_regex(&filter.filter);
            if let Ok(entries) = matching_entries {
                labels.clear();
                labels.push(filter.label_id as u32);
                lens.add_entry_labels(
                    entries.into_iter().map(|e| e as u32).collect(),
                    labels.clone(),
                );
            } else {
                println!("Error, message: invalid regex");
            }
        }
    }
}
