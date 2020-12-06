use log::LevelFilter;
use parking_lot::Mutex;
use simplelog::{CombinedLogger, Config, SimpleLogger};
use std::{cell::RefCell, sync::Arc};

use fltk::{app, app::*, button::*, frame, group, input::*, menu::*, window};

use open;

use serious_organizer_lib::dir_search;
use serious_organizer_lib::lens::Lens;

#[macro_use] 
extern crate log;

// mod counter;
// mod layout;

mod add_label_dialog;
mod entry_table;
mod file_table;
mod label_list;
mod location_dialog;
mod location_table;
mod table_utils;

use entry_table::EntryTable;
use file_table::FileTable;

fn main() {
    println!("Starting");
    CombinedLogger::init(
        vec![
            SimpleLogger::new(LevelFilter::Info, Config::default()),
            // WriteLogger::new(LevelFilter::Info, Config::default(), std::fs::File::create("serious_server.log").expect("Failed to init logger")),
        ]
    ).unwrap();

    let lens = Arc::new(Mutex::new(Lens::new()));

    let w_size: i32 = 715;
    let h_size: i32 = 800;

    let mut app = App::default();
    app.set_scheme(app::AppScheme::Gtk);

    let mut wind = window::Window::new(100, 100, w_size, h_size, "Serious Organizer");
    wind.make_resizable(true);

    println!("Setup app widgets");
    let mut hpack = group::Pack::new(5, 5, w_size - 10, h_size - 10, "");

    let mut top_pack = group::Pack::new(5, 5, w_size, 25, "");
    let _spacer = frame::Frame::default().with_size(45, 25);

    let mut input = Input::new(0, 0, 200, 25, "Search");
    let mut but_reload = Button::new(0, 0, 60, 25, "Reload");
    let mut but = Button::new(0, 0, 80, 25, "Locations");

    top_pack.end();
    top_pack.set_spacing(10);
    top_pack.set_type(group::PackType::Horizontal);

    // Setup dir table

    let mut table_row = group::Pack::new(0, 0, w_size, h_size, "");

    let mut table_col = group::Pack::new(0, 0, w_size - 180, h_size, "");

    let lens_c = lens.clone();
    let mut _spacer = frame::Frame::default().with_size(1, 1);

    let mut dir_tbl = EntryTable::new(5, 5, w_size - 180, 390, lens_c);

    let mut _spacer = frame::Frame::default().with_size(1, 1);

    let mut file_tbl = FileTable::new(5, 5, w_size - 180, 260, lens.clone());

    table_col.resizable(&mut dir_tbl.wid);
    table_col.resizable(&mut file_tbl.wid);
    // table_col.resizable(&mut _spacer);

    table_col.end();
    table_col.set_spacing(5);
    table_col.set_type(group::PackType::Vertical);

    use ::std::rc::Rc;
    let mut dir_tbl_c = dir_tbl.clone();
    let _label_lst = label_list::LabelList::new(
        5,
        5,
        165,
        h_size,
        lens.clone(),
        Rc::new(RefCell::new(move || dir_tbl_c.update())),
    );

    table_row.resizable(&mut table_col);
    table_row.end();
    table_row.set_spacing(10);
    table_row.set_type(group::PackType::Horizontal);
    table_row.auto_layout();

    hpack.resizable(&mut table_row);

    let lens_c = lens.clone();

    but_reload.set_callback(move || {
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

    but.set_callback(move || {
        // println!("Hello World!");
        let dialog = location_dialog::LocationDialog::new(lens_c.clone());
        dialog.show();
    });

    // Setup file table
    let file_tbl_c = file_tbl.clone();
    let lens_c = lens.clone();
    let mut last_click_started = false;
    file_tbl.handle(move |evt: Event| {
        let btn = app::event_button();
        // Left click
        if evt == Event::Push && btn == 1 {
            if !app::event_clicks() {
                last_click_started = false
            }

            let path = file_tbl_c.get_selected_file_path();
            if !last_click_started && app::event_clicks() && path.is_some() {
                last_click_started = true;
                open::that_in_background(path.unwrap());
            }
        }

        // Right click
        if evt == Event::Push && btn == 3 {
            // println!("FL: {:?}, {:?}, {:?}", evt, app::event_clicks(), last_click_started);
            let path = file_tbl_c.get_selected_file_path();
            // println!("Event: {:?}, {:?}, {:?}", evt, app::event_clicks(), path);
            if path.is_some() {
                println!("Context menu!");

                let v = vec!["1st val", "2nd val", "3rd val"];
                let mut x = MenuItem::new(&v);
                match x.popup(app::event_x(), app::event_y()) {
                    None => println!("No value was chosen!"),
                    Some(val) => {
                        println!("{}", val.label().unwrap());
                        if val.label().unwrap() == "1st val" {
                            let dialog = add_label_dialog::AddLabelDialog::new(lens_c.clone());
                            dialog.show();
                        }

                        if val.label().unwrap() == "2st val" {
                            // let lens = lens_c.lock();
                            // lens.add_label
                        }
                    }
                }
            }
        }
        false
    });

    // Setup Entry table

    let mut dir_tbl_c = dir_tbl.clone();
    let lens_c = lens.clone();
    dir_tbl.handle(move |evt: Event| {
        let btn = app::event_button();

        // Right click
        if evt == Event::Push && btn == 3 {
            let selection = dir_tbl_c.get_selected_index();

            if selection.len() > 0 {
                println!("Context menu!");

                let v = vec!["1st val", "2nd val", "3rd val"];
                let mut x = MenuItem::new(&v);
                match x.popup(app::event_x(), app::event_y()) {
                    None => println!("No value was chosen!"),
                    Some(val) => {
                        println!("{}", val.label().unwrap());
                        if val.label().unwrap() == "1st val" {
                            let dialog = add_label_dialog::AddLabelDialog::new(lens_c.clone());
                            dialog.show();
                        }

                        if val.label().unwrap() == "2nd val" {
                            let mut lens = lens_c.lock();
                            let mut entries = Vec::new();

                            for ix in selection.iter() {
                                if let Some(id) = lens.convert_ix(*ix as usize) {
                                    entries.push(id as u32);
                                }
                            }

                            lens.add_entry_labels(entries, vec![1, 2])
                        }

                        if val.label().unwrap() == "3rd val" {
                            let mut lens = lens_c.lock();
                            let mut entries = Vec::new();

                            for ix in selection.iter() {
                                if let Some(e) = lens.get_dir_entry(*ix as usize) {
                                    let id: i32 = e.id.into();
                                    entries.push(id as u32);
                                }
                            }

                            lens.remove_entry_labels(entries, vec![2])
                        }
                    }
                }
            }
        }
        false
    });

    // Setup search input

    input.set_trigger(CallbackTrigger::Changed);
    let lens_c = lens.clone();
    let mut dir_tbl_c = dir_tbl.wid.clone();
    input.set_callback2(move |input_c: &mut Input| {
        let dir_count;
        {
            let mut lens = lens_c.lock();
            lens.update_search_text(&input_c.value());
            dir_count = lens.get_dir_count();
        }
        dir_tbl_c.set_rows(dir_count as u32);
        println!("Banan editing {} found: {}", input_c.value(), dir_count);
    });

    let dir_tbl_c = dir_tbl.wid.clone();
    let mut file_tbl_c = file_tbl.clone();
    dir_tbl.wid.set_trigger(CallbackTrigger::Changed);
    dir_tbl.wid.set_callback(move || {
        let mut cl = 0;
        let mut rt = 0;
        let mut rb = 0;
        let mut cr = 0;
        dir_tbl_c.get_selection(&mut rt, &mut cl, &mut rb, &mut cr);
        println!("Things changed!, {} {}", rt, rb);

        if rt >= 0 {
            file_tbl_c.set_dir_ix(rt as usize);
        }
    });

    let mut file_tbl_c = file_tbl.clone();
    file_tbl.set_trigger(CallbackTrigger::Changed);
    file_tbl.set_callback(move || {
        let mut cl = 0;
        let mut rt = 0;
        let mut rb = 0;
        let mut cr = 0;
        file_tbl_c.get_selection(&mut rt, &mut cl, &mut rb, &mut cr);
        // println!("Files changed!, {} {}", rt, rb);

        file_tbl_c.set_file_ix(rt as usize);
    });

    hpack.end();
    hpack.set_spacing(10);
    hpack.set_type(group::PackType::Vertical);

    wind.end();
    wind.show();
    app.run().unwrap();
}
