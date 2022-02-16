use fltk::enums::Align;
use parking_lot::Mutex;
use std::sync::Arc;

use fltk::table::*;

use serious_organizer_lib::lens::{Lens, Sort, SortColumn, SortOrder};

use crate::base_table::BaseTable;
use crate::table_utils::{draw_data, draw_header, pretty_size, resize_column, ColHeader, ColSize};

#[derive(Clone)]
pub struct EntryTable {
    pub wid: BaseTable,
    lens: Arc<Mutex<Lens>>,
    col_sort: Arc<Mutex<Option<Sort>>>,
}

impl EntryTable {
    pub fn new(  lens: Arc<Mutex<Lens>>) -> EntryTable {
           let headers = vec![
            ColHeader::new("Name", ColSize::Ratio(0.7)),
            ColHeader::new("Path", ColSize::Greedy),
            ColHeader::new("Size", ColSize::Fixed(80)).with_align(Align::Right),
        ];

        let lens_c = lens.clone();
        let lens_c2 = lens.clone();
        let mut new_tbl = BaseTable::new(
            headers,
            move || lens_c.lock().get_dir_count() as i32,
            move |row, col| {
                let l = lens_c2.lock();
                if let Some(dir) = l.get_dir_entry(row as usize) {
                    let data = {
                        match col {
                            0 => dir.name.to_string(),
                            1 => dir.path.to_string(),
                            2 => pretty_size(dir.size),
                            _ => "".to_string(),
                        }
                    };
                    return data;
                }

                return "".to_string();
            },
        );

        let table = EntryTable {
            wid: new_tbl,
            lens,
            col_sort: Arc::new(Mutex::new(None)),
        };

        table
    }

    // pub fn update(&mut self) {
    //     println!("Entry table upate");
    //     let dir_count = { self.lens.lock().get_dir_count() as i32 };
    //     self.set_rows(dir_count);
    //     self.set_damage(true);
    //     self.set_damage_type(Damage::all());
    //     self.set_changed();
    //     self.redraw();
    // }

    // pub fn toggle_sort_column(&mut self, col_id: i32) {
    //     {
    //         let mut sort = self.col_sort.lock();

    //         let col = match col_id {
    //             0 => SortColumn::Name,
    //             1 => SortColumn::Path,
    //             2 => SortColumn::Size,
    //             _ => panic!("Trying to dir sort unknown column"),
    //         };

    //         let ord = if let Some(s) = &*sort {
    //             if s.column == col && s.order == SortOrder::Asc {
    //                 SortOrder::Desc
    //             } else {
    //                 SortOrder::Asc
    //             }
    //         } else {
    //             SortOrder::Asc
    //         };

    //         println!("Change sort column!");

    //         self.lens.lock().order_by(col, ord);

    //         *sort = Some(Sort::new(col, ord));
    //     }
    //     self.update();
    // }
}

use std::ops::{Deref, DerefMut};

impl Deref for EntryTable {
    type Target = TableRow;

    fn deref(&self) -> &Self::Target {
        &self.wid.wid
    }
}

impl DerefMut for EntryTable {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid.wid
    }
}
