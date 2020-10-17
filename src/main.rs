use std::sync::{Arc, RwLock};

use fltk::*;
use fltk::{app::*, button::*, input::*};

use open;

use serious_organizer_lib::dir_search;
use serious_organizer_lib::lens::Lens;

// mod counter;
// mod layout;

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
    let lens = Arc::new(RwLock::new(Lens::new()));

    let w_size: i32 = 715;
    let h_size: i32 = 800;

    let mut app = App::default();
    app.set_scheme(app::AppScheme::Gtk);

    let mut wind = window::Window::new(100, 100, w_size, h_size, "Table");
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
    let mut _spacer = frame::Frame::default().with_size(  1, 1);

    let mut dir_tbl = EntryTable::new(
        5,
        5,
        w_size - 180,
        390,
        vec!["Name".to_string(), "Path".to_string(), "Size".to_string()],
        lens.read().unwrap().get_dir_count() as u32,
        Box::new(move |row, col| {
            let l = lens_c.read().unwrap();
            let dir = l.get_dir_entry(row as usize).unwrap();
            match col {
                0 => (dir.name.to_string(), Align::Left),
                1 => (dir.path.to_string(), Align::Left),
                2 => (dir.size.to_string(), Align::Right),
                _ => ("".to_string(), Align::Center),
            }
        }),
    );
  
    let mut _spacer = frame::Frame::default().with_size(  1, 1);

    let mut file_tbl = FileTable::new(5, 5, w_size - 180, 260, lens.clone());

    table_col.resizable(&mut dir_tbl.wid);
    table_col.resizable(&mut file_tbl.wid);
    // table_col.resizable(&mut _spacer);

    table_col.end();
    table_col.set_spacing(5);
    table_col.set_type(group::PackType::Vertical);

    let _label_lst = label_list::LabelList::new(5, 5, 165, h_size, lens.clone());

    table_row.resizable(&mut table_col);
    table_row.end();
    table_row.set_spacing(10);
    table_row.set_type(group::PackType::Horizontal);

    hpack.resizable(&mut table_row);

    let lens_c = lens.clone();

    but_reload.set_callback(Box::new(move || {
        let mut lens = lens_c.write().unwrap();
        println!("Start update data");

        let paths = lens
            .get_locations()
            .iter()
            .map(|e| (e.id, e.path.clone()))
            .collect();
        let mut dir_s = dir_search::get_all_data(paths);

        lens.update_data(&mut dir_s);
        println!("Done update data");
    }));

    let lens_c = lens.clone();

    but.set_callback(Box::new(move || {
        println!("Hello World!");
        let dialog = location_dialog::LocationDialog::new(lens_c.clone());
        dialog.show();
    }));

    // Setup file table
    let file_tbl_c = file_tbl.clone();
    file_tbl.handle(Box::new(move |evt: Event| {
        if evt == Event::Push {
            let path = file_tbl_c.get_selected_file_path();
            println!("Event: {:?}, {:?}, {:?}", evt, app::event_clicks(), path);
            if app::event_clicks() && path.is_some() {
                open::that_in_background(path.unwrap());
            }
        }
        false
    }));

    // Setup search input

    input.set_trigger(CallbackTrigger::Changed);
    let input_c = input.clone();
    let lens_c = lens.clone();
    let mut dir_tbl_c = dir_tbl.wid.clone();
    input.set_callback(Box::new(move || {
        let dir_count;

        {
            lens_c.write().unwrap().update_search_text(&input_c.value());
            dir_count = lens_c.read().unwrap().get_dir_count();
        }
        dir_tbl_c.set_rows(dir_count as u32);
        println!("Banan editing {} found: {}", input_c.value(), dir_count);
    }));

    let dir_tbl_c = dir_tbl.wid.clone();
    let mut file_tbl_c = file_tbl.clone();
    dir_tbl.wid.set_trigger(CallbackTrigger::Changed);
    dir_tbl.wid.set_callback(Box::new(move || {
        let mut cl = 0;
        let mut rt = 0;
        let mut rb = 0;
        let mut cr = 0;
        dir_tbl_c.get_selection(&mut rt, &mut cl, &mut rb, &mut cr);
        println!("Things changed!, {} {}", rt, rb);

        if rt >= 0 {
            file_tbl_c.set_dir_ix(rt as usize);
        }
    }));

    let mut file_tbl_c = file_tbl.clone();
    // let file_tbl_c = file_tbl.clone();
    file_tbl.set_trigger(CallbackTrigger::Changed);
    file_tbl.set_callback(Box::new(move || {
        let mut cl = 0;
        let mut rt = 0;
        let mut rb = 0;
        let mut cr = 0;
        file_tbl_c.get_selection(&mut rt, &mut cl, &mut rb, &mut cr);
        println!("Files changed!, {} {}", rt, rb);

        file_tbl_c.set_file_ix(rt as usize);
    }));

    hpack.end();
    hpack.set_spacing(10);
    hpack.set_type(group::PackType::Vertical);

    wind.end();
    wind.show();
    app.run().unwrap();
}
