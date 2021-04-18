use fltk::enums::*;
use fltk::*;

pub fn draw_header(s: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, Color::FrameDefault);
    draw::set_draw_color(Color::Black);
    draw::draw_text2(s, x, y, w, h, Align::Left);
    draw::pop_clip();
}

pub fn draw_data(s: &str, x: i32, y: i32, w: i32, h: i32, selected: bool, align: Align) {
    draw::push_clip(x, y, w, h);
    if selected {
        draw::set_draw_color(Color::from_u32(0xD3D3D3));
    } else {
        draw::set_draw_color(Color::White);
    }
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(Color::Gray0);
    draw::draw_text2(s, x, y, w, h, align);
    draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}

const KB: i64 = 1000;
const MB: i64 = KB * KB;
const GB: i64 = KB * KB * KB;

pub fn pretty_size(size: i64) -> String {
    if size > GB {
        format!("{:.1} GB", (size as f32 / GB as f32))
    } else if size > MB {
        format!("{} MB", (size / MB))
    } else if size > KB {
        format!("{} KB", (size / KB))
    } else {
        format!("{} B", size)
    }
}
