use parking_lot::Mutex;
use std::sync::Arc;

use fltk::table::*;
use fltk::{enums::*, prelude::*, *};

use crate::table_utils::{draw_data, draw_header,  resize_column, ColHeader};

type RowCountCallback = dyn Fn() -> i32 + Send + Sync + 'static;
type RowDataCallback = dyn Fn(i32, i32) -> String + Send + Sync + 'static;

#[derive(Debug, Clone, Copy)]
pub struct Sort {
    pub column: i32,
    pub order: SortOrder,
}

impl Sort {
    pub fn new(column: i32, order: SortOrder) -> Self {
        Sort { column, order }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
#[repr(u32)]
pub enum SortOrder {
    Asc = 0,
    Desc = 1,
}

#[derive(Clone)]
pub struct BaseTable {
    pub wid: TableRow,
    col_sort: Arc<Mutex<Option<Sort>>>,
    row_count_callback: Arc<Mutex<RowCountCallback>>,
    row_data_callback: Arc<Mutex<RowDataCallback>>,
}

impl BaseTable {
    pub fn new<
        F: Fn() -> i32 + 'static + Send + Sync,
        F2: Fn(i32, i32) -> String + 'static + Send + Sync,
    >(
        headers: Vec<ColHeader>,
        row_count_callback: F,
        row_data_callback: F2,
    ) -> BaseTable {
        // pub fn new(w: i32, h: i32, row_count_callback: &'static RowCountCallback) -> BaseTable {


        let mut table = BaseTable {
            wid: TableRow::default(),
            col_sort: Arc::new(Mutex::new(None)),
            row_count_callback: Arc::new(Mutex::new(row_count_callback)),
            row_data_callback: Arc::new(Mutex::new(row_data_callback)),
        };

        table.wid.set_row_height_all(20);
        table.wid.set_row_resize(true);

        // Cols
        table.wid.set_cols(headers.len() as i32);
        table.wid.set_col_header(true);
        table.wid.set_col_resize(true);

        table.wid.end();
        table.wid.set_rows(table.row_count_callback.lock()());

        resize_column(&mut table, &headers);

        // let lens_c = table.lens.clone();
        let mut table_c = table.clone();
        let mut first_run= true;
        table
            .wid
            .draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
                TableContext::StartPage => {
                    draw::set_font(Font::Helvetica, 14);
                
                if first_run {
                    resize_column(&mut table_c, &headers);
                    first_run=false;
                }},

                TableContext::ColHeader => draw_header(&headers[col as usize].label, x, y, w, h),
                TableContext::Cell => {
          
                    let align = headers[col as usize].align;
                    let row_data = table_c.row_data_callback.lock()(row, col);
                    draw_data(&row_data, x, y, w, h, t.row_selected(row), align)
               
                }
                _ => (),
            });
        table
    }

    pub fn update(&mut self) {
        println!("Entry table upate");
        let dir_count = { self.row_count_callback.lock()() };
        self.set_rows(dir_count);
        self.set_damage(true);
        self.set_damage_type(Damage::all());
        self.set_changed();
        self.redraw();
    }

    pub fn toggle_sort_column(&mut self, col_id: i32) {
        {
            let mut sort = self.col_sort.lock();

            let col = match col_id {
                0 => SortColumn::Name,
                1 => SortColumn::Path,
                2 => SortColumn::Size,
                _ => panic!("Trying to dir sort unknown column"),
            };

            let ord = if let Some(s) = &*sort {
                if s.column == col && s.order == SortOrder::Asc {
                    SortOrder::Desc
                } else {
                    SortOrder::Asc
                }
            } else {
                SortOrder::Asc
            };

            println!("Change sort column!");

            // self.lens.lock().order_by(col, ord);

            *sort = Some(Sort::new(col, ord));
        }
        self.update();
    }
}

use std::ops::{Deref, DerefMut};

impl Deref for BaseTable {
    type Target = TableRow;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}

impl DerefMut for BaseTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}
