use ::tui::buffer::Buffer;
use ::tui::style::{Style, Color, Modifier};
use ::tui::layout::Rect;

use crate::state::FileType;
use crate::ui::{draw_next_symbol, boundaries};
use crate::ui::format::{DisplaySize, DisplaySizeRounded, truncate_middle};
use crate::state::Tile;

fn tile_first_line (tile: &Tile, selected: bool) -> String {
    let max_text_length = if tile.width > 2 { tile.width - 2 } else { 0 };
    let name = &tile.name.to_string_lossy();
    let descendant_count = &tile.descendants;
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
    first_line
}

fn tile_second_line(tile: &Tile) -> String {
    let max_text_length = if tile.width > 2 { tile.width - 2 } else { 0 };
    let percentage = &tile.percentage;
    let display_size = DisplaySize(tile.size as f64);
    let display_size_rounded = DisplaySizeRounded(tile.size as f64);
    let display_size = format!("{}", display_size);
    let display_size_rounded = format!("{}", display_size_rounded);
    if max_text_length >= display_size.len() as u16 + 7 { // 7 == "(100%)" + 1 space
        format!("{} ({:.0}%)", display_size, percentage * 100.0)
    } else if max_text_length > display_size.len() as u16 {
        display_size
    } else if max_text_length > display_size_rounded.len() as u16 {
        display_size_rounded
    } else if max_text_length > 6 { // 6 == "(100%)"
        format!("({:.0}%)", (percentage * 100.0).round())
    } else if max_text_length >= 4 { // 4 == "100%"
        format!("{:.0}%", (percentage * 100.0).round())
    } else {
        unreachable!("trying to render a rect of less than minimum size")
    }
}

pub fn tile_style (tile: &Tile, selected: bool) -> (Option<Style>, Style, Style ){
    let (background_style, first_line_style, second_line_style) = match ( selected, &tile.file_type ) {
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
    (background_style, first_line_style, second_line_style)
}

pub fn draw_rect_on_grid (buf: &mut Buffer, coords: (u16, u16), dimensions: (u16, u16)) {
    let (coords_x, coords_y) = coords;
    let (width, height) = dimensions;

    // top, bottom and corners
    for x in coords_x..(coords_x + width + 1) {
        if x == coords_x {
            draw_next_symbol(buf, x, coords_y, &boundaries::TOP_LEFT);
            draw_next_symbol(buf, x, coords_y + height, &boundaries::BOTTOM_LEFT);
        } else if x == coords_x + width {
            draw_next_symbol(buf, x, coords_y, &boundaries::TOP_RIGHT);
            draw_next_symbol(buf, x, coords_y + height, &boundaries::BOTTOM_RIGHT);
        } else {
            draw_next_symbol(buf, x, coords_y, &boundaries::HORIZONTAL);
            draw_next_symbol(buf, x, coords_y + height, &boundaries::HORIZONTAL);
        }
    }

    // left and right
    for y in (coords_y + 1)..(coords_y + height) {
        draw_next_symbol(buf, coords_x, y, &boundaries::VERTICAL);
        draw_next_symbol(buf, coords_x + width, y, &boundaries::VERTICAL);
    }
}

pub fn draw_filled_rect(buf: &mut Buffer, fill_style: Style, rect: &Rect) {
    // fill
    for x in rect.x + 1..(rect.x + rect.width) {
        for y in rect.y + 1..(rect.y + rect.height) {
            let cell = buf.get_mut(x, y);
            cell.set_symbol(" ");
            cell.set_style(fill_style);
        }
    }

    // top and bottom
    for x in rect.x..(rect.x + rect.width + 1) {
        if x == rect.x {
            buf.get_mut(x, rect.y).set_symbol(&boundaries::TOP_LEFT).set_style(fill_style);
            buf.get_mut(x, rect.y + rect.height).set_symbol(&boundaries::BOTTOM_LEFT).set_style(fill_style);
        } else if x == rect.x + rect.width {
            buf.get_mut(x, rect.y).set_symbol(&boundaries::TOP_RIGHT).set_style(fill_style);
            buf.get_mut(x, rect.y + rect.height).set_symbol(&boundaries::BOTTOM_RIGHT).set_style(fill_style);
        } else {
            buf.get_mut(x, rect.y).set_symbol(&boundaries::HORIZONTAL).set_style(fill_style);
            buf.get_mut(x, rect.y + rect.height).set_symbol(&boundaries::HORIZONTAL).set_style(fill_style);
        }
    }

    // left and right
    for y in (rect.y + 1)..(rect.y + rect.height) {
        buf.get_mut(rect.x, y).set_symbol(&boundaries::VERTICAL).set_style(fill_style);
        buf.get_mut(rect.x + rect.width, y).set_symbol(&boundaries::VERTICAL).set_style(fill_style);
    }
}

pub fn draw_tile_text_on_grid(buf: &mut Buffer, tile: &Tile, selected: bool) { // TODO: better, combine args
    let first_line = tile_first_line(&tile, selected);
    let first_line_length = first_line.chars().count() as u16;
    let first_line_start_position = ((tile.width - first_line_length) as f64 / 2.0).ceil() as u16 + tile.x;
    let second_line = tile_second_line(&tile);
    let second_line_length = second_line.chars().count();
    let second_line_start_position = ((tile.width - second_line_length as u16) as f64 / 2.0).ceil() as u16 + tile.x;
    let (background_style, first_line_style, second_line_style) = tile_style(&tile, selected);

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
