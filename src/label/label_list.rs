use fltk::app::Sender;
use parking_lot::Mutex;
use std::sync::Arc;

use fltk::table::*;
use fltk::{enums::*, prelude::*, *};

use serious_organizer_lib::lens::LabelState;
use serious_organizer_lib::lens::Lens;

use crate::model::message::Message;
use crate::table_utils::{draw_data, draw_header};

#[derive(Clone)]
pub struct LabelList {
    pub wid: TableRow,
    lens: Arc<Mutex<Lens>>,
    sender: Sender<Message>,
}

// use std::rc::Rc;

impl LabelList {
    pub fn new(w: i32, h: i32, lens: Arc<Mutex<Lens>>, sender: Sender<Message>) -> LabelList {
        let headers = vec!["Name".to_string(), "State".to_string()];
        // let x2 = dyn_clone::clone_box(&*on_update);
        let mut table = LabelList {
            wid: TableRow::default().with_size(w, h),
            lens: lens,
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

        table.update_size();

        let lens_c = table.lens.clone();
        let mut table_c = table.clone();
        table.handle(move |_, evt| table_c.handle_event(evt, lens_c.clone()));
        println!("Setup label click handler");

        let lens_c = table.lens.clone();

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
                        let lbl_text = match lbl.state {
                            LabelState::Unset => "U",
                            LabelState::Include => "I",
                            LabelState::Exclude => "E",
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

    pub fn update_size(&mut self) {
        let label_count = {
            let mut lens = self.lens.lock();
            lens.update_label_states();
            lens.get_labels().len()
        };

        println!("Label count: {}", label_count);
        self.set_rows(label_count as i32);
    }

    pub fn handle_event(&mut self, evt: Event, lens: Arc<Mutex<Lens>>) -> bool {
        if self.callback_context() != TableContext::Cell || evt == Event::NoEvent {
            return false;
        }

        let is_inside = app::event_inside_widget(&self.wid);

        if !is_inside {
            if app::event_is_click()
            //&& evt == Event::Released
            {
                println!("*******************************************************************");
                println!(
                    "***** Got event not inside label widget! {:?} {:?} ******",
                    self.callback_context(),
                    evt
                );
                println!("*******************************************************************");
            }
            return false;
        }

        if app::event_is_click() && evt == Event::Released {
            println!(
                "Label got click: {:?} inside: {} ",
                self.callback_context(),
                is_inside
            );
        }

        if app::event_is_click()
            && evt == Event::Released
            && self.callback_context() == TableContext::Cell
        {
            let lbl_ix = self.callback_row() as usize;
            let state_change = {
                let mut lens = lens.lock();

                let labels_list = lens.get_labels();
                if let Some(lbl) = labels_list.get(lbl_ix) {
                    let label_id= lbl.id as u32;

                    let btn = app::event_button();

                    // Left click
                    if btn == 1 {
                        println!(
                            "Mouse left clicked {:?} {:?} {:?}",
                            lbl.state,
                            lbl.name,
                            self.callback_context()
                        );

                        match lbl.state {
                            LabelState::Unset => lens.add_inlude_label(label_id),
                            LabelState::Include => (), // Do nothing
                            LabelState::Exclude => lens.remove_label_filter(label_id),
                        };

                        self.redraw();
                        true

                    // Right click
                    } else if btn == 3 {
                        println!(
                            "Mouse right clicked {:?} {:?} {:?}",
                            lbl.state,
                            lbl.name,
                            self.callback_context()
                        );

                        match lbl.state {
                            LabelState::Unset => lens.add_exclude_label(label_id),
                            LabelState::Include => lens.remove_label_filter(label_id),
                            LabelState::Exclude => (), // Do nothing
                        };

                        self.redraw();
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            };

            if state_change {
                self.sender.send(Message::EntryTableInvalidated);
                return true;
            }
        }
        false
    }

    pub fn update(&mut self) {
        self.update_size();
    }
}

use std::ops::{Deref, DerefMut};

impl Deref for LabelList {
    type Target = TableRow;

    fn deref(&self) -> &Self::Target {
        &self.wid
    }
}

impl DerefMut for LabelList {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.wid
    }
}
