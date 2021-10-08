use fltk::table::TableRow;
use log::LevelFilter;
use parking_lot::Mutex;
use simplelog::{CombinedLogger, Config, SimpleLogger};

use std::sync::Arc;

use fltk::{app, app::*, button::*, frame, group, input::*, table::TableContext, window};
use fltk::{enums::*, prelude::*};

use serious_organizer_lib::dir_search;
use serious_organizer_lib::lens::Lens;

#[macro_use]
extern crate log;

mod choice_dialog;
mod entry_context_menu;
mod entry_table;
mod error_dialog;
mod file_context_menu;
mod file_table;
mod label;
mod location;
mod model;
mod rename_dialog;
mod table_utils;

use entry_table::EntryTable;

use entry_context_menu::show_entry_context_menu;
use file_context_menu::show_file_context_menu;
use file_table::FileTable;
use model::message::Message;

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

    let  table_col = group::Flex::default_fill().column();

    let lens_c = lens.clone();

    let mut dir_tbl = EntryTable::new(w_size - label_width - 10, 390, lens_c);

    let mut file_tbl = FileTable::new(w_size - label_width - 10, 260, lens.clone());

    table_col.resizable(&dir_tbl.wid);
    table_col.resizable(& file_tbl.wid);

    table_col.end();

    let sender_c = sender.clone();
    let mut label_list = label_list::LabelList::new(label_width, h_size, lens.clone(), sender_c);

    table_row.set_size(&mut label_list.wid, label_width);

    table_row.resizable(&table_col);
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

    let sender_c = sender.clone();
    let mut last_click_started = false;

    file_tbl.handle(move |file_wid, evt: Event| {
        let btn = app::event_mouse_button();

        if evt == Event::Released && btn == app::MouseButton::Left {
            match file_wid.callback_context() {
                TableContext::ColHeader => {
                    println!("Handle File Got colheader callback");
                    sender_c.send(Message::FileTableSortCol(file_wid.callback_col()));
                    sender_c.send(Message::FileTableInvalidated);

                    return true;
                }
                TableContext::Cell => {
                    println!("Handle File Got cell changed");
                    sender_c.send(Message::FileTableChanged(file_wid.callback_row() as usize));

                    println!("Filetable Click!");
                    if !app::event_clicks() {
                        last_click_started = false
                    }

                    if !last_click_started && app::event_clicks() {
                        last_click_started = true;

                        sender_c.send(Message::FileTableOpen);
                        // sender_c.send(Message::FileTableInvalidated);
                    }

                    sender_c.send(Message::FileTableInvalidated);

                    return true;
                }
                _ => (),
            }
        }

        // Right click
        if evt == Event::Push
            && btn == app::MouseButton::Right
            && file_wid.callback_context() == TableContext::Cell
        {
            println!("File table get selected");

            let selection = get_selected_index(file_wid);
            sender_c.send(Message::FileShowContextMenu(selection));
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
                    sender_c.send(Message::EntryChanged(Some(dir_wid.callback_row() as usize)));
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
    let sender_c = sender.clone();
    input.set_callback(move |input_c: &mut Input| {
        let dir_count;
        {
            let mut lens = lens_c.lock();
            lens.update_search_text(&input_c.value());
            dir_count = lens.get_dir_count();
        }
        sender_c.send(Message::EntryTableInvalidated);
        sender_c.send(Message::FileTableInvalidated);

        println!("Banan editing {} found: {}", input_c.value(), dir_count);
    });

    wind.handle(move |h_wnd, evt: Event| {
        if evt == Event::Activate {
            println!("Wind activate!");
        }

        if evt == Event::Deactivate {
            println!("Wind Deactivate!");
        }

        if evt == Event::Focus {
            h_wnd.redraw();
            println!("*** bgn");
            return true;
        }

        false
    });

    while app.wait() {
        if let Some(msg) = reciever.recv() {
            match msg {
                // Label Table
                Message::LabelTableInvalidated => label_list.update(),

                // Entry Table
                Message::EntryChanged(ix) => file_tbl.set_dir_ix(ix),

                Message::EntryTableInvalidated => {
                    dir_tbl.update();
                    let ix = get_selected_index(&mut dir_tbl);
                    if !ix.is_empty() {
                        sender.send(Message::EntryChanged(Some(ix[0] as usize)));
                    } else {
                        sender.send(Message::EntryChanged(None));
                    }
                }
                Message::EntryTableSortCol(col) => dir_tbl.toggle_sort_column(col),
                Message::EntryShowContextMenu(selection) => {
                    show_entry_context_menu(selection, lens.clone(), sender.clone(), &mut wind)
                }

                // File Table
                Message::FileTableInvalidated => file_tbl.update(),
                Message::FileTableSortCol(col) => file_tbl.toggle_sort_column(col),
                Message::FileShowContextMenu(selection) => {
                    show_file_context_menu(&mut file_tbl, selection, lens.clone(), sender.clone())
                }
                Message::FileTableChanged(ix) => file_tbl.set_file_ix(ix as usize),
                Message::FileTableOpen => {
                    let path = file_tbl.get_selected_file_path();
                    println!("Running file table open {:?}", path);

                    if let Some(path) = path {
                        println!("Open! {}", path);
                        open::that_in_background(path);
                    }
                }
            }
        }
    }
}
