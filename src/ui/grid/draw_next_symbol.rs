use ::tui::buffer::Buffer;

pub mod boundaries {
    pub const TOP_RIGHT: &str = "┐";
    pub const VERTICAL: &str = "│";
    pub const HORIZONTAL: &str = "─";
    pub const TOP_LEFT: &str = "┌";
    pub const BOTTOM_RIGHT: &str = "┘";
    pub const BOTTOM_LEFT: &str = "└";
    pub const VERTICAL_LEFT: &str = "┤";
    pub const VERTICAL_RIGHT: &str = "├";
    pub const HORIZONTAL_DOWN: &str = "┬";
    pub const HORIZONTAL_UP: &str = "┴";
    pub const CROSS: &str = "┼";
}

fn combine_symbols(current_symbol: &str, next_symbol: &str) -> Option<&'static str> {
    match (current_symbol, next_symbol) {
        (boundaries::TOP_RIGHT, boundaries::TOP_RIGHT) => Some(boundaries::TOP_RIGHT), // (┐, ┐) => Some(┐)
        (boundaries::TOP_RIGHT, boundaries::VERTICAL) => Some(boundaries::VERTICAL_LEFT), // (┐, │) => Some(┤)
        (boundaries::TOP_RIGHT, boundaries::HORIZONTAL) => Some(boundaries::HORIZONTAL_DOWN), // (┐, ─) => Some(┬)
        (boundaries::TOP_RIGHT, boundaries::TOP_LEFT) => Some(boundaries::HORIZONTAL_DOWN), // (┐, ┌) => Some(┬)
        (boundaries::TOP_RIGHT, boundaries::BOTTOM_RIGHT) => Some(boundaries::VERTICAL_LEFT), // (┐, ┘) => Some(┤)
        (boundaries::TOP_RIGHT, boundaries::BOTTOM_LEFT) => Some(boundaries::CROSS), // (┐, └) => Some(┼)
        (boundaries::TOP_RIGHT, boundaries::VERTICAL_LEFT) => Some(boundaries::VERTICAL_LEFT), // (┐, ┤) => Some(┤)
        (boundaries::TOP_RIGHT, boundaries::VERTICAL_RIGHT) => Some(boundaries::CROSS), // (┐, ├) => Some(┼)
        (boundaries::TOP_RIGHT, boundaries::HORIZONTAL_DOWN) => Some(boundaries::HORIZONTAL_DOWN), // (┐, ┬) => Some(┬)
        (boundaries::TOP_RIGHT, boundaries::HORIZONTAL_UP) => Some(boundaries::CROSS), // (┐, ┴) => Some(┼)
        (boundaries::TOP_RIGHT, boundaries::CROSS) => Some(boundaries::CROSS), // (┐, ┼) => Some(┼)

        (boundaries::HORIZONTAL, boundaries::HORIZONTAL) => Some(boundaries::HORIZONTAL), // (─, ─) => Some(─)
        (boundaries::HORIZONTAL, boundaries::VERTICAL) => Some(boundaries::CROSS), // (─, │) => Some(┼)
        (boundaries::HORIZONTAL, boundaries::TOP_LEFT) => Some(boundaries::HORIZONTAL_DOWN), // (─, ┌) => Some(┬)
        (boundaries::HORIZONTAL, boundaries::BOTTOM_RIGHT) => Some(boundaries::HORIZONTAL_UP), // (─, ┘) => Some(┴)
        (boundaries::HORIZONTAL, boundaries::BOTTOM_LEFT) => Some(boundaries::HORIZONTAL_UP), // (─, └) => Some(┴)
        (boundaries::HORIZONTAL, boundaries::VERTICAL_LEFT) => Some(boundaries::CROSS), // (─, ┤) => Some(┼)
        (boundaries::HORIZONTAL, boundaries::VERTICAL_RIGHT) => Some(boundaries::CROSS), // (─, ├) => Some(┼)
        (boundaries::HORIZONTAL, boundaries::HORIZONTAL_DOWN) => Some(boundaries::HORIZONTAL_DOWN), // (─, ┬) => Some(┬)
        (boundaries::HORIZONTAL, boundaries::HORIZONTAL_UP) => Some(boundaries::HORIZONTAL_UP), // (─, ┴) => Some(┴)
        (boundaries::HORIZONTAL, boundaries::CROSS) => Some(boundaries::CROSS), // (─, ┼) => Some(┼)

        (boundaries::VERTICAL, boundaries::VERTICAL) => Some(boundaries::VERTICAL), // (│, │) => Some(│)
        (boundaries::VERTICAL, boundaries::TOP_LEFT) => Some(boundaries::VERTICAL_RIGHT), // (│, ┌) => Some(├)
        (boundaries::VERTICAL, boundaries::BOTTOM_RIGHT) => Some(boundaries::VERTICAL_LEFT), // (│, ┘) => Some(┤)
        (boundaries::VERTICAL, boundaries::BOTTOM_LEFT) => Some(boundaries::VERTICAL_RIGHT), // (│, └) => Some(├)
        (boundaries::VERTICAL, boundaries::VERTICAL_LEFT) => Some(boundaries::VERTICAL_LEFT), // (│, ┤) => Some(┤)
        (boundaries::VERTICAL, boundaries::VERTICAL_RIGHT) => Some(boundaries::VERTICAL_RIGHT), // (│, ├) => Some(├)
        (boundaries::VERTICAL, boundaries::HORIZONTAL_DOWN) => Some(boundaries::CROSS), // (│, ┬) => Some(┼)
        (boundaries::VERTICAL, boundaries::HORIZONTAL_UP) => Some(boundaries::CROSS), // (│, ┴) => Some(┼)
        (boundaries::VERTICAL, boundaries::CROSS) => Some(boundaries::CROSS), // (│, ┼) => Some(┼)

        (boundaries::TOP_LEFT, boundaries::TOP_LEFT) => Some(boundaries::TOP_LEFT), // (┌, ┌) => Some(┌)
        (boundaries::TOP_LEFT, boundaries::BOTTOM_RIGHT) => Some(boundaries::CROSS), // (┌, ┘) => Some(┼)
        (boundaries::TOP_LEFT, boundaries::BOTTOM_LEFT) => Some(boundaries::VERTICAL_RIGHT), // (┌, └) => Some(├)
        (boundaries::TOP_LEFT, boundaries::VERTICAL_LEFT) => Some(boundaries::CROSS), // (┌, ┤) => Some(┼)
        (boundaries::TOP_LEFT, boundaries::VERTICAL_RIGHT) => Some(boundaries::VERTICAL_RIGHT), // (┌, ├) => Some(├)
        (boundaries::TOP_LEFT, boundaries::HORIZONTAL_DOWN) => Some(boundaries::HORIZONTAL_DOWN), // (┌, ┬) => Some(┬)
        (boundaries::TOP_LEFT, boundaries::HORIZONTAL_UP) => Some(boundaries::CROSS), // (┌, ┴) => Some(┼)
        (boundaries::TOP_LEFT, boundaries::CROSS) => Some(boundaries::CROSS), // (┌, ┼) => Some(┼)

        (boundaries::BOTTOM_RIGHT, boundaries::BOTTOM_RIGHT) => Some(boundaries::BOTTOM_RIGHT), // (┘, ┘) => Some(┘)
        (boundaries::BOTTOM_RIGHT, boundaries::BOTTOM_LEFT) => Some(boundaries::HORIZONTAL_UP), // (┘, └) => Some(┴)
        (boundaries::BOTTOM_RIGHT, boundaries::VERTICAL_LEFT) => Some(boundaries::VERTICAL_LEFT), // (┘, ┤) => Some(┤)
        (boundaries::BOTTOM_RIGHT, boundaries::VERTICAL_RIGHT) => Some(boundaries::CROSS), // (┘, ├) => Some(┼)
        (boundaries::BOTTOM_RIGHT, boundaries::HORIZONTAL_DOWN) => Some(boundaries::CROSS), // (┘, ┬) => Some(┼)
        (boundaries::BOTTOM_RIGHT, boundaries::HORIZONTAL_UP) => Some(boundaries::HORIZONTAL_UP), // (┘, ┴) => Some(┴)
        (boundaries::BOTTOM_RIGHT, boundaries::CROSS) => Some(boundaries::CROSS), // (┘, ┼) => Some(┼)

        (boundaries::BOTTOM_LEFT, boundaries::BOTTOM_LEFT) => Some(boundaries::BOTTOM_LEFT), // (└, └) => Some(└)
        (boundaries::BOTTOM_LEFT, boundaries::VERTICAL_LEFT) => Some(boundaries::CROSS), // (└, ┤) => Some(┼)
        (boundaries::BOTTOM_LEFT, boundaries::VERTICAL_RIGHT) => Some(boundaries::VERTICAL_RIGHT), // (└, ├) => Some(├)
        (boundaries::BOTTOM_LEFT, boundaries::HORIZONTAL_DOWN) => Some(boundaries::CROSS), // (└, ┬) => Some(┼)
        (boundaries::BOTTOM_LEFT, boundaries::HORIZONTAL_UP) => Some(boundaries::HORIZONTAL_UP), // (└, ┴) => Some(┴)
        (boundaries::BOTTOM_LEFT, boundaries::CROSS) => Some(boundaries::CROSS), // (└, ┼) => Some(┼)

        (boundaries::VERTICAL_LEFT, boundaries::VERTICAL_LEFT) => Some(boundaries::VERTICAL_LEFT), // (┤, ┤) => Some(┤)
        (boundaries::VERTICAL_LEFT, boundaries::VERTICAL_RIGHT) => Some(boundaries::CROSS), // (┤, ├) => Some(┼)
        (boundaries::VERTICAL_LEFT, boundaries::HORIZONTAL_DOWN) => Some(boundaries::CROSS), // (┤, ┬) => Some(┼)
        (boundaries::VERTICAL_LEFT, boundaries::HORIZONTAL_UP) => Some(boundaries::HORIZONTAL_UP), // (┤, ┴) => Some(┼)
        (boundaries::VERTICAL_LEFT, boundaries::CROSS) => Some(boundaries::CROSS), // (┤, ┼) => Some(┼)

        (boundaries::VERTICAL_RIGHT, boundaries::VERTICAL_RIGHT) => {
            Some(boundaries::VERTICAL_RIGHT)
        } // (├, ├) => Some(├)
        (boundaries::VERTICAL_RIGHT, boundaries::HORIZONTAL_DOWN) => Some(boundaries::CROSS), // (├, ┬) => Some(┼)
        (boundaries::VERTICAL_RIGHT, boundaries::HORIZONTAL_UP) => Some(boundaries::CROSS), // (├, ┴) => Some(┼)
        (boundaries::VERTICAL_RIGHT, boundaries::CROSS) => Some(boundaries::CROSS), // (├, ┼) => Some(┼)

        (boundaries::HORIZONTAL_DOWN, boundaries::HORIZONTAL_DOWN) => {
            Some(boundaries::HORIZONTAL_DOWN)
        } // (┬, ┬) => Some(┬)
        (boundaries::HORIZONTAL_DOWN, boundaries::HORIZONTAL_UP) => Some(boundaries::CROSS), // (┬, ┴) => Some(┼)
        (boundaries::HORIZONTAL_DOWN, boundaries::CROSS) => Some(boundaries::CROSS), // (┬, ┼) => Some(┼)

        (boundaries::HORIZONTAL_UP, boundaries::HORIZONTAL_UP) => Some(boundaries::HORIZONTAL_UP), // (┴, ┴) => Some(┬)
        (boundaries::HORIZONTAL_UP, boundaries::CROSS) => Some(boundaries::CROSS), // (┴, ┼) => Some(┼)

        (boundaries::CROSS, boundaries::CROSS) => Some(boundaries::CROSS), // (┼, ┼) => Some(┼)

        (_, _) => None,
    }
}

fn find_next_symbol(first_symbol: &str, second_symbol: &str) -> Option<&'static str> {
    if let Some(symbol) = combine_symbols(first_symbol, second_symbol) {
        Some(symbol)
    } else {
        combine_symbols(second_symbol, first_symbol)
    }
}

pub fn draw_next_symbol(buf: &mut Buffer, x: u16, y: u16, symbol: &str) {
    if let Some(next_symbol) = find_next_symbol(buf.get(x, y).symbol(), symbol) {
        buf.get_mut(x, y).set_symbol(next_symbol);
    } else {
        buf.get_mut(x, y).set_symbol(symbol);
    }
}
