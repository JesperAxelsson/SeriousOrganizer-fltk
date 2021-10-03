use fltk::table::TableRow;
use fltk::window::Window;
use log::LevelFilter;
use parking_lot::Mutex;
use serious_organizer_lib::models::Entry;
use simplelog::{CombinedLogger, Config, SimpleLogger};

use std::fs::metadata;
use std::sync::Arc;

use fltk::{app, app::*, button::*, frame, group, input::*, menu::*, table::TableContext, window};
use fltk::{enums::*, prelude::*};

use open;

use serious_organizer_lib::lens::Lens;
use serious_organizer_lib::{dir_search, models::EntryId};

#[macro_use]
extern crate log;

// mod counter;
// mod layout;

mod choice_dialog;
mod entry_table;
mod error_dialog;
mod file_table;
mod label;
mod location;
mod model;
mod rename_dialog;
mod table_utils;

use choice_dialog::ChoiceDialog;
use entry_table::EntryTable;
use error_dialog::ErrorDialog;
use file_table::FileTable;
use model::message::Message;

use label::add_label_dialog;
use label::entry_label_dialog;
use label::label_list;
use location::location_dialog;
use location::location_table;

pub fn get_selected_index(table: &mut TableRow) -> Vec<u32> {
    let mut selected = Vec::new();

    for ix in 0..table.rows() {
        if table.row_selected(ix as i32) {
            selected.push(ix as u32);
        }
    }
    selected
}

#[cfg(debug_assertions)]
fn get_dir_path() -> String {
    ::std::env::current_exe()
        .unwrap()
        .with_file_name("test.sqlite3")
        .to_string_lossy()
        .to_string()
}

#[cfg(not(debug_assertions))]
fn get_dir_path() -> String {
    use directories::BaseDirs;
    use std::fs::{self, File};

    if let Some(base_dirs) = BaseDirs::new() {
        let dir = base_dirs.data_dir();
        let mut dir = dir.to_path_buf();
        dir.push("SeriousOrganizer");
        println!("Got base dir!");
        fs::create_dir_all(&dir).expect(&format!(
            "Failed to create data dir: {}",
            dir.to_string_lossy()
        ));

        dir.push("Database.sqlite3");

        if !dir.exists() {
            File::create(&dir).expect(&format!(
                "Failed to create db_file: {:?}",
                dir.to_string_lossy()
            ));
        }

        dir.to_string_lossy().to_string()
    } else {
        println!("No base dir! :'(");
        ::std::env::current_exe()
            .unwrap()
            .with_file_name("test.sqlite3")
            .to_string_lossy()
            .to_string()
    }
}

fn main() {
    info!("Starting");
    CombinedLogger::init(vec![
        SimpleLogger::new(LevelFilter::Info, Config::default()),
        // WriteLogger::new(LevelFilter::Info, Config::default(), std::fs::File::create("serious_server.log").expect("Failed to init logger")),
    ])
    .unwrap();

    let db_path = get_dir_path();
    println!("dbpath: {}", db_path);
    let lens = Arc::new(Mutex::new(Lens::new(&db_path)));

    let w_size: i32 = 715;
    let h_size: i32 = 800;

    let mut app = App::default();
    app.set_scheme(app::AppScheme::Base);

    let (sender, reciever) = app::channel::<Message>();

    let mut wind = window::Window::new(100, 100, w_size, h_size, "Serious Organizer");
    wind.make_resizable(true);

    println!("Setup app widgets");

    let mut col = group::Flex::default_fill().column();
    col.set_margin(10);

    let mut top_pack = group::Pack::default().with_size(w_size - 10, 25);

    let _spacer = frame::Frame::default().with_size(45, 25);

    let mut input = Input::default().with_size(200, 25).with_label("Search");
    let mut but_reload = Button::default().with_size(60, 25).with_label("Reload");
    let mut but = Button::default().with_size(80, 25).with_label("Locations");

    top_pack.end();
    top_pack.set_spacing(10);
    top_pack.set_type(group::PackType::Horizontal);
    col.set_size(&mut top_pack, 25);

    // Setup dir table
    let label_width = 195;

    let mut table_row = group::Flex::default_fill().row();

    let mut table_col = group::Flex::default_fill().column();

    let lens_c = lens.clone();

    let mut dir_tbl = EntryTable::new(w_size - label_width - 10, 390, lens_c);

    let mut file_tbl = FileTable::new(w_size - label_width - 10, 260, lens.clone());

    table_col.resizable(&mut dir_tbl.wid);
    table_col.resizable(&mut file_tbl.wid);

    table_col.end();

    let sender_c = sender.clone();
    let mut label_list = label_list::LabelList::new(label_width, h_size, lens.clone(), sender_c);

    table_row.set_size(&mut label_list.wid, label_width);

    table_row.resizable(&mut table_col);
    table_row.end();

    col.end();

    wind.end();
    wind.show();

    // *** End of widget contruction ***

    let lens_c = lens.clone();

    but_reload.set_callback(move |_| {
        let mut lens = lens_c.lock();
        println!("Start update data");

        let paths = lens
            .get_locations()
            .iter()
            .map(|e| (e.id, e.path.clone()))
            .collect();
        let mut dir_s = dir_search::get_all_data(paths);

        lens.update_data(&mut dir_s);
        println!("Done update data");
    });

    let lens_c = lens.clone();

    but.set_callback(move |_| {
        // println!("Hello World!");
        let dialog = location_dialog::LocationDialog::new(lens_c.clone());
        dialog.show();
    });

    // Setup file table
    let file_tbl_c = file_tbl.clone();
    let mut last_click_started = false;
    file_tbl.handle(move |_, evt: Event| {
        if file_tbl_c.callback_context() != TableContext::Cell {
            return false;
        }

        let btn = app::event_mouse_button();
        // Left click
        if evt == Event::Push && btn == app::MouseButton::Left {
            println!("Filetable Click!");
            if !app::event_clicks() {
                last_click_started = false
            }

            let path = file_tbl_c.get_selected_file_path();
            if !last_click_started && app::event_clicks() && path.is_some() {
                println!("Open!");
                last_click_started = true;
                open::that_in_background(path.unwrap());
            }

            return true;
        }

        // Right click
        if evt == Event::Push && btn == app::MouseButton::Right {
            // println!("FL: {:?}, {:?}, {:?}", evt, app::event_clicks(), last_click_started);
            let path = file_tbl_c.get_selected_file_path();
            // println!("Event: {:?}, {:?}, {:?}", evt, app::event_clicks(), path);
            if path.is_some() {
                println!("Context menu!");

                let v = vec!["1st val", "2nd val", "3rd val", "Rename Entry"];
                let x = MenuItem::new(&v);
                match x.popup(app::event_x(), app::event_y()) {
                    None => println!("No value was chosen!"),
                    Some(val) => {
                        println!("{}", val.label().unwrap());
                        if val.label().unwrap() == "1st val" {}

                        if val.label().unwrap() == "2st val" {
                            // let lens = lens_c.lock();
                            // lens.add_label
                        }
                    }
                }
            }

            return true;
        }
        false
    });

    // ** Setup Entry table **

    let sender_c = sender.clone();
    // let lens_c = lens.clone();
    dir_tbl.handle(move |dir_wid, evt: Event| {
        let btn = app::event_mouse_button();

        if evt == Event::Released && btn == app::MouseButton::Left {
            match dir_wid.callback_context() {
                TableContext::ColHeader => {
                    println!("Handle Got colheader callback");
                    sender_c.send(Message::EntryTableSortCol(dir_wid.callback_col()));
                    return true;
                }
                TableContext::Cell => {
                    println!("Handle Got cell changed");
                    sender_c.send(Message::EntryChanged(dir_wid.callback_row() as usize));
                    return true;
                }
                _ => (),
            }
        }

        // Right click
        if evt == Event::Push
            && btn == app::MouseButton::Right
            && dir_wid.callback_context() == TableContext::Cell
        {
            println!("Dir table get selected");

            let selection = get_selected_index(dir_wid);
            sender_c.send(Message::EntryShowContextMenu(selection));
            return true;
        }
        false
    });

    // Setup search input

    input.set_trigger(CallbackTrigger::Changed);
    let lens_c = lens.clone();
    let mut dir_tbl_c = dir_tbl.wid.clone();
    input.set_callback(move |input_c: &mut Input| {
        let dir_count;
        {
            let mut lens = lens_c.lock();
            lens.update_search_text(&input_c.value());
            dir_count = lens.get_dir_count();
        }
        dir_tbl_c.set_rows(dir_count as i32);
        println!("Banan editing {} found: {}", input_c.value(), dir_count);
    });

    let mut file_tbl_c = file_tbl.clone();
    file_tbl.set_trigger(CallbackTrigger::Changed);
    file_tbl.set_callback(move |_| match file_tbl_c.callback_context() {
        TableContext::ColHeader => {
            file_tbl_c.toggle_sort_column(file_tbl_c.callback_col());
        }
        TableContext::Cell => {
            file_tbl_c.set_file_ix(file_tbl_c.callback_row() as usize);
        }
        _ => (),
    });

    // let label_list_c = label_list.clone();
    // let mut dir_tbl_c = dir_tbl.clone();

    wind.handle(move |h_wnd, evt: Event| {
        // if evt == Event::NoEvent {
        //     // println!("Wind NoEvent?!");
        //     return true;
        // }

        if evt == Event::Activate {
            println!("Wind activate!");
            // return true;
        }

        if evt == Event::Deactivate {
            println!("Wind Deactivate!");
            // return true;
        }

        if evt == Event::Focus {
            h_wnd.redraw();
            println!("*** bgn");
            // println!(
            //     "Wind Focus! lbls: ({}, {})",
            //     dir_tbl_c.wid.x(), dir_tbl_c.wid.y()
            // );
            // println!(
            //     "Wind Focus! lbls: ({}, {})",
            //     label_list_c.wid.x(), label_list_c.wid.y()
            // );
            // println!("*** end");
            return true;
        }

        false
    });

    // app.run().unwrap();
    while app.wait() {
        if let Some(msg) = reciever.recv() {
            match msg {
                Message::LabelTableInvalidated => label_list.update(),
                Message::EntryChanged(ix) => file_tbl.set_dir_ix(ix),
                Message::EntryTableInvalidated => dir_tbl.update(),
                Message::EntryTableSortCol(col) => dir_tbl.toggle_sort_column(col),
                Message::EntryShowContextMenu(selection) => {
                    show_entry_context_menu(selection, lens.clone(), sender.clone(), &mut wind)
                }
            }
        }
    }
}

fn show_entry_context_menu(
    selection: Vec<u32>,
    lens: Arc<Mutex<Lens>>,
    sender: Sender<Message>,
    wind: &mut Window,
) {
    if selection.len() > 0 {
        println!("Context menu!");

        let entry_ix = selection.iter().next().unwrap();
        let entry = {
            lens.lock()
                .get_dir_entry(*entry_ix as usize)
                .unwrap()
                .clone()
        };

        let meta = metadata(&entry.path).expect(&format!(
            "Failed to find meta data for entry! path: {}",
            entry.path
        ));

        let choices = if meta.is_file() {
            vec![
                "Add label",
                "Label >",
                "Delete Entry",
                "Rename Entry",
                "Move to Dir",
            ]
        } else {
            vec!["Add label", "Label >", "Delete Entry", "Rename Entry"]
        };

        let x = MenuItem::new(&choices);

        let mut entries = Vec::new();
        {
            let lens = lens.lock();
            // Get selected entries
            for ix in selection.iter() {
                if let Some(dir_entry) = lens.get_dir_entry(*ix as usize) {
                    let EntryId(id) = dir_entry.id;
                    println!("Convert ix {} to {}", ix, id);
                    entries.push(id as u32);
                }
            }
        }

        // let x = MenuItem::new(&v);
        match x.popup(app::event_x(), app::event_y()) {
            None => println!("No value was chosen!"),
            Some(val) => {
                println!("{}", val.label().unwrap());

                match val.label().unwrap().as_str() {
                    "Add label" => {
                        let dialog = add_label_dialog::AddLabelDialog::new(
                            lens.clone(),
                            sender.clone(),
                            // label_update.clone(),
                        );
                        dialog.show();
                    }
                    "Label >" => {
                        println!("Got entries: {:?}", entries);

                        // Label select dialog
                        let dialog = entry_label_dialog::EntryLabelDialog::new(
                            lens.clone(),
                            entries,
                            // label_update.clone(),
                        );

                        wind.deactivate();
                        dialog.show();
                        wind.activate();
                        sender.send(Message::EntryTableInvalidated);
                    }
                    "Delete Entry" => {
                        delete_entry(&entry, lens.clone());
                        sender.send(Message::EntryTableInvalidated);
                    }
                    "Rename Entry" => {
                        let dialog = rename_dialog::RenameDialog::new(lens.clone(), entry);
                        dialog.show();
                        sender.send(Message::EntryTableInvalidated);
                    }
                    "Move to Dir" => {
                        let dialog = ChoiceDialog::new(
                            format!("Move {:?} to a dir?", entry.path),
                            vec!["Yes".to_string(), "No".to_string()],
                        );

                        dialog.show();
                        if dialog.result() == 0 {
                            {
                                let result = lens.lock().move_file_entry_to_dir_entry(entry);
                                if let Err(err) = result {
                                    println!("Error while renaming file: {:?}", err);
                                    let err_dialog = ErrorDialog::new(err.to_string());
                                    err_dialog.show();
                                }
                            }
                            sender.send(Message::EntryTableInvalidated);
                        } else {
                            println!("Abort dir thing");
                        }
                    }
                    _ => {
                        println!("Unknown popup string: {}", val.label().unwrap())
                    }
                }
            }
        }
    }
}

fn delete_entry(entry: &Entry, lens: Arc<Mutex<Lens>>) {
    if !show_delete_confirmation_dialog(entry.name.to_string()) {
        return;
    }

    println!("Delete the entry {}", entry.name);

    let mut lens = lens.lock();
    lens.remove_entry(entry)
        .expect(&format!("Failed to remove entry {}", entry.name));
}

fn show_delete_confirmation_dialog(text: String) -> bool {
    let dialog = choice_dialog::ChoiceDialog::new(
        format!("Are you sure you want to remove {}", text),
        vec!["Ok".to_string(), "Cancel".to_string()],
    );
    dialog.show();

    dialog.result() == 0
}
