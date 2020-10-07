use fltk::*;
use fltk::{button::*, input::*, window::*};
use serious_organizer_lib::lens::Lens;
use std::sync::{Arc, RwLock};

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
    lens: Arc<RwLock<Lens>>,
    location: Arc<RwLock<Location>>,
}

impl LocationDialog {
    pub fn new(lens: Arc<RwLock<Lens>>) -> Self {
        LocationDialog {
            lens,
            location: Arc::new(RwLock::new(Location {
                name: None,
                path: None,
            })),
        }
    }

    pub fn show(&self) {
        let mut dialog = Window::new(300, 100, 500, 515, "Location");
        dialog.make_modal(true);

        let lens_c = self.lens.clone();

        let mut input_name = Input::new(60, 10, 80, 25, "Name");
        let mut input_path = Input::new(190, 10, 150, 25, "Path");

        let mut but_save = Button::new(360, 10, 60, 25, "Save");

        let location_c = self.location.clone();
        but_save.set_callback(Box::new(move || {
            let loc = location_c.read().unwrap();
            if let Some((name, path)) = loc.values() {
                let mut lens = lens_c.write().unwrap();
                lens.add_location(&name, &path);
                println!("Add name: {} location: {}", name, path);
            }
        }));
        but_save.deactivate();

        // Name changed
        input_name.set_trigger(CallbackTrigger::Changed);
        let input_c = input_name.clone();
        let location_c = self.location.clone();
        let mut but_c = but_save.clone();
        input_name.set_callback(Box::new(move || {
            let name = input_c.value();
            let mut loc = location_c.write().unwrap();
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
            let mut loc = location_c.write().unwrap();
            loc.set_path(&path);

            if loc.valid() {
                but_c.activate();
            } else {
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
