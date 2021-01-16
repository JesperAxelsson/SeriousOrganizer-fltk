use fltk::table::*;
use fltk::*;

use crate::table_utils::{draw_data, draw_header};

pub struct LocationTable {
    pub wid: TableRow,
}

impl LocationTable {
    pub fn new(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        headers: Vec<String>,
        row_count: u32,
        cell_data: Box<dyn Fn(i32, i32) -> (String, Align)>,
    ) -> LocationTable {
        let mut table = LocationTable {
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

        table.wid.draw_cell2(
            move |table_c: &mut TableRow, ctx, row, col, x, y, w, h| match ctx {
                table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
                table::TableContext::ColHeader => draw_header(&headers[col as usize], x, y, w, h),
                // table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h),
                table::TableContext::Cell => {
                    let (data, align) = cell_data(row, col);
                    draw_data(&data, x, y, w, h, table_c.row_selected(row), align)
                }
                _ => (),
            },
        );

        table
    }

    // pub fn change_rows(&mut self, new_count: u32) {
    //     self.wid.set_rows(new_count);
    // }
}

use std::ops::{Deref, DerefMut};

impl Deref for LocationTable {
    type Target = TableRow;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}

impl DerefMut for LocationTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}
