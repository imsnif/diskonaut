use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Style, Color};
use tui::symbols::line;
use tui::widgets::{Borders, Widget};

#[derive(Clone, Debug)]
pub struct RectWithText {
    pub rect: Rect,
    pub text: String,
    pub selected: bool,
    pub file_name: String, // TODO: better
}

#[derive(Clone)]
pub struct RectangleGrid {
    rectangles: Vec<RectWithText>
}

pub struct BoundariesToUse {
    pub TOP_RIGHT: String,
    pub VERTICAL: String,
    pub HORIZONTAL: String,
    pub TOP_LEFT: String,
    pub BOTTOM_RIGHT: String,
    pub BOTTOM_LEFT: String,
    pub VERTICAL_LEFT: String,
    pub VERTICAL_RIGHT: String,
    pub HORIZONTAL_DOWN: String,
    pub HORIZONTAL_UP: String,
    pub CROSS: String,
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


pub mod selected_boundaries {
    pub const TOP_RIGHT: &str = "╗";
    pub const VERTICAL: &str = "║";
    pub const HORIZONTAL: &str = "═";
    pub const TOP_LEFT: &str = "╔";
    pub const BOTTOM_RIGHT: &str = "╝";
    pub const BOTTOM_LEFT: &str = "╚";
    pub const VERTICAL_LEFT: &str = "╣";
    pub const VERTICAL_RIGHT: &str = "╠";
    pub const HORIZONTAL_DOWN: &str = "╦";
    pub const HORIZONTAL_UP: &str = "╩";
    pub const CROSS: &str = "╬";
}

impl<'a> RectangleGrid {
    pub fn new (rectangles: Vec<RectWithText>) -> Self {
        RectangleGrid { rectangles }
    }
}

fn truncate_middle(row: &str, max_length: u16) -> String {
    if max_length <= 6 {
        String::from(".") // TODO: make sure this never happens
//    } else if max_length == 4 {
//        String::from("[..]")
    } else if row.len() as u16 > max_length {
        let first_slice = &row[0..(max_length as usize / 2) - 2];
        let second_slice = &row[(row.len() - (max_length / 2) as usize + 2)..row.len()];
        format!("{}[..]{}", first_slice, second_slice)
    } else {
        row.to_string()
    }
}

impl<'a> Widget for RectangleGrid {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        if area.width < 2 || area.height < 2 {
            return;
        }
        for rect_with_text in &self.rectangles {

            let rect_boundary_chars = BoundariesToUse {
                TOP_RIGHT: String::from(boundaries::TOP_RIGHT),
                VERTICAL: String::from(boundaries::VERTICAL),
                HORIZONTAL: String::from(boundaries::HORIZONTAL),
                TOP_LEFT: String::from(boundaries::TOP_LEFT),
                BOTTOM_RIGHT: String::from(boundaries::BOTTOM_RIGHT),
                BOTTOM_LEFT: String::from(boundaries::BOTTOM_LEFT),
                VERTICAL_LEFT: String::from(boundaries::VERTICAL_LEFT),
                VERTICAL_RIGHT: String::from(boundaries::VERTICAL_RIGHT),
                HORIZONTAL_DOWN: String::from(boundaries::HORIZONTAL_DOWN),
                HORIZONTAL_UP: String::from(boundaries::HORIZONTAL_UP),
                CROSS: String::from(boundaries::CROSS),
            };
            let rect = rect_with_text.rect;
            let max_text_length = if rect.width > 4 { rect.width - 4 } else { 0 };
            // TODO: we should not accept a rectangle with a width of less than 8 so that the text
            // will be at least partly legible... these rectangles should be created with a small
            // height instead
            let display_text = truncate_middle(&rect_with_text.text, max_text_length);
            let text_length = display_text.len(); // TODO: better

            let text_start_position = ((rect.width - text_length as u16) as f64 / 2.0).ceil() as u16 + rect.x;




            let text_style = if rect_with_text.selected {
                Style::default().bg(Color::White).fg(Color::Black)
            } else {
                Style::default()
            };
            buf.set_string(text_start_position, rect.height / 2 + rect.y, display_text, text_style);

            for x in rect.x..(rect.x + rect.width + 1) {
                if x == rect.x {
                    let current_symbol_top = &buf.get(x, rect.y).symbol;
                    if current_symbol_top == &rect_boundary_chars.CROSS || current_symbol_top == &rect_boundary_chars.HORIZONTAL_DOWN {
                        // no-op
                    } else if current_symbol_top == &rect_boundary_chars.TOP_RIGHT || current_symbol_top == &rect_boundary_chars.HORIZONTAL {
                        buf.get_mut(x, rect.y) // TODO: do not get twice?
                            .set_symbol(&rect_boundary_chars.HORIZONTAL_DOWN);
                    } else if current_symbol_top == &rect_boundary_chars.BOTTOM_LEFT || current_symbol_top == &rect_boundary_chars.VERTICAL || current_symbol_top == &rect_boundary_chars.VERTICAL_RIGHT {
                        buf.get_mut(x, rect.y) // TODO: do not get twice?
                            .set_symbol(&rect_boundary_chars.VERTICAL_RIGHT);
                    } else if current_symbol_top == &rect_boundary_chars.HORIZONTAL_UP || current_symbol_top == &rect_boundary_chars.BOTTOM_LEFT || current_symbol_top == &rect_boundary_chars.VERTICAL_LEFT {
                        buf.get_mut(x, rect.y) // TODO: do not get twice?
                            .set_symbol(&rect_boundary_chars.CROSS);
                    } else {
                        buf.get_mut(x, rect.y)
                            .set_symbol(&rect_boundary_chars.TOP_LEFT);
                            // .set_style(self.border_style);
                    }

                    let current_symbol_bottom = &buf.get(x, rect.y + rect.height).symbol;
                    if current_symbol_bottom == &rect_boundary_chars.BOTTOM_RIGHT || current_symbol_bottom == &rect_boundary_chars.HORIZONTAL {
                        buf.get_mut(x, rect.y + rect.height)
                            .set_symbol(&rect_boundary_chars.HORIZONTAL_UP);
                    } else if current_symbol_bottom == &rect_boundary_chars.VERTICAL {
                        buf.get_mut(x, rect.y + rect.height)
                            .set_symbol(&rect_boundary_chars.VERTICAL_RIGHT);
                    } else {
                        buf.get_mut(x, rect.y + rect.height)
                            .set_symbol(&rect_boundary_chars.BOTTOM_LEFT);
                    }
                } else if x == rect.x + rect.width {
                    let current_symbol_top = &buf.get(x, rect.y).symbol;
                    if current_symbol_top == &rect_boundary_chars.CROSS {
                        // no-op
                    } else if current_symbol_top == &rect_boundary_chars.TOP_LEFT || current_symbol_top == &rect_boundary_chars.TOP_RIGHT || current_symbol_top == &rect_boundary_chars.HORIZONTAL {
                        buf.get_mut(x, rect.y)
                            .set_symbol(&rect_boundary_chars.HORIZONTAL_DOWN);
                    } else if current_symbol_top == &rect_boundary_chars.HORIZONTAL_UP {
                        buf.get_mut(x, rect.y)
                            .set_symbol(&rect_boundary_chars.CROSS);
                    } else if current_symbol_top == &rect_boundary_chars.BOTTOM_RIGHT {
                        buf.get_mut(x, rect.y)
                            .set_symbol(&rect_boundary_chars.VERTICAL_LEFT);
                    } else {
                        buf.get_mut(x, rect.y)
                            .set_symbol(&rect_boundary_chars.TOP_RIGHT);
                    }
                    let current_symbol_bottom = &buf.get(x, rect.y + rect.height).symbol;
                    if current_symbol_bottom == &rect_boundary_chars.BOTTOM_LEFT || current_symbol_bottom == &rect_boundary_chars.BOTTOM_RIGHT || current_symbol_bottom == &rect_boundary_chars.HORIZONTAL {
                        buf.get_mut(x, rect.y + rect.height)
                            .set_symbol(&rect_boundary_chars.HORIZONTAL_UP);
                    } else {
                        buf.get_mut(x, rect.y + rect.height)
                            .set_symbol(&rect_boundary_chars.BOTTOM_RIGHT);
                    }
                } else {
                    let current_symbol_top = &buf.get(x, rect.y).symbol;
                    if current_symbol_top == &rect_boundary_chars.CROSS || current_symbol_top == &rect_boundary_chars.HORIZONTAL_UP {
                        // no-op
                    } else if current_symbol_top == &rect_boundary_chars.TOP_LEFT || current_symbol_top == &rect_boundary_chars.TOP_RIGHT {
                        buf.get_mut(x, rect.y)
                            .set_symbol(&rect_boundary_chars.HORIZONTAL_DOWN);
                    } else if current_symbol_top == &rect_boundary_chars.BOTTOM_LEFT || current_symbol_top == &rect_boundary_chars.BOTTOM_RIGHT {
                        buf.get_mut(x, rect.y)
                            .set_symbol(&rect_boundary_chars.HORIZONTAL_UP);
                    } else if current_symbol_top == &rect_boundary_chars.VERTICAL {
                        buf.get_mut(x, rect.y)
                            .set_symbol(&rect_boundary_chars.CROSS);
                    } else {
                        buf.get_mut(x, rect.y)
                            .set_symbol(&rect_boundary_chars.HORIZONTAL);
                    }
                    let current_symbol_bottom = &buf.get(x, rect.y + rect.height).symbol;
                    if current_symbol_bottom == &rect_boundary_chars.BOTTOM_LEFT || current_symbol_bottom == &rect_boundary_chars.BOTTOM_RIGHT {
                        buf.get_mut(x, rect.y + rect.height)
                            .set_symbol(&rect_boundary_chars.HORIZONTAL_UP);
                    } else if current_symbol_bottom == &rect_boundary_chars.VERTICAL {
                        buf.get_mut(x, rect.y + rect.height)
                            .set_symbol(&rect_boundary_chars.CROSS);
                    } else {
                        buf.get_mut(x, rect.y + rect.height)
                            .set_symbol(&rect_boundary_chars.HORIZONTAL);
                    }
                }
            }

            // sides
            for y in (rect.y + 1)..(rect.y + rect.height) {
                let current_symbol_left = &buf.get(rect.x, y).symbol;
                if current_symbol_left == &rect_boundary_chars.HORIZONTAL {
                    buf.get_mut(rect.x, y)
                        .set_symbol(&rect_boundary_chars.CROSS);
                } else {
                    buf.get_mut(rect.x, y)
                        .set_symbol(&rect_boundary_chars.VERTICAL);
                }
                let current_symbol_right = &buf.get(rect.x + rect.width, y).symbol;
                if current_symbol_right == &rect_boundary_chars.HORIZONTAL {
                    buf.get_mut(rect.x + rect.width, y)
                        .set_symbol(&rect_boundary_chars.CROSS);
                } else {
                    buf.get_mut(rect.x + rect.width, y)
                        .set_symbol(&rect_boundary_chars.VERTICAL);
                }
            }

        }

//        self.background(area, buf, self.style.bg);
//
//        // Sides
//        if self.borders.intersects(Borders::LEFT) {
//            for y in area.top()..area.bottom() {
//                buf.get_mut(area.left(), y)
//                    .set_symbol(line::VERTICAL)
//                    .set_style(self.border_style);
//            }
//        }
//        if self.borders.intersects(Borders::TOP) {
//            for x in area.left()..area.right() {
//                buf.get_mut(x, area.top())
//                    .set_symbol(line::HORIZONTAL)
//                    .set_style(self.border_style);
//            }
//        }
//        if self.borders.intersects(Borders::RIGHT) {
//            let x = area.right() - 1;
//            for y in area.top()..area.bottom() {
//                buf.get_mut(x, y)
//                    .set_symbol(line::VERTICAL)
//                    .set_style(self.border_style);
//            }
//        }
//        if self.borders.intersects(Borders::BOTTOM) {
//            let y = area.bottom() - 1;
//            for x in area.left()..area.right() {
//                buf.get_mut(x, y)
//                    .set_symbol(line::HORIZONTAL)
//                    .set_style(self.border_style);
//            }
//        }
//
//        // Corners
//        if self.borders.contains(Borders::LEFT | Borders::TOP) {
//            buf.get_mut(area.left(), area.top())
//                .set_symbol(line::TOP_LEFT)
//                .set_style(self.border_style);
//        }
//        if self.borders.contains(Borders::RIGHT | Borders::TOP) {
//            buf.get_mut(area.right() - 1, area.top())
//                .set_symbol(line::TOP_RIGHT)
//                .set_style(self.border_style);
//        }
//        if self.borders.contains(Borders::LEFT | Borders::BOTTOM) {
//            buf.get_mut(area.left(), area.bottom() - 1)
//                .set_symbol(line::BOTTOM_LEFT)
//                .set_style(self.border_style);
//        }
//        if self.borders.contains(Borders::RIGHT | Borders::BOTTOM) {
//            buf.get_mut(area.right() - 1, area.bottom() - 1)
//                .set_symbol(line::BOTTOM_RIGHT)
//                .set_style(self.border_style);
//        }
//
//        if area.width > 2 {
//            if let Some(title) = self.title {
//                let lx = if self.borders.intersects(Borders::LEFT) {
//                    1
//                } else {
//                    0
//                };
//                let rx = if self.borders.intersects(Borders::RIGHT) {
//                    1
//                } else {
//                    0
//                };
//                let width = area.width - lx - rx;
//                buf.set_stringn(
//                    area.left() + lx,
//                    area.top(),
//                    title,
//                    width as usize,
//                    self.title_style,
//                );
//            }
//        }
    }
}
