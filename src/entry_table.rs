use parking_lot::Mutex;
use std::sync::Arc;

use fltk::table::*;
use fltk::{enums::*, prelude::*, *};

use serious_organizer_lib::lens::{Lens, Sort, SortColumn, SortOrder};

use crate::table_utils::{draw_data, draw_header, pretty_size, resize_column, ColHeader, ColSize, pretty_grade};

#[derive(Clone)]
pub struct EntryTable {
    pub wid: TableRow,
    lens: Arc<Mutex<Lens>>,
    col_sort: Arc<Mutex<Option<Sort>>>,
}

impl EntryTable {
    pub fn new(w: i32, h: i32, lens: Arc<Mutex<Lens>>) -> EntryTable {
        let headers = vec![
            ColHeader::new("Name", ColSize::Ratio(0.7)),
            ColHeader::new("Path", ColSize::Greedy),
            ColHeader::new("Size", ColSize::Fixed(80)),
            ColHeader::new("Grade", ColSize::Fixed(40)),
        ];

        let mut table = EntryTable {
            wid: TableRow::default().with_size(w, h),
            lens,
            col_sort: Arc::new(Mutex::new(None)),
        };

        table.wid.set_row_height_all(20);
        table.wid.set_row_resize(true);

        // Cols
        table.wid.set_cols(headers.len() as i32);
        table.wid.set_col_header(true);
        table.wid.set_col_resize(true);

        table.wid.end();
        table.wid.set_rows(table.lens.lock().get_dir_count() as i32);

        resize_column(&mut table, &headers);

        let lens_c = table.lens.clone();

        table
            .wid
            .draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
                TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
                TableContext::ColHeader => draw_header(&headers[col as usize].label, x, y, w, h),
                TableContext::Cell => {
                    let l = lens_c.lock();
                    if let Some(dir) = l.get_dir_entry(row as usize) {
                        let (data, align) = {
                            match col {
                                0 => (dir.name.to_string(), Align::Left),
                                1 => (dir.path.to_string(), Align::Left),
                                2 => (pretty_size(dir.size), Align::Right),
                                3 => (pretty_grade(dir.grade), Align::Center),
                                _ => ("".to_string(), Align::Center),
                            }
                        };

                        draw_data(&data, x, y, w, h, t.row_selected(row), align)
                    }
                }
                _ => (),
            });
        table
    }

    pub fn update(&mut self) {
        println!("Entry table upate");
        let dir_count = { self.lens.lock().get_dir_count() as i32 };
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
                3 => SortColumn::Grade,
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

            self.lens.lock().order_by(col, ord);

            *sort = Some(Sort::new(col, ord));
        }
        self.update();
    }
}

use std::ops::{Deref, DerefMut};

impl Deref for EntryTable {
    type Target = TableRow;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}

impl DerefMut for EntryTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}
