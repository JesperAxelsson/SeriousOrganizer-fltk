use fltk::{button::*, window::*};

use fltk::prelude::*;

use serious_organizer_lib::{lens::Lens, models::LabelId};
// use serious_organizer_lib::lens
use parking_lot::Mutex;
use std::{collections::HashSet, sync::Arc};

use crate::label::entry_label_list::EntryLabelList;

// use super::entry_label_list::EntryLabelList;

pub struct EntryLabelDialog {
    lens: Arc<Mutex<Lens>>,
    entry_ids: Arc<Vec<u32>>,
    select_labels: Arc<Mutex<HashSet<u32>>>,
}

impl EntryLabelDialog {
    pub fn new(lens: Arc<Mutex<Lens>>, entry_ids: Vec<u32>) -> Self {
        // Gather all labels that have entries that are selected

        let mut org_labels = HashSet::new();
        {
            let lens_c = lens.lock();
            for entry_id in entry_ids.iter() {
                let lbls = lens_c.entry_labels(*entry_id);
                for LabelId(lbl_id) in lbls {
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
        let mut dialog = Window::new(300, 100, 210, 260, "Select Labels");
        dialog.make_modal(true);
        println!("Make modal");
        let mut but_save = Button::new(10, 10, 60, 25, "Save");
        let mut but_delete = Button::new(80, 10, 60, 25, "Cancel");

        let lens_c = self.lens.clone();
        let lbl_table = EntryLabelList::new(10, 50, 200, 205, lens_c, self.select_labels.clone());

        // Button save callback
        let entry_ids_c = self.entry_ids.clone();
        let lbl_table_c = lbl_table.clone();
        let lens_c = self.lens.clone();
        let mut dialog_c = dialog.clone();
        but_save.set_callback(move |_| {
            let currently_selected_labels = lbl_table_c.selected_label_ids.lock();

            println!("Entries lbls: {:?}", entry_ids_c);
            println!("Sel lbls: {:?}", currently_selected_labels);

            // Get all labels
            let mut lens = lens_c.lock();
            let mut labels = HashSet::new();
            for label in lens.get_labels().iter() {
                let LabelId(label_id) = label.id;
                labels.insert(label_id as u32);
            }

            let cur_select: Vec<u32> = currently_selected_labels
                .iter()
                .copied()
                .collect::<Vec<u32>>(); //Vec::new();
            let entries = entry_ids_c.iter().copied().collect();
            lens.add_entry_labels(entries, cur_select);

            let to_remove: Vec<u32> = labels
                .difference(&*currently_selected_labels)
                .collect::<HashSet<&u32>>()
                .iter()
                .map(|e| *e)
                .copied()
                .collect::<Vec<u32>>(); //Vec::new();

            println!("Rem lbls: {:?}", to_remove);

            let entries = entry_ids_c.iter().copied().collect();
            lens.remove_entry_labels(entries, to_remove);

            dialog_c.hide();
        });
        // but_save.deactivate();

        // Button delete callback
        let mut dialog_c = dialog.clone();
        but_delete.set_callback(move |_| {
            dialog_c.hide();
        });

        // Name changed
        // let label_c = self.label.clone();
        // let mut but_c = but_save.clone();
        // input_name.set_trigger(CallbackTrigger::Changed);
        // input_name.set_callback2(move |input_c: &mut Input| {
        //     let name = input_c.value();
        //     let mut lbl = label_c.lock();
        //     if !name.is_empty() {
        //         (*lbl) = Some(name);
        //         but_c.activate();
        //     } else {
        //         (*lbl) = None;
        //         but_c.deactivate();
        //     }
        // });

        dialog.end();
        dialog.show();
        dialog.make_current();
        
        while dialog.shown() {
            let _ = fltk::app::wait();
        }
    }
}
