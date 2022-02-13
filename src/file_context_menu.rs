use parking_lot::Mutex;
use serious_organizer_lib::models::File;

use std::sync::Arc;

use fltk::{app, app::*, menu::*};

use serious_organizer_lib::lens::Lens;

use crate::file_table::FileTable;
use crate::model::message::Message;

use crate::choice_dialog::ChoiceDialog;
// use crate::rename_dialog::RenameDialog;

pub fn show_file_context_menu(
    file_table: &mut FileTable,
    selection: Vec<u32>,
    lens: Arc<Mutex<Lens>>,
    sender: Sender<Message>,
) {
    if !selection.is_empty() {
        println!("Context menu!");

        let choices = vec![
            "Delete File",
            // "Rename Entry",
        ];

        let x = MenuItem::new(&choices);

        let files = if let Some(file) = file_table.get_files().lock().clone() {
            file.iter()
                .enumerate()
                .filter_map(|(ix, f)| {
                    if selection.binary_search(&(ix as u32)).is_ok() {
                        Some(f)
                    } else {
                        None
                    }
                })
                .cloned()
                .collect()
        } else {
            return;
        };

        // let x = MenuItem::new(&v);
        match x.popup(app::event_x(), app::event_y()) {
            None => println!("No value was chosen!"),
            Some(val) => {
                println!("{}", val.label().unwrap());

                match val.label().unwrap().as_str() {
                    "Delete File" => {
                        delete_files(files, lens);
                        sender.send(Message::FileTableInvalidated);
                    }
                    // "Rename Entry" => {
                    //     let dialog = RenameDialog::new(lens.clone(), entry);
                    //     dialog.show();
                    //     sender.send(Message::EntryTableInvalidated);
                    // }
                    _ => {
                        println!("Unknown popup string: {}", val.label().unwrap())
                    }
                }
            }
        }
    }
}

fn delete_files(files: Vec<File>, lens: Arc<Mutex<Lens>>) {
    if !show_delete_confirmation_dialog(files.len()) {
        return;
    }

    println!("Delete the file {:?}", files);

    let mut lens = lens.lock();

    for file in files.iter() {
        lens.remove_file(file)
            .unwrap_or_else(|_| panic!("Failed to remove entry {}", file.name));
    }
}

fn show_delete_confirmation_dialog(count: usize) -> bool {
    let dialog = ChoiceDialog::new(
        format!("Are you sure you want to remove {} files?", count),
        vec!["Ok".to_string(), "Cancel".to_string()],
    );
    dialog.show();

    dialog.result() == 0
}
