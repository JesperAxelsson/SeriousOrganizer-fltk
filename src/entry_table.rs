use parking_lot::Mutex;
use std::sync::Arc;

use fltk::table::*;
use fltk::*;

use serious_organizer_lib::lens::Lens;

use crate::table_utils::{draw_data, draw_header};

pub struct EntryTable {
    pub wid: TableRow,
    lens: Arc<Mutex<Lens>>,
}

impl EntryTable {
    pub fn new(x: i32, y: i32, w: i32, h: i32, lens: Arc<Mutex<Lens>>) -> EntryTable {
        let headers = vec!["Name".to_string(), "Path".to_string(), "Size".to_string()];
        let mut table = EntryTable {
            wid: TableRow::new(x, y, w, h, ""),
            lens,
        };

        table.wid.set_row_height_all(20);
        table.wid.set_row_resize(true);

        // Cols
        table.wid.set_cols(headers.len() as u32);
        table.wid.set_col_header(true);
        table.wid.set_col_resize(true);

        table.wid.end();
        table.wid.set_rows(table.lens.lock().get_dir_count() as u32);

        let mut table_c = table.wid.clone();

        let lens_c = table.lens.clone();

        table
            .wid
            .draw_cell(Box::new(move |ctx, row, col, x, y, w, h| match ctx {
                table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
                table::TableContext::ColHeader => draw_header(&headers[col as usize], x, y, w, h),
                table::TableContext::Cell => {
                    let (data, align) = {
                        let l = lens_c.lock();
                        let dir = l.get_dir_entry(row as usize).unwrap();
                        match col {
                            0 => (dir.name.to_string(), Align::Left),
                            1 => (dir.path.to_string(), Align::Left),
                            2 => (dir.size.to_string(), Align::Right),
                            _ => ("".to_string(), Align::Center),
                        }
                    };
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
