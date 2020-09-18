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

