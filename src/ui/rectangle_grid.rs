use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Style, Color, Modifier};
use tui::widgets::{Widget};

use crate::state::FileType;
use crate::ui::{draw_symbol, boundaries};
use crate::ui::format::{DisplaySize, DisplaySizeRounded, truncate_middle};
use crate::state::FileRect;

pub const MINIMUM_HEIGHT: u16 = 2;
pub const MINIMUM_WIDTH: u16 = 8;

#[derive(Clone)]
pub struct RectangleGrid {
    rectangles: Vec<FileRect>
}

impl<'a> RectangleGrid {
    pub fn new (rectangles: Vec<FileRect>) -> Self {
        RectangleGrid { rectangles }
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
        unreachable!("trying to render a rect of less than minimum size")
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

fn draw_small_files_rect_on_grid(buf: &mut Buffer, rect: Rect) {
    for x in rect.x..rect.width {
        for y in rect.y..rect.height {
            let buf = buf.get_mut(x, y);
            buf.set_symbol("x");
            buf.set_style(Style::default().bg(Color::White).fg(Color::Black));
        }
    }
}

fn draw_rect_text_on_grid(buf: &mut Buffer, rect: &Rect, file_rect: &FileRect) { // TODO: better, combine args
    let max_text_length = if rect.width > 2 { rect.width - 2 } else { 0 };
    let name = &file_rect.file_metadata.name.to_string_lossy();
    let descendant_count = &file_rect.file_metadata.descendants;
    let percentage = &file_rect.file_metadata.percentage;

    let filename_text = if file_rect.selected {
        match file_rect.file_metadata.file_type {
            FileType::File => format!("{}", name),
            FileType::Folder => format!("{}/", name),
        }
    } else {
        match file_rect.file_metadata.file_type {
            FileType::File => format!("{}", name),
            FileType::Folder=> format!("{}/", name),
        }
    };
    let first_line = match file_rect.file_metadata.file_type {
        FileType::File => {
            truncate_middle(&filename_text, max_text_length)
        },
        FileType::Folder => {
            let descendant_count = descendant_count.expect("folder should have descendants");
            let short_descendants_indication = format!("(+{})", descendant_count); // TODO: use DisplaySize in case there is a bazillion
            let long_descendants_indication = format!("(+{} descendants)", descendant_count); // TODO: use DisplaySize in case there is a bazillion
            if &filename_text.len() + long_descendants_indication.len() <= max_text_length as usize {
                format!("{} {}", filename_text, long_descendants_indication)
            } else if &filename_text.len() + short_descendants_indication.len() <= max_text_length as usize {
                format!("{} {}", filename_text, short_descendants_indication)
            } else {
                truncate_middle(&filename_text, max_text_length)
            }
        }
    };
    let first_line_length = first_line.len(); // TODO: better
    let first_line_start_position = ((rect.width - first_line_length as u16) as f64 / 2.0).ceil() as u16 + rect.x;

    let second_line = truncate_size_line(&file_rect.file_metadata.size, &percentage, &max_text_length);

    let second_line_length = second_line.len(); // TODO: better
    let second_line_start_position = ((rect.width - second_line_length as u16) as f64 / 2.0).ceil() as u16 + rect.x;


    let first_line_style = if file_rect.selected {
        match file_rect.file_metadata.file_type {
            FileType::File => Style::default().bg(Color::Green).fg(Color::White).modifier(Modifier::BOLD),
            FileType::Folder => Style::default().bg(Color::Green).fg(Color::Magenta).modifier(Modifier::BOLD)
        }
    } else {
        match file_rect.file_metadata.file_type {
            FileType::File => Style::default().fg(Color::White),
            FileType::Folder => Style::default().fg(Color::Blue).modifier(Modifier::BOLD)
        }
    };

    let text_style = if file_rect.selected {
        Style::default().bg(Color::Green).fg(Color::White).modifier(Modifier::BOLD)
    } else {
        Style::default()
    };

    if file_rect.selected {
        let selected_string = format!("{:1$}", " ", max_text_length as usize + 1);
        for y in rect.y..(rect.y + rect.height) {
            if y > rect.y {
                buf.set_string(rect.x + 1, y, &selected_string, text_style);
            }
        }
    }
    if rect.height > 5 {
        let line_gap = if rect.height % 2 == 0 { 1 } else { 2 };
        buf.set_string(first_line_start_position, (rect.height / 2) + rect.y - 1, first_line, first_line_style);
        buf.set_string(second_line_start_position, (rect.height / 2) + rect.y + line_gap, second_line, text_style);
    } else if rect.height == 5 {
        buf.set_string(first_line_start_position, (rect.height / 2) + rect.y, first_line, first_line_style);
        buf.set_string(second_line_start_position, (rect.height / 2) + rect.y + 1, second_line, text_style);
    } else if rect.height > 4 {
        buf.set_string(first_line_start_position, rect.y + 1, first_line, first_line_style);
        buf.set_string(second_line_start_position, rect.y + 2, second_line, text_style);
    } else if rect.height == 4 {
        buf.set_string(first_line_start_position, rect.y + 1, first_line, first_line_style);
        buf.set_string(second_line_start_position, rect.y + 3, second_line, text_style);
    } else if rect.height > 2 {
        buf.set_string(first_line_start_position, rect.y + 1, first_line, first_line_style);
        buf.set_string(second_line_start_position, rect.y + 2, second_line, text_style);
    } else {
        buf.set_string(first_line_start_position, rect.y + 1, first_line, first_line_style);
    }
}

struct SmallFilesArea {
    leftmost_top_left_coordinates: Option<(u16, u16)>,
    highest_top_left_coordinates: Option<(u16, u16)>,
}

impl SmallFilesArea {
    pub fn new () -> Self {
        Self {
            leftmost_top_left_coordinates: None,
            highest_top_left_coordinates: None,
        }
    }
    pub fn add_rect(&mut self, rect: &Rect) {
        match self.leftmost_top_left_coordinates {
            Some((x, y)) => {
                if rect.x == 0 {
                    // do nothing
                    // this happens because of a bug in treemap.rs
                    // where somehow file_rects are created with x/y as NaN
                    // TODO: fix this properly
                } else if rect.x < x {
                    self.leftmost_top_left_coordinates = Some((rect.x, rect.y));
                } else if rect.x == x && rect.y < y {
                    self.leftmost_top_left_coordinates = Some((rect.x, rect.y));
                }
            },
            None => {
                self.leftmost_top_left_coordinates = Some((rect.x, rect.y));
            }
        }

        match self.highest_top_left_coordinates {
            Some((x, y)) => {
                if rect.y == 0 {
                    // do nothing
                    // this happens because of a bug in treemap.rs
                    // where somehow file_rects are created with x/y as NaN
                    // TODO: fix this properly
                } else if rect.y < y {
                    self.highest_top_left_coordinates = Some((rect.x, rect.y));
                } else if rect.y == y && rect.x < x {
                    self.highest_top_left_coordinates = Some((rect.x, rect.y));
                }
            },
            None => {
                self.highest_top_left_coordinates = Some((rect.x, rect.y));
            }
        }
    }
    pub fn draw(&self, area: &Rect, buf: &mut Buffer) {
        if let Some((small_files_start_x, small_files_start_y)) = self.highest_top_left_coordinates {
            if small_files_start_x > 0 && small_files_start_y > 0 {
                draw_small_files_rect_on_grid(buf, Rect {
                    x: small_files_start_x + 1,
                    y: small_files_start_y + 1,
                    width: area.x + area.width,
                    height: area.y + area.height,
                });
            } else {
                // TODO: ui indication that we have X small unrenderable files
            }
        }
        if let Some((small_files_start_x, small_files_start_y)) = self.leftmost_top_left_coordinates {
            if small_files_start_x > 0 && small_files_start_y > 0 {
                draw_small_files_rect_on_grid(buf, Rect {
                    x: small_files_start_x + 1,
                    y: small_files_start_y + 1,
                    width: area.x + area.width,
                    height: area.y + area.height,
                });
            } else {
                // TODO: ui indication that we have X small unrenderable files
            }
        }
    }
}

impl<'a> Widget for RectangleGrid {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        if area.width < 2 || area.height < 2 {
            return;
        }
        let mut small_files = SmallFilesArea::new();
        for file_rect in &self.rectangles {
            let rect = file_rect.rect.round();

            if rect.height < MINIMUM_HEIGHT || rect.width < MINIMUM_WIDTH {
                small_files.add_rect(&rect);
            } else {
                draw_rect_text_on_grid(buf, &rect, &file_rect);
                draw_rect_on_grid(buf, rect);
            }
        }
        small_files.draw(&area, buf);
        draw_rect_on_grid(buf, area); // we draw a frame around the whole area to make up for the "small files" block not having a frame of its own
    }
}
