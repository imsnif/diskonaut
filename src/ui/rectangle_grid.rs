use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Style, Color, Modifier};
use tui::widgets::{Widget};

use crate::ui::{FileMetadata, FileType, draw_symbol, boundaries, DisplaySize, DisplaySizeRounded};

pub const MINIMUM_HEIGHT: u16 = 2;
pub const MINIMUM_WIDTH: u16 = 8;

#[derive(Clone, Debug)]
pub struct FileSizeRect {
    pub rect: RectFloat,
    pub file_metadata: FileMetadata,
    pub selected: bool,
}

#[derive(Clone, Debug)]
pub struct RectFloat {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone)]
pub struct RectangleGrid {
    rectangles: Vec<FileSizeRect>
}

impl<'a> RectangleGrid {
    pub fn new (rectangles: Vec<FileSizeRect>) -> Self {
        RectangleGrid { rectangles }
    }
}

fn truncate_middle(row: &str, max_length: u16) -> String {
    if max_length < 6 {
        String::from("") // TODO: make sure this never happens
    } else if row.len() as u16 > max_length {
        let first_slice = &row[0..(max_length as usize / 2) - 2];
        let second_slice = &row[(row.len() - (max_length / 2) as usize + 2)..row.len()];
        if max_length % 2 == 0 {
            format!("{}[...]{}", first_slice, second_slice)
        } else {
            format!("{}[..]{}", first_slice, second_slice)
        }
    } else {
        row.to_string()
    }
}

fn truncate_size_line (size: &u64, percentage: &f64, max_length: &u16) -> String {
    let display_size = DisplaySize(*size as f64);
    let display_size_rounded = DisplaySizeRounded(*size as f64); // TODO: better
    let display_size = format!("{}", display_size);
    let display_size_rounded = format!("{}", display_size_rounded);
    if *max_length >= display_size.len() as u16 + 7 { // 7 == "(100%)" + 1 space
        format!("{} ({:.0}%)", display_size, percentage * 100.0)
    } else if *max_length > display_size.len() as u16 {
        display_size
    } else if *max_length > display_size_rounded.len() as u16 {
        display_size_rounded
    } else if *max_length > 6 { // 6 == "(100%)"
        format!("({:.0}%)", (percentage * 100.0).round())
    } else if *max_length >= 4 { // 4 == "100%"
        format!("{:.0}%", (percentage * 100.0).round())
    } else {
        String::from(".") // TODO: make sure this never happens
    }
}

fn draw_rect_on_grid (buf: &mut Buffer, rect: Rect) {
    // top and bottom
    for x in rect.x..(rect.x + rect.width + 1) {
        if x == rect.x {
            draw_symbol(buf, x, rect.y, &boundaries::TOP_LEFT);
            draw_symbol(buf, x, rect.y + rect.height, &boundaries::BOTTOM_LEFT);
        } else if x == rect.x + rect.width {
            draw_symbol(buf, x, rect.y, &boundaries::TOP_RIGHT);
            draw_symbol(buf, x, rect.y + rect.height, &boundaries::BOTTOM_RIGHT);
        } else {
            draw_symbol(buf, x, rect.y, &boundaries::HORIZONTAL);
            draw_symbol(buf, x, rect.y + rect.height, &boundaries::HORIZONTAL);
        }
    }

    // left and right
    for y in (rect.y + 1)..(rect.y + rect.height) {
        draw_symbol(buf, rect.x, y, &boundaries::VERTICAL);
        draw_symbol(buf, rect.x + rect.width, y, &boundaries::VERTICAL);
    }
}

fn draw_rect_text_on_grid(buf: &mut Buffer, rect: &Rect, file_size_rect: &FileSizeRect) { // TODO: better, combine args
    let max_text_length = if rect.width > 2 { rect.width - 2 } else { 0 };
    let name = &file_size_rect.file_metadata.name;
    let percentage = &file_size_rect.file_metadata.percentage;

    let first_line_text = if file_size_rect.selected {
        match file_size_rect.file_metadata.file_type {
            FileType::File => format!("{}", name),
            FileType::Folder => format!("{}/", name),
        }
    } else {
        match file_size_rect.file_metadata.file_type {
            FileType::File => format!("{}", name),
            FileType::Folder=> format!("{}/", name),
        }
    };
    let first_line = truncate_middle(&first_line_text, max_text_length);
    let first_line_length = first_line.len(); // TODO: better
    let first_line_start_position = ((rect.width - first_line_length as u16) as f64 / 2.0).ceil() as u16 + rect.x;

    let second_line = truncate_size_line(&file_size_rect.file_metadata.size, &percentage, &max_text_length);

    let second_line_length = second_line.len(); // TODO: better
    let second_line_start_position = ((rect.width - second_line_length as u16) as f64 / 2.0).ceil() as u16 + rect.x;

    let text_style = if file_size_rect.selected {
        Style::default().bg(Color::Green).fg(Color::White).modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    if file_size_rect.selected {
        let selected_string = format!("{:1$}", " ", max_text_length as usize + 1);
        for y in rect.y..(rect.y + rect.height) {
            if y > rect.y {
                buf.set_string(rect.x + 1, y, &selected_string, text_style);
            }
        }
    }
    if rect.height > 5 {
        let line_gap = if rect.height % 2 == 0 { 1 } else { 2 };
        buf.set_string(first_line_start_position, (rect.height / 2) + rect.y - 1, first_line, text_style);
        buf.set_string(second_line_start_position, (rect.height / 2) + rect.y + line_gap, second_line, text_style);
    } else if rect.height == 5 {
        buf.set_string(first_line_start_position, (rect.height / 2) + rect.y, first_line, text_style);
        buf.set_string(second_line_start_position, (rect.height / 2) + rect.y + 1, second_line, text_style);
    } else if rect.height > 4 {
        buf.set_string(first_line_start_position, rect.y + 1, first_line, text_style);
        buf.set_string(second_line_start_position, rect.y + 2, second_line, text_style);
    } else if rect.height == 4 {
        buf.set_string(first_line_start_position, rect.y + 1, first_line, text_style);
        buf.set_string(second_line_start_position, rect.y + 3, second_line, text_style);
    } else if rect.height > 2 {
        buf.set_string(first_line_start_position, rect.y + 1, first_line, text_style);
        buf.set_string(second_line_start_position, rect.y + 2, second_line, text_style);
    } else {
        buf.set_string(first_line_start_position, rect.y + 1, first_line, text_style);
    }
}

impl<'a> Widget for RectangleGrid {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        if area.width < 2 || area.height < 2 {
            return;
        }
        for file_size_rect in &self.rectangles {
            let rounded_x = file_size_rect.rect.x.round();
            let rounded_y = file_size_rect.rect.y.round();
            let mut rect = Rect {
                x: rounded_x as u16,
                y: rounded_y  as u16,
                width: ((file_size_rect.rect.x - rounded_x) + file_size_rect.rect.width).round() as u16,
                height: ((file_size_rect.rect.y - rounded_y) + file_size_rect.rect.height).round() as u16,
            };

            // fix rounding errors
            if (file_size_rect.rect.x + file_size_rect.rect.width).round() as u16 > rect.x + rect.width {
                rect.width += 1;
            }
            if (file_size_rect.rect.y + file_size_rect.rect.height).round() as u16 > rect.y + rect.height {
                rect.height += 1;
            }

            if rect.height < MINIMUM_HEIGHT || rect.width < MINIMUM_WIDTH {

                for x in rect.x..(rect.x + rect.width + 1) { // +1 because the width might be 0
                    if x > rect.x && x < area.x + area.width {
                        for y in rect.y..(rect.y + rect.height + 1) { // +1 because the height might be 0
                            if y > rect.y && y < area.y + area.height {
                                let buf = buf.get_mut(x, y);
                                buf.set_symbol("x");
                                buf.set_style(Style::default().bg(Color::White).fg(Color::Black));
                            }
                        }
                    }
                }
            } else {
                draw_rect_text_on_grid(buf, &rect, &file_size_rect);
                draw_rect_on_grid(buf, rect);
            }
        }
        draw_rect_on_grid(buf, area); // we draw a frame around the whole area to make up for the "small files" block not having a frame of its own
    }
}
