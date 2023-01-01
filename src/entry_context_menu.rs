use fltk::window::Window;
use parking_lot::Mutex;
use serious_organizer_lib::models::Entry;

use std::fs::metadata;
use std::sync::Arc;

use fltk::prelude::*;
use fltk::{app, app::*, menu::*};

use serious_organizer_lib::lens::Lens;

use crate::model::message::Message;

use crate::choice_dialog::ChoiceDialog;
use crate::error_dialog::ErrorDialog;
use crate::label::entry_label_dialog;
use crate::rename_dialog::RenameDialog;

pub fn show_entry_context_menu(
    selection: Vec<u32>,
    lens: Arc<Mutex<Lens>>,
    sender: Sender<Message>,
    wind: &mut Window,
) {
    if !selection.is_empty() {
        println!("Context menu!");

        let entry_ix = selection.get(0).unwrap();
        let entry = {
            lens.lock()
                .get_dir_entry(*entry_ix as usize)
                .unwrap()
                .clone()
        };

        let mut entries = Vec::new();
        {
            let lens = lens.lock();
            // Get selected entries
            for ix in selection.iter() {
                if let Some(dir_entry) = lens.get_dir_entry(*ix as usize) {
                    entries.push(dir_entry.clone());
                }
            }
        }

        let has_file = entries.iter().any(|e| {
            let meta = if let Ok(meta) = metadata(&e.path) {
                meta
            } else {
                println!("Failed to find meta data for entry! path: {}", e.path);
                let err_dialog = ErrorDialog::new(format!(
                    "Failed to find meta data for entry! path: {}",
                    e.path
                ));
                err_dialog.show();
                return false;
            };

            meta.is_file()
        });

        let choices = if has_file {
            vec!["Label >", "Delete Entry", "Rename Entry", "Grade", "Move to Dir"]
        } else {
            vec!["Label >", "Delete Entry", "Rename Entry", "Grade", "Open dir"]
        };

        let x = MenuItem::new(&choices);

        // let x = MenuItem::new(&v);
        match x.popup(app::event_x(), app::event_y()) {
            None => println!("No value was chosen!"),
            Some(val) => {
                println!("{}", val.label().unwrap());

                match val.label().unwrap().as_str() {
                    "Label >" => {
                        println!("Got entries: {:?}", entries);

                        // Label select dialog
                        let dialog = entry_label_dialog::EntryLabelDialog::new(lens, entries);

                        wind.deactivate();
                        dialog.show();
                        wind.activate();
                        sender.send(Message::EntryTableInvalidated);
                    }
                    "Delete Entry" => {
                        delete_entry(entries, lens);
                        sender.send(Message::EntryTableInvalidated);
                    }
                    "Rename Entry" => {
                        let dialog = RenameDialog::new(lens, entry);
                        dialog.show();
                        sender.send(Message::EntryTableInvalidated);
                    }
                    "Move to Dir" => {
                        let dialog = ChoiceDialog::new(
                            "Move selected files to a dir?".to_string(),
                            vec!["Yes".to_string(), "No".to_string()],
                        );

                        dialog.show();
                        if dialog.result() == 0 {
                            {
                                let mut lens = lens.lock();
                                for e in entries.iter() {
                                    if let Ok(meta) = metadata(&e.path) {
                                        if meta.is_file() {
                                            let result = lens.move_file_entry_to_dir_entry(e);
                                            if let Err(err) = result {
                                                println!("Error while renaming file: {:?}", err);
                                                let err_dialog = ErrorDialog::new(err.to_string());
                                                err_dialog.show();
                                            }
                                        }
                                    }
                                }
                            }
                            sender.send(Message::EntryTableInvalidated);
                        } else {
                            println!("Abort dir move");
                        }
                    }
                    "Open dir" => {
                        if let Err(e) = open::that(&entry.path) {
                            eprintln !("Failed to open path: {} Error {}", entry.path, e);
                        }
                    }
                    "Grade" => {
                     // Show grade choices
                    }
                    _ => {
                        println!("Unknown popup string: {}", val.label().unwrap())
                    }
                }
            }
        }
    }
}

fn delete_entry(entries: Vec<Entry>, lens: Arc<Mutex<Lens>>) {
    if !show_delete_confirmation_dialog(entries.len()) {
        return;
    }

    println!("Delete the entry {:?}", entries);

    let mut lens = lens.lock();

    for entry in entries.iter() {
        lens.remove_entry(entry)
            .unwrap_or_else(|_| panic!("Failed to remove entry {}", entry.name));
    }
}

fn show_delete_confirmation_dialog(count: usize) -> bool {
    let dialog = ChoiceDialog::new(
        format!("Are you sure you want to remove {} entries?", count),
        vec!["Ok".to_string(), "Cancel".to_string()],
    );
    dialog.show();

    dialog.result() == 0
}
