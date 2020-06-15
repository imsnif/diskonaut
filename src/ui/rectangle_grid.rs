use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Style, Color, Modifier};
use tui::widgets::{Widget};

use std::ffi::OsString;

use crate::state::FileType;
use crate::ui::{draw_symbol, boundaries};
use crate::ui::format::{DisplaySize, DisplaySizeRounded, truncate_middle};
use crate::state::Tile;


#[derive(Clone)]
pub struct RectangleGrid<'a> {
    rectangles: &'a [Tile],
    small_files_coordinates: Option<(u16, u16)>,
    selected_rect_index: Option<usize>,
}

impl<'a> RectangleGrid<'a> {
    pub fn new (rectangles: &'a [Tile], small_files_coordinates: Option<(u16, u16)>, selected_rect_index: Option<usize>) -> Self {
        RectangleGrid { rectangles, small_files_coordinates, selected_rect_index }
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

fn draw_rect_on_grid (buf: &mut Buffer, rect: &Tile) {
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
    for x in rect.x + 1..(rect.x + rect.width) {
        for y in rect.y + 1..(rect.y + rect.height) {
            let buf = buf.get_mut(x, y);
            buf.set_symbol("x");
            buf.set_style(Style::default().bg(Color::White).fg(Color::Black));
        }
    }
    // TODO: combine with draw_rect_on_grid
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

fn draw_rect_text_on_grid(buf: &mut Buffer, tile: &Tile, selected: bool) { // TODO: better, combine args
    let max_text_length = if tile.width > 2 { tile.width - 2 } else { 0 };
    let name = &tile.name.to_string_lossy();
    let descendant_count = &tile.descendants;
    let percentage = &tile.percentage;

    let filename_text = if selected {
        match tile.file_type {
            FileType::File => format!("{}", name),
            FileType::Folder => format!("{}/", name),
        }
    } else {
        match tile.file_type {
            FileType::File => format!("{}", name),
            FileType::Folder=> format!("{}/", name),
        }
    };
    let first_line = match tile.file_type {
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
    let first_line_start_position = ((tile.width - first_line_length as u16) as f64 / 2.0).ceil() as u16 + tile.x;

    let second_line = truncate_size_line(&tile.size, &percentage, &max_text_length);

    let second_line_length = second_line.len(); // TODO: better
    let second_line_start_position = ((tile.width - second_line_length as u16) as f64 / 2.0).ceil() as u16 + tile.x;

    let ( background_style, first_line_style, second_line_style ) = match ( selected, &tile.file_type ) {
        ( true, FileType::File ) => {
            (
                Some(Style::default().fg(Color::DarkGray).bg(Color::DarkGray)),
                Style::default().fg(Color::Black).bg(Color::DarkGray),
                Style::default().fg(Color::Black).bg(Color::DarkGray)
            )
        },
        ( false, FileType::File ) => {
            (
                None,
                Style::default(),
                Style::default(),
            )
        },
        ( true, FileType::Folder) => {
            (
                Some(Style::default().fg(Color::Blue).bg(Color::Blue)),
                Style::default().fg(Color::White).bg(Color::Blue).modifier(Modifier::BOLD),
                Style::default().fg(Color::Black).bg(Color::Blue),
            )
        },
        ( false, FileType::Folder) => {
            (
                None,
                Style::default().fg(Color::Blue).modifier(Modifier::BOLD),
                Style::default(),
            )
        }
    };

    if let Some(background_style) = background_style {
        for x in tile.x + 1..tile.x + tile.width {
            for y in tile.y + 1..tile.y + tile.height {
                buf.get_mut(x, y).set_symbol("█").set_style(background_style);
                // we set both the filling symbol and the style
                // because some terminals do not show this symbol on the one side
                // and our tests need it in order to pass on the other side
                // some terminals also don't have colors and would need this
                // as an indication so... best of all worlds!
            }
        }
    }

    if tile.height > 5 {
        let line_gap = if tile.height % 2 == 0 { 1 } else { 2 };
        buf.set_string(first_line_start_position, (tile.height / 2) + tile.y - 1, first_line, first_line_style);
        buf.set_string(second_line_start_position, (tile.height / 2) + tile.y + line_gap, second_line, second_line_style);
    } else if tile.height == 5 {
        buf.set_string(first_line_start_position, (tile.height / 2) + tile.y, first_line, first_line_style);
        buf.set_string(second_line_start_position, (tile.height / 2) + tile.y + 1, second_line, second_line_style);
    } else if tile.height > 4 {
        buf.set_string(first_line_start_position, tile.y + 1, first_line, first_line_style);
        buf.set_string(second_line_start_position, tile.y + 2, second_line, second_line_style);
    } else if tile.height == 4 {
        buf.set_string(first_line_start_position, tile.y + 1, first_line, first_line_style);
        buf.set_string(second_line_start_position, tile.y + 3, second_line, second_line_style);
    } else if tile.height > 2 {
        buf.set_string(first_line_start_position, tile.y + 1, first_line, first_line_style);
        buf.set_string(second_line_start_position, tile.y + 2, second_line, second_line_style);
    } else {
        buf.set_string(first_line_start_position, tile.y + 1, first_line, first_line_style);
    }
}

impl<'a> Widget for RectangleGrid<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        if area.width < 2 || area.height < 2 {
            return;
        }
        if self.rectangles.len() == 0 {
            for x in area.x + 1..area.x + area.width {
                for y in area.y + 1..area.y + area.height {
                    let buf = buf.get_mut(x, y);
                    buf.set_symbol("█");
                    buf.set_style(Style::default().bg(Color::White).fg(Color::Black));
                }
            }
            let empty_folder_line = "Folder is empty";
            let text_length = empty_folder_line.len();
            let text_style = Style::default();
            let text_start_position = ((area.width - text_length as u16) as f64 / 2.0).ceil() as u16 + area.x;
            buf.set_string(text_start_position, (area.height / 2) + area.y - 1, empty_folder_line, text_style);
            draw_rect_on_grid(buf, &Tile { // TODO: better
                x: area.x,
                y: area.y,
                width: area.width,
                height: area.height,
                name: OsString::new(),
                size: 0,
                descendants: None,
                percentage: 0.0,
                file_type: FileType::Folder,
            });
        } else {
            for (index, tile) in self.rectangles.into_iter().enumerate() {
                let selected = if let Some(selected_rect_index) = self.selected_rect_index {
                    index == selected_rect_index
                } else {
                    false
                };
                draw_rect_text_on_grid(buf, &tile, selected);
                draw_rect_on_grid(buf, &tile);
            }
        }
        if let Some(coords) = self.small_files_coordinates {
            let (x, y) = coords;
            let small_files_rect = Rect {
                x,
                y,
                width: (area.x + area.width) - x,
                height: (area.y + area.height) - y,
            };
            draw_small_files_rect_on_grid(buf, small_files_rect);
        }
    }
}
