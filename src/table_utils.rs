#![allow(clippy::too_many_arguments)]
use std::cmp::{self};

use fltk::enums::*;
use fltk::prelude::{TableExt, WidgetExt};
use fltk::table::TableRow;
use fltk::*;

pub fn draw_header(s: &str, x: i32, y: i32, w: i32, h: i32) {
    draw::push_clip(x, y, w, h);
    draw::draw_box(FrameType::ThinUpBox, x, y, w, h, Color::FrameDefault);
    draw::set_draw_color(Color::Black);
    draw::draw_text2(s, x, y, w, h, Align::Left);
    draw::pop_clip();
}

pub fn draw_data(s: &str, x: i32, y: i32, w: i32, h: i32, selected: bool, align: Align) {
    draw_data_color(s, x, y, w, h, Color::Gray0, selected, align)
}

pub fn draw_data_color(
    s: &str,
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    text_color: Color,
    selected: bool,
    align: Align,
) {
    draw::push_clip(x, y, w, h);
    if selected {
        draw::set_draw_color(Color::from_u32(0xD3D3D3));
    } else {
        draw::set_draw_color(Color::White);
    }
    draw::draw_rectf(x, y, w, h);
    draw::set_draw_color(text_color);
    draw::draw_text2(s, x, y, w, h, align);
    draw::set_draw_color(Color::Gray0);
    draw::draw_rect(x, y, w, h);
    draw::pop_clip();
}

pub fn get_file_color(file_name: &str) -> Color {
    const VIDEO_FORMATS: [&str; 23] = [
        ".mkv", ".webm", ".flv", ".vob", ".ogg", ".ogv", ".avi", ".mov", ".qt", ".wmv", ".rm",
        ".rmvb", ".asf", ".amv", ".mp4", ".m4p", ".m4v", ".mpg", ".mp2", ".mpeg", ".mpe", ".mpv",
        ".m2v",
    ];

    let file_name = file_name.to_lowercase();

    for fmt in VIDEO_FORMATS {
        if file_name.ends_with(fmt) {
            return Color::Blue;
        }
    }

    Color::Gray0
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

#[derive(Debug, Clone)]
pub struct ColHeader {
    pub label: String,
    pub width: ColSize,
}

#[derive(Debug, Clone, Copy)]
pub enum ColSize {
    Fixed(i32),
    Ratio(f32),
    Greedy,
}

// impl ColSize {

// }

impl ColHeader {
    pub fn new(label: &str, col_size: ColSize) -> Self {
        ColHeader {
            label: label.to_owned(),
            width: col_size,
        }
    }
}

pub fn resize_column(table: &mut TableRow, columns: &Vec<ColHeader>) {
    const MIN_WIDTH: i32 = 50;
    let width = table.width()-20;

    let mut sum_width = 0;
    let new_sizes = resize_column_internal(width, MIN_WIDTH, columns);
    for (ix, width) in new_sizes.iter() {
        table.set_col_width(*ix, *width);
        sum_width+=*width;
    }

    println!("Cols resize, width: {} sum {} w: {}", width, sum_width, table.w()
);
}

fn resize_column_internal(
    table_width: i32,
    min_width: i32,
    columns: &Vec<ColHeader>,
) -> Vec<(i32, i32)> {
    let mut result = Vec::new();

    let mut width_left = table_width;
    let cols = columns.iter().enumerate().collect::<Vec<_>>();

    for (ix, header) in cols
        .iter()
        .filter(|(_, c)| matches!(c.width, ColSize::Fixed(_)))
    {
        if let ColSize::Fixed(width) = header.width {
            // table.set_col_width(*ix as i32, width);
            result.push((*ix as i32, width));
            width_left -= width;
        }
    }

    let mut ratio_size = 0;
    for (ix, header) in cols
        .iter()
        .filter(|(_, c)| matches!(c.width, ColSize::Ratio(_)))
    {
        if let ColSize::Ratio(ratio) = header.width {
            let width = cmp::max(((width_left as f32 * ratio) as i32) as i32, min_width);
            result.push((*ix as i32, width));
            ratio_size += width;
        }
    }

    width_left -= ratio_size;

    let greedy_count = cols
        .iter()
        .filter(|(_, c)| matches!(c.width, ColSize::Greedy))
        .count();

    if greedy_count > 0 {
        let greedy_size = cmp::max((width_left / greedy_count as i32) as i32, min_width);

        for (ix, _) in cols
            .iter()
            .filter(|(_, c)| matches!(c.width, ColSize::Greedy))
        {
            result.push((*ix as i32, greedy_size));
        }
    }

    result.sort_by_key(|e| e.0);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fixed_simple() {
        let headers = vec![
            ColHeader::new("Path", ColSize::Fixed(50)),
            ColHeader::new("Size", ColSize::Fixed(50)),
        ];

        let sizes = resize_column_internal(100, 20, &headers);

        assert_eq!(sizes, vec![(0, 50), (1, 50)]);
    }

    #[test]
    fn test_fixed_and_greedy_simple() {
        let headers = vec![
            ColHeader::new("Path", ColSize::Fixed(50)),
            ColHeader::new("Size", ColSize::Greedy),
            ColHeader::new("Size", ColSize::Fixed(50)),
        ];

        let sizes = resize_column_internal(120, 20, &headers);

        assert_eq!(sizes, vec![(0, 50), (1, 20), (2, 50)]);
    }

    #[test]
    fn test_ratio_simple() {
        let headers = vec![
            ColHeader::new("Path", ColSize::Ratio(0.5)),
            ColHeader::new("Size", ColSize::Ratio(0.5)),
        ];

        let sizes = resize_column_internal(100, 20, &headers);

        assert_eq!(sizes, vec![(0, 50), (1, 50)]);
    }

    #[test]
    fn test_greedy_single() {
        let headers = vec![
            ColHeader::new("Path", ColSize::Ratio(0.5)),
            ColHeader::new("Size", ColSize::Ratio(0.5)),
        ];

        let sizes = resize_column_internal(100, 20, &headers);

        assert_eq!(sizes, vec![(0, 50), (1, 50)]);
    }

    #[test]
    fn test_greedy_two() {
        let headers = vec![
            ColHeader::new("Path", ColSize::Greedy),
            ColHeader::new("Size", ColSize::Greedy),
        ];

        let sizes = resize_column_internal(100, 20, &headers);

        assert_eq!(sizes, vec![(0, 50), (1, 50)]);
    }

    #[test]
    fn test_fixed_and_ratio() {
        let headers = vec![
            ColHeader::new("Path", ColSize::Fixed(50)),
            ColHeader::new("Size", ColSize::Ratio(0.5)),
        ];

        let sizes = resize_column_internal(100, 20, &headers);

        assert_eq!(sizes, vec![(0, 50), (1, 25)]);
    }

    #[test]
    fn test_fixed_and_ratio_and_greedy() {
        let headers = vec![
            ColHeader::new("Path", ColSize::Fixed(50)),
            ColHeader::new("Size", ColSize::Greedy),
            ColHeader::new("Size", ColSize::Ratio(0.5)),
        ];

        let sizes = resize_column_internal(100, 20, &headers);

        assert_eq!(sizes, vec![(0, 50), (1, 25), (2, 25)]);
    }

    #[test]
    fn test_fixed_and_ratio_and_greedy_width_too_small() {
        let headers = vec![
            ColHeader::new("Path", ColSize::Fixed(50)),
            ColHeader::new("Size", ColSize::Greedy),
            ColHeader::new("Size", ColSize::Ratio(2.)),
        ];

        let sizes = resize_column_internal(100, 20, &headers);

        assert_eq!(sizes, vec![(0, 50), (1, 20), (2, 100)]);
    }
}
