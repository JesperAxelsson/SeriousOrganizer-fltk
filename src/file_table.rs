use parking_lot::Mutex;
use std::sync::atomic::{AtomicIsize, Ordering};
use std::sync::Arc;

use fltk::table::*;
use fltk::{enums::*, prelude::*, *};

use serious_organizer_lib::{
    lens::{Lens, Sort, SortColumn, SortOrder},
    models::File,
};

use crate::table_utils::{draw_data_color, draw_header, get_file_color, pretty_size};

#[derive(Clone)]
pub struct FileTable {
    pub wid: TableRow,
    dir_id: Arc<AtomicIsize>,
    file_id: Arc<AtomicIsize>,
    lens: Arc<Mutex<Lens>>,
    pub files: Arc<Mutex<Option<Vec<File>>>>,
    col_sort: Arc<Mutex<Sort>>,
}

impl FileTable {
    pub fn new(w: i32, h: i32, lens: Arc<Mutex<Lens>>) -> FileTable {
        let headers = vec!["Name".to_string(), "Path".to_string(), "Size".to_string()];
        let mut table = FileTable {
            wid: TableRow::default().with_size(w, h),
            lens: lens,
            dir_id: Arc::new(AtomicIsize::new(-1)),
            file_id: Arc::new(AtomicIsize::new(-1)),
            files: Arc::new(Mutex::new(None)),
            col_sort: Arc::new(Mutex::new(Sort::new(SortColumn::Name, SortOrder::Asc))),
        };

        table.wid.set_row_height_all(20);
        table.wid.set_row_resize(true);

        // Cols
        table.wid.set_cols(headers.len() as i32);
        table.wid.set_col_header(true);
        table.wid.set_col_resize(true);

        table.wid.end();
        table.wid.set_rows(0);

        let table_c = table.clone();
        table
            .wid
            .draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
                TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
                TableContext::ColHeader => draw_header(&headers[col as usize], x, y, w, h),
                TableContext::Cell => {
                    let dir_id = table_c.get_dir_ix();
                    if dir_id.is_some() {
                        // let selected = table_c.row_selected(row);
                        let selected = t.row_selected(row);

                        if let Some(files) = &*table_c.files.lock() {
                            if let Some(file) = files.get(row as usize) {
                                let name = file.name.as_str();
                                let color = get_file_color(name);

                                match col {
                                    0 => draw_data_color(
                                        name,
                                        x,
                                        y,
                                        w,
                                        h,
                                        color,
                                        selected,
                                        Align::Left,
                                    ),
                                    1 => draw_data_color(
                                        &file.path,
                                        x,
                                        y,
                                        w,
                                        h,
                                        color,
                                        selected,
                                        Align::Left,
                                    ),
                                    2 => draw_data_color(
                                        &pretty_size(file.size),
                                        x,
                                        y,
                                        w,
                                        h,
                                        color,
                                        selected,
                                        Align::Right,
                                    ),
                                    _ => (),
                                }
                            }
                        };
                    } 
                }
                _ => (),
            });
        table
    }

    pub fn get_dir_ix(&self) -> Option<usize> {
        let dir_id = self.dir_id.load(Ordering::Relaxed);
        if dir_id >= 0 {
            Some(dir_id as usize)
        } else {
            None
        }
    }

    pub fn set_dir_ix(&mut self, new_id: Option<usize>) {
        {
            if new_id.is_none() {
                self.dir_id.store(-1, Ordering::Relaxed);
                return;
            }

            let new_id = new_id.unwrap();

            let lens = self.lens.lock();

            let new_ix = lens.convert_ix(new_id);
            println!("Got new dir id: {:?}", new_ix);

            let new_id = if new_ix.is_some() {
                new_id as isize
            } else {
                -1
            };

            self.dir_id.store(new_id, Ordering::Relaxed);

            {
                *self.files.lock() = lens.get_dir_files(new_id as usize).cloned();
            }
        }
        self.update();
    }

    pub fn set_file_ix(&mut self, new_id: usize) {
        // let files = self.files.lock();
        if let Some(files) = &*self.files.lock() {
            if new_id >= files.len() {
                println!(
                    "Got unexpected file id! got: {} max: {}",
                    new_id,
                    files.len()
                );
            }
        } else {
            println!("No files found when settings file_ix!");
        }

        // println!("Got new file id: {}", new_id);
        self.file_id.store(new_id as isize, Ordering::Relaxed);
    }

    pub fn get_selected_file_path(&self) -> Option<String> {
        let file_id = self.file_id.load(Ordering::Relaxed);
        let dir_id = self.get_dir_ix();

        let files = &*self.files.lock();

        if files.is_none() || dir_id.is_none() || file_id < 0 {
            return None;
        }

        let file_id = file_id as usize;

        if let Some(ref bl) = files {
            let ff = bl.get(file_id);
            if let Some(file) = ff {
                return Some(file.path.clone());
            }
        }
        None
    }

    fn sort_by_column(&self) {
        {
            if self.files.lock().is_none() {
                return;
            }
        }

        let (column, order) = {
            let sort = self.col_sort.lock();
            (sort.column, sort.order)
        };

        let selector = |ax: &File, bx: &File| {
            let a = ax;
            let b = bx;

            match column {
                SortColumn::Date => a.name.cmp(&b.name),
                SortColumn::Name => a.name.cmp(&b.name),
                SortColumn::Path => a.path.cmp(&b.path),
                SortColumn::Size => a.size.cmp(&b.size),
            }
        };

        (*self.files.lock()).as_mut().unwrap().sort_by(move |a, b| {
            let ordered = selector(a, b);

            match order {
                SortOrder::Asc => ordered,
                SortOrder::Desc => ordered.reverse(),
            }
        });
    }

    pub fn toggle_sort_column(&self, col_id: i32) {
        // println!("Got new file id: {}", new_id);

        // let mut l = self.lens.lock();
        {
            let mut sort = self.col_sort.lock();

            let col = match col_id {
                0 => SortColumn::Name,
                1 => SortColumn::Path,
                2 => SortColumn::Size,
                _ => panic!("Trying to dir sort unknown column"),
            };

            let ord = {
                if sort.column == col && sort.order == SortOrder::Asc {
                    SortOrder::Desc
                } else {
                    SortOrder::Asc
                }
            };

            println!("Sort by {:?} {:?} {:?}", sort, col, ord);

            // l.order_by(col, ord);

            *sort = Sort::new(col, ord);
        }
        self.sort_by_column();
    }

    pub fn update(&mut self) {
        println!("File table update");

        let dir_ix = self.get_dir_ix();

        if let Some(dir_ix) = dir_ix {
            let lens = &*self.lens.lock();
            if let Some(len) = lens.get_file_count(dir_ix as usize) {
                self.sort_by_column();
                self.wid.set_rows(len as i32);
            } else {
                self.wid.set_rows(0);
            }
        } else {
            println!("File table update set row to zero");

            self.wid.set_rows(0);
        }

        self.set_damage(true);
        self.set_damage_type(Damage::all());
        self.set_changed();
        self.redraw();
    }

    pub fn get_files(&self) -> Arc<Mutex<Option<Vec<File>>>> {
        self.files.clone()
    }
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
