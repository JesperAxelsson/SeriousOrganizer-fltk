use enums::Font;
use fltk::app::Sender;
use parking_lot::Mutex;
use std::{collections::HashSet, sync::Arc};

use fltk::table::*;
use fltk::{enums::*, prelude::*, *};

use serious_organizer_lib::lens::Lens;

use crate::table_utils::{draw_data, draw_header};

#[derive(Clone)]
pub struct EntryLabelList {
    pub wid: TableRow,
    lens: Arc<Mutex<Lens>>,
    pub selected_label_ids: Arc<Mutex<HashSet<u32>>>,
    pub sender: Sender<LabelMessage>,
}

impl EntryLabelList {
    pub fn new(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        lens: Arc<Mutex<Lens>>,
        selected_label_ids: Arc<Mutex<HashSet<u32>>>,
        sender: Sender<LabelMessage>,
    ) -> EntryLabelList {
        let headers = vec!["Name".to_string(), "State".to_string()];

        let mut table = EntryLabelList {
            wid: TableRow::new(x, y, w, h, ""),
            lens: lens,
            selected_label_ids,
            sender: sender,
        };

        table.set_row_height_all(20);
        table.set_row_resize(true);
        table.set_type(TableRowSelectMode::Single);

        // Cols
        table.set_cols(headers.len() as i32);
        table.set_col_header(true);
        table.set_col_resize(true);

        table.end();

        table.update();

        let lens_c = table.lens.clone();
        let mut table_c = table.clone();
        table.handle(move |_, evt| table_c.handle_event(evt, lens_c.clone()));

        let lens_c = table.lens.clone();
        let selected_label_ids_c = table.selected_label_ids.clone();

        table
            .wid
            .draw_cell(move |_, ctx, row, col, x, y, w, h| match ctx {
                table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
                table::TableContext::ColHeader => draw_header(&headers[col as usize], x, y, w, h),
                table::TableContext::Cell => {
                    let selected = false;

                    let l = lens_c.lock();
                    let label_lst = l.get_labels();

                    if let Some(lbl) = label_lst.get(row as usize) {
                        let sel_lbl = selected_label_ids_c.lock();

                        let lbl_text = if sel_lbl.contains(&(lbl.id as u32)) {
                            "X"
                        } else {
                            ""
                        };

                        match col {
                            0 => draw_data(&lbl.name, x, y, w, h, selected, Align::Left),
                            1 => draw_data(lbl_text, x, y, w, h, selected, Align::Right),

                            _ => (),
                        };
                    }
                }
                _ => (),
            });
        table
    }

    fn handle_event(&mut self, evt: Event, lens: Arc<Mutex<Lens>>) -> bool {
        let btn = app::event_mouse_button();
        if evt == Event::Released && btn == app::MouseButton::Left {
            let lbl_ix = self.callback_row() as usize;

            // Left click
            match self.callback_context() {
                TableContext::StartPage => println!("Label StartPage!"),
                TableContext::EndPage => println!("Label EndPage!"),
                TableContext::Cell => {
                    let labels_list = {
                        let lens = lens.lock();
                        lens.get_labels().clone()
                    };

                    if let Some(lbl) = labels_list.get(lbl_ix) {
                        let label_id: i32 = lbl.id.into();
                        let label_id: u32 = label_id as u32;

                        {
                            let mut selected_label_ids = self.selected_label_ids.lock();

                            let lbl_is_selected = selected_label_ids.contains(&label_id);

                            if !lbl_is_selected {
                                selected_label_ids.insert(label_id);
                            } else {
                                selected_label_ids.remove(&label_id);
                            }
                        }

                        self.update();
                        return true;
                    }
                }
                _ => (),
            }
        }

        false
    }

    pub fn update(&mut self) {
        println!("Entry label table update");
        let label_count = {
            let mut lens = self.lens.lock();
            lens.update_label_states();
            lens.get_labels().len()
        };

        println!("Label count: {}", label_count);
        self.set_rows(label_count as i32);

        self.sender.send(LabelMessage::LabelListChanged);
        // self.redraw();
    }
}

use std::ops::{Deref, DerefMut};

use super::entry_label_dialog::LabelMessage;

impl Deref for EntryLabelList {
    type Target = TableRow;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}

impl DerefMut for EntryLabelList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}
