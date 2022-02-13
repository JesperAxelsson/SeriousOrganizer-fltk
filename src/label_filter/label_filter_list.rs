use super::label_filter_dialog::LabelFilterMessage;
use enums::Font;
use fltk::app::Sender;
use parking_lot::Mutex;
use std::sync::Arc;

use fltk::table::*;
use fltk::{enums::*, prelude::*, *};

use serious_organizer_lib::lens::Lens;

use crate::table_utils::{draw_data, draw_header};

#[derive(Clone)]
pub struct LabelFilterList {
    pub wid: TableRow,
    lens: Arc<Mutex<Lens>>,
    pub sender: Sender<LabelFilterMessage>,
}

impl LabelFilterList {
    pub fn new(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        lens: Arc<Mutex<Lens>>,
        sender: Sender<LabelFilterMessage>,
    ) -> LabelFilterList {
        let headers = vec![
            "Name".to_string(),
            "Filter".to_string(),
            "Label".to_string(),
        ];

        let mut table = LabelFilterList {
            wid: TableRow::new(x, y, w, h, ""),
            lens,
            sender,
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

        table
            .wid
            .draw_cell(move |t, ctx, row, col, x, y, w, h| match ctx {
                table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
                table::TableContext::ColHeader => draw_header(&headers[col as usize], x, y, w, h),
                table::TableContext::Cell => {
                    //TODO: Add selected?
                    let selected = t.row_selected(row);

                    let lens = lens_c.lock();
                    let labels_filter = lens.get_label_filters();
                    if let Some(filter) = labels_filter.get(row as usize) {
                        if col == 0 || col == 1 {
                            match col {
                                0 => draw_data(&filter.name, x, y, w, h, selected, Align::Left),
                                1 => draw_data(&filter.filter, x, y, w, h, selected, Align::Left),
                                _ => (),
                            };
                        } else if col == 2 {
                            let label_lst = lens.get_labels();
                            if let Some(lbl) = label_lst.iter().find(|l| l.id == filter.label_id) {
                                draw_data(&lbl.name, x, y, w, h, selected, Align::Left);
                            }
                        }
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
                TableContext::StartPage => println!("Label filter StartPage!"),
                TableContext::EndPage => println!("Label filter EndPage!"),
                TableContext::Cell => {
                    let label_filter_list = {
                        let lens = lens.lock();
                        lens.get_label_filters()
                    };

                    if let Some(lbl) = label_filter_list.get(lbl_ix) {
                        self.sender
                            .send(LabelFilterMessage::ListSelected(Some(lbl.clone())));
                    } else {
                        self.sender.send(LabelFilterMessage::ListSelected(None));
                    }

                    return true;
                }
                TableContext::Table => self.sender.send(LabelFilterMessage::ListSelected(None)), // Clicked is on table background, row is deselected
                _ => (),
            }
        }

        false
    }

    pub fn update(&mut self) {
        println!("label filter table update");
        let label_count = {
            let mut lens = self.lens.lock();
            lens.update_label_states();
            lens.get_label_filters().len()
        };

        println!("Label filter count: {}", label_count);
        self.set_rows(label_count as i32);

        // self.sender.send(LabelFilterMessage::ListChanged);
        self.redraw();
    }
}

use std::ops::{Deref, DerefMut};

impl Deref for LabelFilterList {
    type Target = TableRow;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}

impl DerefMut for LabelFilterList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}
