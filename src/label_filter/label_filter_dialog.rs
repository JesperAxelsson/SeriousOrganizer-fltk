use fltk::app::{self, channel};
use fltk::enums::{Event, Key};
use fltk::group::Flex;
use fltk::{button::*, window::*};

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

        let mut row = Flex::default_fill().row();

        Button::default()
            .with_label("Run")
            .with_size(60, 25)
            .emit(sender.clone(), LabelFilterMessage::RunFilters);
        Button::new(10, 10, 60, 25, "New").emit(sender.clone(), LabelFilterMessage::ShowNewDialog);
        let mut but_edit = Button::new(10, 10, 60, 25, "Edit");
        let mut but_delete = Button::new(80, 10, 80, 25, "Delete");
        Button::new(80, 10, 60, 25, "Exit").emit(sender.clone(), LabelFilterMessage::ExitDialog);

        row.end();
        col.set_size(&mut row, 25);

        let mut lbl_table =
            LabelFilterList::new(10, 50, 200, 205, self.lens.clone(), sender.clone());

        col.end();

        but_edit.deactivate();
        but_delete.deactivate();

        // Button new
        but_edit.emit(sender.clone(), LabelFilterMessage::ShowEditDialog);
        but_delete.emit(sender.clone(), LabelFilterMessage::DeleteSelected);

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
                        LabelFilterMessage::RunFilters => self.run_filters(),
                        LabelFilterMessage::ShowNewDialog => {
                            let lens_c = self.lens.clone();
                            let dialog = LabelFilterEditDialog::new(lens_c.clone());
                            dialog.show_new();
                            sender.send(LabelFilterMessage::ListChanged);
                        }
                        LabelFilterMessage::ShowEditDialog => {
                            if let Some(label_filter) = &*self.selected_label_filter.lock() {
                                let dialog = LabelFilterEditDialog::new(self.lens.clone());
                                dialog.show_edit(&label_filter);
                                sender.send(LabelFilterMessage::ListChanged);
                            } else {
                                println!("ShowEditDialog got no label filter selected!");
                            }
                        }
                        LabelFilterMessage::DeleteSelected => {
                            if let Some(label_filter) = &*self.selected_label_filter.lock() {
                                let mut lens_c = self.lens.lock();
                                lens_c.delete_label_filter(label_filter);
                                sender.send(LabelFilterMessage::ListChanged);
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
                            dialog_c.hide();
                            break;
                        }
                    }
                }
            }
        }

        println!("Exit labellist");
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
