use fltk::*;
use fltk::{button::*, input::*, window::*};
use serious_organizer_lib::lens::Lens;
// use serious_organizer_lib::lens
use std::sync::Arc;
use parking_lot::Mutex;

use crate::location_table;

pub struct Location {
    name: Option<String>,
    path: Option<String>,
}

impl Location {
    pub fn valid(&self) -> bool {
        if let Some(ref name) = self.name {
            if let Some(ref path) = self.path {
                if !name.trim().is_empty() && !path.trim().is_empty() {
                    return true;
                }
            }
        }

        return false;
    }

    pub fn values(&self) -> Option<(String, String)> {
        if self.valid() {
            if let Some(ref name) = self.name {
                if let Some(ref path) = self.path {
                    return Some((name.clone(), path.clone()));
                }
            }
        }
        None
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = Some(name.to_string())
    }

    pub fn set_path(&mut self, path: &str) {
        self.path = Some(path.to_string())
    }
}

pub struct LocationDialog {
    lens: Arc<Mutex<Lens>>,
    location: Arc<Mutex<Location>>,
    selected_location: Arc<Mutex<Option<usize>>>,
}

impl LocationDialog {
    pub fn new(lens: Arc<Mutex<Lens>>) -> Self {
        LocationDialog {
            lens,
            location: Arc::new(Mutex::new(Location {
                name: None,
                path: None,
            })),
            selected_location: Arc::new(Mutex::new(None)),
        }
    }

    pub fn show(&self) {
        let mut dialog = Window::new(300, 100, 500, 515, "Location");
        dialog.make_modal(true);

        let lens_c = self.lens.clone();

        let mut input_name = Input::new(60, 10, 80, 25, "Name");
        let mut input_path = Input::new(190, 10, 140, 25, "Path");
        let mut but_save = Button::new(350, 10, 60, 25, "Save");
        let mut but_delete = Button::new(420, 10, 60, 25, "Remove");

        let mut location_table = location_table::LocationTable::new(
            5,
            45,
            480,
            390,
            vec!["Name".to_string(), "Path".to_string()],
            self.lens.lock().get_locations().len() as u32,
            Box::new(move |row, col| {
                let l = lens_c.lock();
                let loc_list = l.get_locations();
                if loc_list.len() >= row as usize {
                    let ref loc = loc_list[row as usize];

                    match col {
                        0 => (loc.name.to_string(), Align::Left),
                        1 => (loc.path.to_string(), Align::Left),
                        _ => ("".to_string(), Align::Center),
                    }
                } else {
                    print!("Invalid location row: {}", row);
                    ("".to_string(), Align::Center)
                }
            }),
        );

        // Button save callback
        let location_c = self.location.clone();
        let lens_c = self.lens.clone();
        let mut table_c = location_table.clone();
        but_save.set_callback(Box::new(move || {
            let loc = location_c.lock();
            if let Some((name, path)) = loc.values() {
                let mut lens = lens_c.lock();
                lens.add_location(&name, &path);

                let len = lens.get_locations().len();
                table_c.set_rows(len as u32);
                table_c.redraw();
            }
        }));
        but_save.deactivate();

        // Button delete callback
        let lens_c = self.lens.clone();
        let select_c = self.selected_location.clone();
        let mut table_c = location_table.clone();
        but_delete.set_callback(Box::new(move || {
            let select = *select_c.lock();
            if let Some(loc_ix) = select {
                let mut lens = lens_c.lock();

                let locations = lens.get_locations();
                let ref loc = locations[loc_ix as usize];
                let loc_id: i32 = loc.id.into();

                lens.remove_location(loc_id as u32);

                let len = lens.get_locations().len();
                table_c.set_rows(len as u32);
                table_c.redraw();
            }
        }));
        but_delete.deactivate();

        // Name changed
        input_name.set_trigger(CallbackTrigger::Changed);
        let input_c = input_name.clone();
        let location_c = self.location.clone();
        let mut but_c = but_save.clone();
        input_name.set_callback(Box::new(move || {
            let name = input_c.value();
            let mut loc = location_c.lock();
            loc.set_name(&name);

            if loc.valid() {
                but_c.activate();
            } else {
                but_c.deactivate();
            }
        }));

        // Path changed
        input_path.set_trigger(CallbackTrigger::Changed);
        let input_c = input_path.clone();
        let location_c = self.location.clone();
        let mut but_c = but_save.clone();
        input_path.set_callback(Box::new(move || {
            let path = input_c.value();
            let mut loc = location_c.lock();
            loc.set_path(&path);

            if loc.valid() {
                but_c.activate();
            } else {
                but_c.deactivate();
            }
        }));

        // Location selected
        let table_c = location_table.clone();
        let select_c = self.selected_location.clone();
        let mut but_c = but_delete.clone();
        location_table.wid.set_trigger(CallbackTrigger::Changed);
        location_table.wid.set_callback(Box::new(move || {
            let mut cl = 0;
            let mut rt = 0;
            let mut rb = 0;
            let mut cr = 0;
            table_c.get_selection(&mut rt, &mut cl, &mut rb, &mut cr);
            println!("Select location!, {} {}", rt, rb);
            if rt >= 0 {
                *select_c.lock() = Some(rt as usize);
                but_c.activate();
            } else {
                *select_c.lock() = None;
                but_c.deactivate();
            }
        }));

        dialog.end();
        dialog.show();
        while dialog.shown() {
            let _ = fltk::app::wait();
        }
    }
}
