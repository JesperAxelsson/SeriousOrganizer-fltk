use parking_lot::Mutex;
use std::{cell::RefCell, sync::Arc};

use fltk::table::*;
use fltk::*;

use serious_organizer_lib::lens::LabelState;
use serious_organizer_lib::lens::Lens;

use time::Instant;

use crate::table_utils::{draw_data, draw_header};
 
use std::rc::Rc;

#[derive(Clone)]
pub struct LabelList {
    pub wid: TableRow,
    lens: Arc<Mutex<Lens>>,
    last_clicked: Instant,
    on_update: Rc<RefCell<dyn FnMut() -> ()>>,
}

// use std::rc::Rc;

impl LabelList {
    pub fn new(
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        lens: Arc<Mutex<Lens>>,
        // on_update: Box<dyn Fn() -> () >
        on_update: Rc<RefCell<dyn FnMut() -> ()>>,
    ) -> LabelList {
        let headers = vec!["Name".to_string(), "State".to_string()];
        // let x2 = dyn_clone::clone_box(&*on_update);
        let mut table = LabelList {
            wid: TableRow::new(x, y, w, h, ""),
            lens: lens,
            last_clicked: Instant::now(),
            on_update,
        };

        table.set_row_height_all(20);
        table.set_row_resize(true);
        table.set_type(TableRowSelectMode::SelectSingle);

        // Cols
        table.set_cols(headers.len() as u32);
        table.set_col_header(true);
        table.set_col_resize(true);

        table.end();

        table.update_size();

        let lens_c = table.lens.clone();
        let mut table_c = table.clone();
        table.handle(move |evt| table_c.handle_event(evt, lens_c.clone()));
        println!("Setup label click handler");

        let lens_c = table.lens.clone();

        table
            .wid
            .draw_cell(move |ctx, row, col, x, y, w, h| match ctx {
                table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
                table::TableContext::ColHeader => draw_header(&headers[col as usize], x, y, w, h),
                // table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h),
                table::TableContext::Cell => {
                    let selected = false;

                    let l = lens_c.lock();
                    let label_lst = l.get_labels();
                    let ref lbl = label_lst[row as usize];

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
        self.set_rows(label_count as u32);
    }

    pub fn get_selected_index(&mut self) -> Vec<u32> {
        let mut selected = Vec::new();
        // draw_data(&data, x, y, w, h, table_c.row_selected(row), align);
        for ix in 0..self.rows() {
            if self.row_selected(ix as i32) {
                selected.push(ix as u32);
            }
        }
        selected
    }

    fn handle_event(&mut self, evt: Event, lens: Arc<Mutex<Lens>>) -> bool {
        // if app::event_is_click() && (evt == Event::Push || evt == Event::Released) {
        //     println!("Click {:?}", evt);
        // }

        // if !app::event_is_click() || app::event_clicks() {
        //     return false;
        // }

        // if evt == Event::Push {
        //     return false;
        // }
        if app::event_is_click() && evt == Event::Push {
            if self.last_clicked.elapsed().whole_milliseconds() < 20 {
                println!("Time limit");
                return false;
            }
            self.last_clicked = Instant::now();

            let lbl = self.get_selected_index();

            // println!("Got selected lbls: {} {:?}", app::event_is_click(), evt);

            if lbl.len() == 0 {
                return false;
            }

            let lbl_ix = lbl[0] as usize;
            let state_change = {
                let mut lens = lens.lock();

                let labels_list = lens.get_labels();
                let ref lbl = labels_list[lbl_ix];
                let label_id: i32 = lbl.id.into();
                let label_id: u32 = label_id as u32;

                let btn = app::event_button();

                // Left click
                if btn == 1 {
                    println!("Mouse left clicked {:?}", lbl.state);

                    match lbl.state {
                        LabelState::Unset => lens.add_inlude_label(label_id),
                        LabelState::Include => (), // Do nothing
                        LabelState::Exclude => lens.remove_label_filter(label_id),
                    };

                    self.redraw();
                    true

                // Right click
                } else if btn == 3 {
                    println!("Mouse right clicked {:?}", lbl.state);

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
            };

            if state_change {
                self.on_update.borrow_mut()();
            }
        }
        false
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
