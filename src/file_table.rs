use std::sync::{Arc, RwLock};

use fltk::*;
use fltk::{table::*};

use serious_organizer_lib::lens::Lens;

use crate::table_utils::{draw_data, draw_header};

pub struct FileTable {
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

    pub fn set_dir_ix(&mut self, new_id: usize) {
        println!("Got new file id: {}", new_id);
        let mut dir_id = self.dir_id.write().unwrap();
        println!("Got old file id: {:?}", *dir_id);
        if dir_id.is_none() || dir_id.unwrap() != new_id {
            *dir_id = Some(new_id);
            let len = self.lens.read().unwrap().get_file_count(new_id).unwrap();
            self.wid.set_rows(len as u32);
            self.wid.redraw();
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
