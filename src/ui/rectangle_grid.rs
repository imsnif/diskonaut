use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Style, Color};
use tui::widgets::{Widget};

#[derive(Clone, Debug)]
pub struct RectWithText {
    pub rect: RectFloat,
    pub text: String,
    pub selected: bool,
    pub file_name: String, // TODO: better
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
    rectangles: Vec<RectWithText>
}

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

impl<'a> RectangleGrid {
    pub fn new (rectangles: Vec<RectWithText>) -> Self {
        RectangleGrid { rectangles }
    }
}

fn truncate_middle(row: &str, max_length: u16) -> String {
    if max_length <= 6 {
        String::from(".") // TODO: make sure this never happens
    } else if row.len() as u16 > max_length {
        let first_slice = &row[0..(max_length as usize / 2) - 2];
        let second_slice = &row[(row.len() - (max_length / 2) as usize + 2)..row.len()];
        format!("{}[..]{}", first_slice, second_slice)
    } else {
        row.to_string()
    }
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

        (boundaries::VERTICAL_RIGHT, boundaries::VERTICAL_RIGHT) => Some(boundaries::VERTICAL_RIGHT), // (├, ├) => Some(├)
        (boundaries::VERTICAL_RIGHT, boundaries::HORIZONTAL_DOWN) => Some(boundaries::CROSS), // (├, ┬) => Some(┼)
        (boundaries::VERTICAL_RIGHT, boundaries::HORIZONTAL_UP) => Some(boundaries::CROSS), // (├, ┴) => Some(┼)
        (boundaries::VERTICAL_RIGHT, boundaries::CROSS) => Some(boundaries::CROSS), // (├, ┼) => Some(┼)

        (boundaries::HORIZONTAL_DOWN, boundaries::HORIZONTAL_DOWN) => Some(boundaries::HORIZONTAL_DOWN), // (┬, ┬) => Some(┬)
        (boundaries::HORIZONTAL_DOWN, boundaries::HORIZONTAL_UP) => Some(boundaries::CROSS), // (┬, ┴) => Some(┼)
        (boundaries::HORIZONTAL_DOWN, boundaries::CROSS) => Some(boundaries::CROSS), // (┬, ┼) => Some(┼)

        (boundaries::HORIZONTAL_UP, boundaries::HORIZONTAL_UP) => Some(boundaries::HORIZONTAL_UP), // (┴, ┴) => Some(┬)
        (boundaries::HORIZONTAL_UP, boundaries::CROSS) => Some(boundaries::CROSS), // (┴, ┼) => Some(┼)

        (boundaries::CROSS, boundaries::CROSS) => Some(boundaries::CROSS), // (┼, ┼) => Some(┼)

        (_, _) => None
    }
}

fn find_next_symbol (first_symbol: &str, second_symbol: &str) -> Option<&'static str> {
    if let Some(symbol) = combine_symbols(first_symbol, second_symbol) {
        Some(symbol)
    } else {
        combine_symbols(second_symbol, first_symbol)
    }
}

fn set_symbol_on_grid(buf: &mut Buffer, x: u16, y: u16, symbol: &str) {
    if let Some(next_symbol) = find_next_symbol(&buf.get(x, y).symbol, symbol) {
        buf.get_mut(x, y).set_symbol(next_symbol);
    } else {
        buf.get_mut(x, y).set_symbol(symbol);
    }
}

fn draw_rect_on_grid (buf: &mut Buffer, rect: Rect) {
    // top and bottom
    for x in rect.x..(rect.x + rect.width + 1) {
        if x == rect.x {
            set_symbol_on_grid(buf, x, rect.y, &boundaries::TOP_LEFT);
            set_symbol_on_grid(buf, x, rect.y + rect.height, &boundaries::BOTTOM_LEFT);
        } else if x == rect.x + rect.width {
            set_symbol_on_grid(buf, x, rect.y, &boundaries::TOP_RIGHT);
            set_symbol_on_grid(buf, x, rect.y + rect.height, &boundaries::BOTTOM_RIGHT);
        } else {
            set_symbol_on_grid(buf, x, rect.y, &boundaries::HORIZONTAL);
            set_symbol_on_grid(buf, x, rect.y + rect.height, &boundaries::HORIZONTAL);
        }
    }

    // left and right
    for y in (rect.y + 1)..(rect.y + rect.height) {
        set_symbol_on_grid(buf, rect.x, y, &boundaries::VERTICAL);
        set_symbol_on_grid(buf, rect.x + rect.width, y, &boundaries::VERTICAL);
    }
}

impl<'a> Widget for RectangleGrid {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        if area.width < 2 || area.height < 2 {
            return;
        }
        for rect_with_text in &self.rectangles {
            let rounded_x = rect_with_text.rect.x.round();
            let rounded_y = rect_with_text.rect.y.round();
            let mut rect = Rect {
                x: rounded_x as u16,
                y: rounded_y  as u16,
                width: ((rect_with_text.rect.x - rounded_x) + rect_with_text.rect.width).round() as u16,
                height: ((rect_with_text.rect.y - rounded_y) + rect_with_text.rect.height).round() as u16,
            };

            // fix rounding errors
            if (rect_with_text.rect.x + rect_with_text.rect.width).round() as u16 > rect.x + rect.width {
                rect.width += 1;
            }
            if (rect_with_text.rect.y + rect_with_text.rect.height).round() as u16 > rect.y + rect.height {
                rect.height += 1;
            }

            if rect.height < 2 || rect.width < 8 {
                for x in rect.x..(rect.x + rect.width + 1) {
                    if x > rect.x {
                        for y in rect.y..(rect.y + rect.height + 1) {
                            if y > rect.y {
                                let buf = buf.get_mut(x, y);
                                buf.set_symbol("▒");
                            }
                        }
                    }
                }
            } else {
                let max_text_length = if rect.width > 4 { rect.width - 4 } else { 0 };
                // TODO: we should not accept a rectangle with a width of less than 8 so that the text
                // will be at least partly legible... these rectangles should be created with a small
                // height instead
                let text = if rect_with_text.selected { format!("=> {} <=", rect_with_text.text) } else { rect_with_text.text.to_owned() }; // TODO: better
                let display_text = truncate_middle(&text, max_text_length);
                let text_length = display_text.len(); // TODO: better

                let text_start_position = ((rect.width - text_length as u16) as f64 / 2.0).ceil() as u16 + rect.x;

                let text_style = if rect_with_text.selected {
                    Style::default().bg(Color::White).fg(Color::Black)
                } else {
                    Style::default()
                };
                buf.set_string(text_start_position, rect.height / 2 + rect.y, display_text, text_style);
                draw_rect_on_grid(buf, rect);
            }
        }
        draw_rect_on_grid(buf, area); // we draw a frame around the whole area to make up for the "small files" block not having a frame of its own
    }
}
