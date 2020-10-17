use std::sync::Arc;
use parking_lot::Mutex;

use fltk::table::*;
use fltk::*;

use serious_organizer_lib::lens::Lens;
use serious_organizer_lib::lens::LabelState;


use crate::table_utils::{draw_data, draw_header};
#[derive(Clone)]
pub struct LabelList {
    pub wid: TableRow,
    lens: Arc<Mutex<Lens>>,
}

// use std::rc::Rc;

impl LabelList {
    pub fn new(x: i32, y: i32, w: i32, h: i32, lens: Arc<Mutex<Lens>>) -> LabelList {
        let headers = vec!["Name".to_string(), "State".to_string()];
        let mut table = LabelList {
            wid: TableRow::new(x, y, w, h, ""),
            lens: lens,
        };

        table.wid.set_row_height_all(20);
        table.wid.set_row_resize(true);

        // Cols
        table.wid.set_cols(headers.len() as u32);
        table.wid.set_col_header(true);
        table.wid.set_col_resize(true);

        table.wid.end();
        table.wid.set_rows(0);

        let lens_c = table.lens.clone();
 
        table
            .wid
            .draw_cell(Box::new(move |ctx, row, col, x, y, w, h| match ctx {
                table::TableContext::StartPage => draw::set_font(Font::Helvetica, 14),
                table::TableContext::ColHeader => draw_header(&headers[col as usize], x, y, w, h),
                // table::TableContext::RowHeader => draw_header(&format!("{}", row + 1), x, y, w, h),
                table::TableContext::Cell => {
                        let selected = false; //table_c.row_selected(row);
                        let l = lens_c.lock();
                        let label_lst= l.get_labels();
                        let ref lbl = label_lst[row as usize];
                        

                        let lbl_text = match lbl.state {
                            LabelState::Unset=> "U",
                            LabelState::Include=> "I",
                            LabelState::Exclude=> "E",
                        };
                        
                        match col {
                            0 => draw_data(&lbl.name, x, y, w, h, selected, Align::Left),
                            1 => draw_data(lbl_text, x, y, w, h, selected, Align::Right),
                            
                            _ => (),
                        };
                
                }
                _ => (),
            }));
        table
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
