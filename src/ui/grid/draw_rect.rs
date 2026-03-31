use ::unicode_width::UnicodeWidthStr;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};

use crate::state::tiles::{FileType, Tile};
use crate::ui::format::{truncate_middle, DisplaySize, DisplaySizeRounded};
use crate::ui::grid::{boundaries, draw_next_symbol};

fn tile_first_line(tile: &Tile) -> String {
    let max_text_length = tile.width.saturating_sub(2);
    let name = &tile.name.to_string_lossy();
    let descendant_count = &tile.descendants;
    let filename_text = match tile.file_type {
        FileType::File => format!("{name}"),
        FileType::Folder => format!("{name}/"),
    };
    match tile.file_type {
        FileType::File => truncate_middle(&filename_text, max_text_length),
        FileType::Folder => {
            let descendant_count = descendant_count.expect("folder should have descendants");
            let short_descendants_indication = format!("(+{descendant_count})");
            let long_descendants_indication = format!("(+{descendant_count} descendants)");
            if filename_text.len() + long_descendants_indication.len() <= max_text_length as usize {
                format!("{filename_text} {long_descendants_indication}")
            } else if filename_text.len() + short_descendants_indication.len()
                <= max_text_length as usize
            {
                format!("{filename_text} {short_descendants_indication}")
            } else {
                truncate_middle(&filename_text, max_text_length)
            }
        }
    }
}

fn tile_second_line(tile: &Tile) -> String {
    let max_text_length = tile.width.saturating_sub(2);
    let percentage = &tile.percentage;
    let display_size = DisplaySize(tile.size as f64);
    let display_size_rounded = DisplaySizeRounded(tile.size as f64);
    let display_size = format!("{display_size}");
    let display_size_rounded = format!("{display_size_rounded}");
    if max_text_length >= display_size.len() as u16 + 7 {
        // 7 == "(100%)" + 1 space
        format!("{} ({:.0}%)", display_size, percentage * 100.0)
    } else if max_text_length > display_size.len() as u16 {
        display_size
    } else if max_text_length > display_size_rounded.len() as u16 {
        display_size_rounded
    } else if max_text_length > 6 {
        // 6 == "(100%)"
        format!("({:.0}%)", (percentage * 100.0).round())
    } else if max_text_length >= 4 {
        // 4 == "100%"
        format!("{:.0}%", (percentage * 100.0).round())
    } else {
        unreachable!("trying to render a rect of less than minimum size")
    }
}

pub fn tile_style(tile: &Tile, selected: bool) -> (Option<Style>, Style, Style) {
    let (background_style, first_line_style, second_line_style) = match (selected, &tile.file_type)
    {
        (true, FileType::File) => (
            Some(Style::default().fg(Color::Gray).bg(Color::Gray)),
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Gray)
                .add_modifier(Modifier::BOLD),
            Style::default()
                .fg(Color::Magenta)
                .bg(Color::Gray)
                .add_modifier(Modifier::BOLD),
        ),
        (false, FileType::File) => (None, Style::default(), Style::default()),
        (true, FileType::Folder) => (
            Some(Style::default().fg(Color::Blue).bg(Color::Blue)),
            Style::default()
                .fg(Color::White)
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            Style::default().fg(Color::Black).bg(Color::Blue),
        ),
        (false, FileType::Folder) => (
            None,
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
            Style::default(),
        ),
    };
    (background_style, first_line_style, second_line_style)
}

pub fn draw_rect_on_grid(buf: &mut Buffer, coords: (u16, u16), dimensions: (u16, u16)) {
    let (coords_x, coords_y) = coords;
    let (width, height) = dimensions;
    if width < 1 || height < 1 {
        return;
    }

    let buf_width = buf.area().width;
    let buf_height = buf.area().height;

    // top, bottom and corners
    for x in coords_x..=(coords_x + width) {
        if x >= buf_width {
            break;
        }
        if x == coords_x {
            if coords_y < buf_height {
                draw_next_symbol(buf, x, coords_y, boundaries::TOP_LEFT);
            }
            if coords_y + height < buf_height {
                draw_next_symbol(buf, x, coords_y + height, boundaries::BOTTOM_LEFT);
            }
        } else if x == coords_x + width {
            if coords_y < buf_height {
                draw_next_symbol(buf, x, coords_y, boundaries::TOP_RIGHT);
            }
            if coords_y + height < buf_height {
                draw_next_symbol(buf, x, coords_y + height, boundaries::BOTTOM_RIGHT);
            }
        } else {
            if coords_y < buf_height {
                draw_next_symbol(buf, x, coords_y, boundaries::HORIZONTAL);
            }
            if coords_y + height < buf_height {
                draw_next_symbol(buf, x, coords_y + height, boundaries::HORIZONTAL);
            }
        }
    }

    // left and right
    for y in (coords_y + 1)..(coords_y + height) {
        if y >= buf_height {
            break;
        }
        if coords_x < buf_width {
            draw_next_symbol(buf, coords_x, y, boundaries::VERTICAL);
        }
        if coords_x + width < buf_width {
            draw_next_symbol(buf, coords_x + width, y, boundaries::VERTICAL);
        }
    }
}

pub fn draw_filled_rect(buf: &mut Buffer, fill_style: Style, rect: Rect) {
    let buf_width = buf.area().width;
    let buf_height = buf.area().height;

    // fill
    for x in rect.x + 1..(rect.x + rect.width) {
        if x >= buf_width {
            break;
        }
        for y in rect.y + 1..(rect.y + rect.height) {
            if y >= buf_height {
                break;
            }
            let cell = &mut buf[(x, y)];
            cell.set_symbol(" ");
            cell.set_style(fill_style);
        }
    }

    // top and bottom
    for x in rect.x..=(rect.x + rect.width) {
        if x >= buf_width {
            break;
        }
        if x == rect.x {
            if rect.y < buf_height {
                buf[(x, rect.y)]
                    .set_symbol(boundaries::TOP_LEFT)
                    .set_style(fill_style);
            }
            if rect.y + rect.height < buf_height {
                buf[(x, rect.y + rect.height)]
                    .set_symbol(boundaries::BOTTOM_LEFT)
                    .set_style(fill_style);
            }
        } else if x == rect.x + rect.width {
            if rect.y < buf_height {
                buf[(x, rect.y)]
                    .set_symbol(boundaries::TOP_RIGHT)
                    .set_style(fill_style);
            }
            if rect.y + rect.height < buf_height {
                buf[(x, rect.y + rect.height)]
                    .set_symbol(boundaries::BOTTOM_RIGHT)
                    .set_style(fill_style);
            }
        } else {
            if rect.y < buf_height {
                buf[(x, rect.y)]
                    .set_symbol(boundaries::HORIZONTAL)
                    .set_style(fill_style);
            }
            if rect.y + rect.height < buf_height {
                buf[(x, rect.y + rect.height)]
                    .set_symbol(boundaries::HORIZONTAL)
                    .set_style(fill_style);
            }
        }
    }

    // left and right
    for y in (rect.y + 1)..(rect.y + rect.height) {
        if y >= buf_height {
            break;
        }
        if rect.x < buf_width {
            buf[(rect.x, y)]
                .set_symbol(boundaries::VERTICAL)
                .set_style(fill_style);
        }
        if rect.x + rect.width < buf_width {
            buf[(rect.x + rect.width, y)]
                .set_symbol(boundaries::VERTICAL)
                .set_style(fill_style);
        }
    }
}

pub fn draw_tile_text_on_grid(buf: &mut Buffer, tile: &Tile, selected: bool) {
    let buf_width = buf.area().width;
    let buf_height = buf.area().height;

    let first_line = tile_first_line(tile);
    let first_line_length = first_line.width() as u16;
    let first_line_start_position =
        (f64::from(tile.width - first_line_length) / 2.0).ceil() as u16 + tile.x;
    let second_line = tile_second_line(tile);
    let second_line_length = second_line.width();
    let second_line_start_position =
        (f64::from(tile.width - second_line_length as u16) / 2.0).ceil() as u16 + tile.x;
    let (background_style, first_line_style, second_line_style) = tile_style(tile, selected);

    if let Some(background_style) = background_style {
        for x in tile.x + 1..tile.x + tile.width {
            if x >= buf_width {
                break;
            }
            for y in tile.y + 1..tile.y + tile.height {
                if y >= buf_height {
                    break;
                }
                buf[(x, y)].set_symbol("█").set_style(background_style);
                // we set both the filling symbol and the style
                // because some terminals do not show this symbol on the one side
                // and our tests need it in order to pass on the other side
                // some terminals also don't have colors and would need this
                // as an indication so... best of all worlds!
            }
        }
    }

    if tile.height > 5 {
        let line_gap = if tile.height.is_multiple_of(2) { 1 } else { 2 };
        buf.set_string(
            first_line_start_position,
            (tile.height / 2) + tile.y - 1,
            first_line,
            first_line_style,
        );
        buf.set_string(
            second_line_start_position,
            (tile.height / 2) + tile.y + line_gap,
            second_line,
            second_line_style,
        );
    } else if tile.height == 5 {
        buf.set_string(
            first_line_start_position,
            (tile.height / 2) + tile.y,
            first_line,
            first_line_style,
        );
        buf.set_string(
            second_line_start_position,
            (tile.height / 2) + tile.y + 1,
            second_line,
            second_line_style,
        );
    } else if tile.height > 4 {
        buf.set_string(
            first_line_start_position,
            tile.y + 1,
            first_line,
            first_line_style,
        );
        buf.set_string(
            second_line_start_position,
            tile.y + 2,
            second_line,
            second_line_style,
        );
    } else if tile.height == 4 {
        buf.set_string(
            first_line_start_position,
            tile.y + 1,
            first_line,
            first_line_style,
        );
        buf.set_string(
            second_line_start_position,
            tile.y + 3,
            second_line,
            second_line_style,
        );
    } else if tile.height > 2 {
        buf.set_string(
            first_line_start_position,
            tile.y + 1,
            first_line,
            first_line_style,
        );
        buf.set_string(
            second_line_start_position,
            tile.y + 2,
            second_line,
            second_line_style,
        );
    } else {
        buf.set_string(
            first_line_start_position,
            tile.y + 1,
            first_line,
            first_line_style,
        );
    }
}
