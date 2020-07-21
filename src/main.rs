use std::sync::{Arc, RwLock};

use fltk::*;
use fltk::{app::*, input::*, table::*};

use serious_organizer_lib::dir_search;
use serious_organizer_lib::lens::Lens;

struct MyTable {
    wid: TableRow,
}

impl MyTable {
    pub fn new(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        headers: Vec<String>,
        row_count: u32,
        cell_data: Box<dyn Fn(i32, i32) -> (String, Align)>,
    ) -> MyTable {
        let mut table = MyTable {
            wid: TableRow::new(x, y, w, h, ""),
        };

        table.wid.set_row_height_all(20);
        table.wid.set_row_resize(true);

        // Cols
        table.wid.set_cols(headers.len() as u32);
        table.wid.set_col_header(true);
        table.wid.set_col_resize(true);

        table.wid.end();
        table.wid.set_rows(row_count);

        let mut table_c = table.wid.clone();

        table
            .wid
            .draw_cell(Box::new(move |ctx, row, col, x, y, w, h| match ctx {
                table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
                table::TableContext::ColHeader => draw_header(&headers[col as usize], x, y, w, h),
                // table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h),
                table::TableContext::Cell => {
                    let (data, align) = cell_data(row, col);
                    draw_data(&data, x, y, w, h, table_c.row_selected(row), align)
                }
                _ => (),
            }));
        table
    }

    // pub fn change_rows(&mut self, new_count: u32) {
    //     self.wid.set_rows(new_count);
    // }
}

fn main() {
    println!("Starting");
    let lens = Arc::new(RwLock::new(Lens::new()));
    {
        let mut lens = lens.write().unwrap();
        if lens.get_locations().len() == 0 {
            lens.add_location("TankTemp", "/home/jesper");
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
        file_tbl.set_dir_id(rt as usize);
    }));

    wind.end();
    wind.show();
    app.run().unwrap();
}

fn draw_header(s: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, Color::FrameDefault);
    draw::set_draw_color(Color::Black);
    draw::draw_text2(s, x, y, w, h, Align::Left);
    draw::pop_clip();
}

fn draw_data(s: &str, x: i32, y: i32, w: i32, h: i32, selected: bool, align: Align) {
    draw::push_clip(x, y, w, h);
    if selected {
        draw::set_draw_color(Color::from_u32(0xD3D3D3));
    } else {
        draw::set_draw_color(Color::White);
    }
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(Color::Gray0);
    draw::draw_text2(s, x, y, w, h, align);
    draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}

pub struct Counter {
    pub count: usize,
}

impl Counter {
    pub fn new() -> Self {
        Counter { count: 0 }
    }

    pub fn get_next(&mut self, margin: usize, size: usize) -> i32 {
        let pos = self.count + margin;
        self.count += size;
        return pos as i32;
    }
}

struct FileTable {
    wid: TableRow,
    dir_id: Arc<RwLock<Option<usize>>>,
    lens: Arc<RwLock<Lens>>,
}

// use std::rc::Rc;

impl FileTable {
    pub fn new(x: i32, y: i32, w: i32, h: i32, lens: Arc<RwLock<Lens>>) -> FileTable {
        let headers = vec!["Name".to_string(), "Path".to_string(), "Size".to_string()];
        let mut table = FileTable {
            wid: TableRow::new(x, y, w, h, ""),
            lens: lens,
            dir_id: Arc::new(RwLock::new(None)),
        };

        table.wid.set_row_height_all(20);
        table.wid.set_row_resize(true);

        // Cols
        table.wid.set_cols(headers.len() as u32);
        table.wid.set_col_header(true);
        table.wid.set_col_resize(true);

        table.wid.end();
        table.wid.set_rows(0);

        let mut table_c = table.wid.clone();

        let lens_c = table.lens.clone();
        let dir_id_c = table.dir_id.clone();
        // (*Rc::get_mut(&mut table).unwrap()).wid.draw_cell(Box::new(
        table
            .wid
            .draw_cell(Box::new(move |ctx, row, col, x, y, w, h| match ctx {
                table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
                table::TableContext::ColHeader => draw_header(&headers[col as usize], x, y, w, h),
                // table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h),
                table::TableContext::Cell => {
                    if let Some(dir_id) = *dir_id_c.read().unwrap() {
                        let selected = table_c.row_selected(row);
                        let l = lens_c.read().unwrap();
                        let files = l.get_dir_files(dir_id as usize).unwrap();
                        let file = &files[row as usize];
                        match col {
                            0 => draw_data(&file.name, x, y, w, h, selected, Align::Left),
                            1 => draw_data(&file.path, x, y, w, h, selected, Align::Left),
                            2 => draw_data(
                                &format!("{}", file.size),
                                x,
                                y,
                                w,
                                h,
                                selected,
                                Align::Right,
                            ),
                            _ => (),
                        };
                    } else {
                        ()
                    }
                }
                _ => (),
            }));
        table
    }

    pub fn set_dir_id(&mut self, new_id: usize) {
        println!("Got new file id: {}", new_id);
        let mut dir_id = self.dir_id.write().unwrap();
        println!("Got old file id: {:?}", *dir_id);
        if dir_id.is_none() || dir_id.unwrap() != new_id {
            *dir_id = Some(new_id);
            let len = self.lens.read().unwrap().get_file_count(new_id).unwrap();
            self.wid.set_rows(len as u32);
            // self.wid.redraw();
            println!("Redrawing, len {}", len);
        }
    }

    // pub fn change_rows(&mut self, new_count: u32) {
    //     self.wid.set_rows(new_count);
    // }
}

use std::ops::{Deref, DerefMut};

impl Deref for FileTable {
    type Target = TableRow;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}

impl DerefMut for FileTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}
