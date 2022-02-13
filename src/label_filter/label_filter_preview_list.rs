use fltk::app::Sender;
use parking_lot::Mutex;
use std::sync::Arc;

use fltk::table::*;
use fltk::{enums::*, prelude::*, *};

// use serious_organizer_lib::lens::{Lens, Sort, SortColumn, SortOrder};
use serious_organizer_lib::lens::Lens;

use crate::table_utils::{draw_data, draw_header, pretty_size};

#[derive(Clone)]
pub struct LabelFilterPreviewList {
    pub wid: TableRow,
    pub sender: Sender<LabelFilterEditMessage>,
    entries: Arc<Mutex<Vec<i32>>>,
    // col_sort: Arc<Mutex<Option<Sort>>>, // Do we need sorting here?
}

impl LabelFilterPreviewList {
    pub fn new(
        w: i32,
        h: i32,
        lens: Arc<Mutex<Lens>>,
        sender: Sender<LabelFilterEditMessage>,
    ) -> LabelFilterPreviewList {
        let headers = vec!["Name".to_string(), "Path".to_string(), "Size".to_string()];

        let mut table = LabelFilterPreviewList {
            wid: TableRow::default().with_size(w, h),
            sender,
            entries: Arc::new(Mutex::new(Vec::new())),
            // col_sort: Arc::new(Mutex::new(None)),
        };

        table.wid.set_row_height_all(20);
        table.wid.set_row_resize(true);

        // Cols
        table.wid.set_cols(headers.len() as i32);
        table.wid.set_col_header(true);
        table.wid.set_col_resize(true);

        table.wid.end();
        table.wid.set_rows(table.entries.lock().len() as i32);

        let mut table_c = table.clone();
        table.handle(move |_, evt| table_c.handle_event(evt));

        let table_c = table.clone();

        table
            .wid
            .draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
                TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
                TableContext::ColHeader => draw_header(&headers[col as usize], x, y, w, h),
                TableContext::Cell => {
                    let l = lens.lock();
                    if let Some(entry_id) = table_c.entries.lock().get(row as usize) {
                        if let Some(entry) = l.get_dir_entry_by_id(*entry_id) {
                            let (data, align) = {
                                match col {
                                    0 => (entry.name.to_string(), Align::Left),
                                    1 => (entry.path.to_string(), Align::Left),
                                    2 => (pretty_size(entry.size), Align::Right),
                                    _ => ("".to_string(), Align::Center),
                                }
                            };

                            draw_data(&data, x, y, w, h, t.row_selected(row), align)
                        }
                    }
                }
                _ => (),
            });
        table
    }

    pub fn update(&mut self) {
        println!("Preview table upate");
        let dir_count = { self.entries.lock().len() as i32 };
        self.set_rows(dir_count);
        self.set_damage(true);
        self.set_damage_type(Damage::all());
        self.set_changed();
        self.redraw();
    }

    fn handle_event(&mut self, evt: Event) -> bool {
        let btn = app::event_mouse_button();
        if evt == Event::Released && btn == app::MouseButton::Left {
            // let lbl_ix = self.callback_row() as usize;

            // Left click
            match self.callback_context() {
                TableContext::StartPage => println!("Label StartPage!"),
                TableContext::EndPage => println!("Label EndPage!"),
                TableContext::Cell => {
                    //    self.update();
                    // return true;
                }
                _ => (),
            }
        }

        false
    }

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

    pub fn set_entries(&mut self, entries: Vec<i32>) {
        *self.entries.lock() = entries;
        self.sender.send(LabelFilterEditMessage::ListChanged);
    }
}

use std::ops::{Deref, DerefMut};

use super::label_filter_edit_dialog::LabelFilterEditMessage;

impl Deref for LabelFilterPreviewList {
    type Target = TableRow;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}

impl DerefMut for LabelFilterPreviewList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}
