use fltk::app::{self, channel};
use fltk::enums::{Event, Key};
use fltk::{button::*, window::*};

use fltk::prelude::*;

use serious_organizer_lib::lens::Lens;
use serious_organizer_lib::models::Entry;
// use serious_organizer_lib::lens
use parking_lot::Mutex;
use std::{collections::HashSet, sync::Arc};

use crate::label::entry_label_list::EntryLabelList;

// use super::entry_label_list::EntryLabelList;

#[derive(Clone, Debug)]
pub enum LabelMessage {
    LabelListChanged,
    ExitDialog,
}

pub struct EntryLabelDialog {
    lens: Arc<Mutex<Lens>>,
    entry_ids: Arc<Vec<u32>>,
    select_labels: Arc<Mutex<HashSet<u32>>>,
}

impl EntryLabelDialog {
    pub fn new(lens: Arc<Mutex<Lens>>, entries: Vec<Entry>) -> Self {
        let entry_ids: Vec<u32> = entries.iter().map(|e| e.id as u32).collect();

        // Gather all labels that have entries that are selected
        let mut org_labels = HashSet::new();
        {
            let lens_c = lens.lock();
            for entry_id in entry_ids.iter() {
                let lbls = lens_c.entry_labels(*entry_id);
                for lbl_id in lbls {
                    org_labels.insert(lbl_id as u32);
                }
            }
        }

        let select_labels = org_labels.iter().cloned().collect();

        EntryLabelDialog {
            lens,
            entry_ids: Arc::new(entry_ids),
            select_labels: Arc::new(Mutex::new(select_labels)),
        }
    }

    pub fn show(&self) {
        let (sender, reciever) = channel::<LabelMessage>();

        let mut dialog = Window::new(300, 100, 210, 260, "Select Labels");
        dialog.make_modal(true);
        println!("Make label modal");
        let mut but_save = Button::new(10, 10, 60, 25, "Save");
        let mut but_cancel = Button::new(80, 10, 60, 25, "Cancel");

        let lens_c = self.lens.clone();
        let sender_c = sender.clone();
        let mut lbl_table = EntryLabelList::new(
            10,
            50,
            200,
            205,
            lens_c,
            self.select_labels.clone(),
            sender_c,
        );

        // Button save callback
        let entry_ids_c = self.entry_ids.clone();
        let lbl_table_c = lbl_table.clone();
        let lens_c = self.lens.clone();
        let sender_c = sender.clone();
        but_save.set_callback(move |_| {
            let currently_selected_labels = lbl_table_c.selected_label_ids.lock();

            println!("Entries lbls: {:?}", entry_ids_c);
            println!("Sel lbls: {:?}", currently_selected_labels);

            // Get all labels
            let mut lens = lens_c.lock();
            let mut labels = HashSet::new();
            for label in lens.get_labels().iter() {
                labels.insert(label.id as u32);
            }

            let cur_select: Vec<u32> = currently_selected_labels
                .iter()
                .copied()
                .collect::<Vec<u32>>();
            let entries = entry_ids_c.iter().copied().collect();
            lens.add_entry_labels(entries, cur_select);

            #[allow(clippy::map_clone)]
            let to_remove: Vec<u32> = labels
                .difference(&*currently_selected_labels)
                .collect::<HashSet<&u32>>()
                .iter()
                .map(|e| *e)
                .copied()
                .collect::<Vec<u32>>();

            println!("Rem lbls: {:?}", to_remove);

            let entries = entry_ids_c.iter().copied().collect();
            lens.remove_entry_labels(entries, to_remove);

            sender_c.send(LabelMessage::ExitDialog);
        });

        // Button cancel callback
        let sender_c = sender.clone();
        but_cancel.set_callback(move |_| {
            sender_c.send(LabelMessage::ExitDialog);
        });

        dialog.end();
        dialog.show();
        dialog.make_current();

        let sender_c = sender;
        dialog.handle(move |_, evt: Event| {
            if evt.contains(Event::Hide)
                || (evt.contains(Event::Shortcut) && app::event_key() == Key::Escape)
            {
                sender_c.send(LabelMessage::ExitDialog);
                return true;
            }

            false
        });

        let mut dialog_c = dialog.clone();

        while dialog.shown() {
            while fltk::app::wait() {
                if let Some(msg) = reciever.recv() {
                    match msg {
                        LabelMessage::LabelListChanged => lbl_table.redraw(),
                        LabelMessage::ExitDialog => {
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
