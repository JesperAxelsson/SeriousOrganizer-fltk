use std::sync::{Arc, RwLock};

use fltk::*;
use fltk::{app::*, input::*};

use serious_organizer_lib::dir_search;
use serious_organizer_lib::lens::Lens;

mod counter;
mod table_utils;
mod file_table;
mod entry_table;


use counter::Counter;
use file_table::FileTable;
use entry_table::MyTable;

fn main() {
    println!("Starting");
    let lens = Arc::new(RwLock::new(Lens::new()));
    {
        let mut lens = lens.write().unwrap();
        if lens.get_locations().len() == 0 {
            lens.add_location("TankTemp", "/home/jesper/Documents");
        }
        let paths = lens
            .get_locations()
            .iter()
            .map(|e| (e.id, e.path.clone()))
            .collect();
        let mut dir_s = dir_search::get_all_data(paths);

        println!("Update data");
        lens.update_data(&mut dir_s);
    }

    let mut h_count = Counter::new();

    let mut app = App::default();
    app.set_scheme(app::AppScheme::Gtk);
    let mut wind = window::Window::new(100, 100, 800, 700, "Table");

    println!("Setup app widgets");
    let mut input = Input::new(60, h_count.get_next(5, 30), 200, 30, "Search");

    // Setup dir table
    let lens_c = lens.clone();

    let mut dir_tbl = MyTable::new(
        5,
        h_count.get_next(5, 390),
        790,
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

    // Setup file table
    let mut file_tbl = FileTable::new(5, h_count.get_next(5, 390), 790, 390, lens.clone());

    file_tbl.handle(Box::new(move |evt: Event| {
        if evt == Event::Push {
            println!("Event: {:?}, {:?}", evt, app::event_clicks());
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
    // let file_tbl_c = file_tbl.clone();
    dir_tbl.wid.set_trigger(CallbackTrigger::Changed);
    dir_tbl.wid.set_callback(Box::new(move || {
        let mut cl = 0;
        let mut rt = 0;
        let mut rb = 0;
        let mut cr = 0;
        dir_tbl_c.get_selection(&mut rt, &mut cl, &mut rb, &mut cr);
        println!("Things changed!, {} {}", rt, rb);

        file_tbl.set_dir_ix(rt as usize);
    }));

    wind.end();
    wind.show();
    app.run().unwrap();
}

