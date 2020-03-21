use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Style, Color, Modifier};
use tui::symbols::line;
use tui::widgets::{Borders, Widget};

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

            if rect_with_text.rect.width > 0.0 && rect_with_text.rect.height > 0.0 {

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
                    // println!("\rrect {:?}", rect);
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

            }
        }

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
                    for x in (area.x + 1)..(area.x + area.width + 1) {
                        if x == area.x {
                            let current_symbol_top = &buf.get(x, area.y).symbol;
                            if current_symbol_top == &rect_boundary_chars.CROSS || current_symbol_top == &rect_boundary_chars.HORIZONTAL_DOWN {
                                // no-op
                            } else if current_symbol_top == &rect_boundary_chars.TOP_RIGHT || current_symbol_top == &rect_boundary_chars.HORIZONTAL {
                                buf.get_mut(x, area.y) // TODO: do not get twice?
                                    .set_symbol(&rect_boundary_chars.HORIZONTAL_DOWN);
                            } else if current_symbol_top == &rect_boundary_chars.BOTTOM_LEFT || current_symbol_top == &rect_boundary_chars.VERTICAL || current_symbol_top == &rect_boundary_chars.VERTICAL_RIGHT {
                                buf.get_mut(x, area.y) // TODO: do not get twice?
                                    .set_symbol(&rect_boundary_chars.VERTICAL_RIGHT);
                            } else if current_symbol_top == &rect_boundary_chars.HORIZONTAL_UP || current_symbol_top == &rect_boundary_chars.BOTTOM_LEFT || current_symbol_top == &rect_boundary_chars.VERTICAL_LEFT {
                                buf.get_mut(x, area.y) // TODO: do not get twice?
                                    .set_symbol(&rect_boundary_chars.CROSS);
                            } else {
                                buf.get_mut(x, area.y)
                                    .set_symbol(&rect_boundary_chars.TOP_LEFT);
                                    // .set_style(self.border_style);
                            }

                            let current_symbol_bottom = &buf.get(x, area.y + area.height).symbol;
                            if current_symbol_bottom == &rect_boundary_chars.BOTTOM_RIGHT || current_symbol_bottom == &rect_boundary_chars.HORIZONTAL {
                                buf.get_mut(x, area.y + area.height)
                                    .set_symbol(&rect_boundary_chars.HORIZONTAL_UP);
                            } else if current_symbol_bottom == &rect_boundary_chars.VERTICAL {
                                buf.get_mut(x, area.y + area.height)
                                    .set_symbol(&rect_boundary_chars.VERTICAL_RIGHT);
                            } else {
                                buf.get_mut(x, area.y + area.height)
                                    .set_symbol(&rect_boundary_chars.BOTTOM_LEFT);
                            }
                        } else if x == area.x + area.width {
                            let current_symbol_top = &buf.get(x, area.y).symbol;
                            if current_symbol_top == &rect_boundary_chars.CROSS {
                                // no-op
                            } else if current_symbol_top == &rect_boundary_chars.TOP_LEFT || current_symbol_top == &rect_boundary_chars.TOP_RIGHT || current_symbol_top == &rect_boundary_chars.HORIZONTAL {
                                buf.get_mut(x, area.y)
                                    .set_symbol(&rect_boundary_chars.HORIZONTAL_DOWN);
                            } else if current_symbol_top == &rect_boundary_chars.HORIZONTAL_UP {
                                buf.get_mut(x, area.y)
                                    .set_symbol(&rect_boundary_chars.CROSS);
                            } else if current_symbol_top == &rect_boundary_chars.BOTTOM_RIGHT {
                                buf.get_mut(x, area.y)
                                    .set_symbol(&rect_boundary_chars.VERTICAL_LEFT);
                            } else {
                                buf.get_mut(x, area.y)
                                    .set_symbol(&rect_boundary_chars.TOP_RIGHT);
                            }
                            let current_symbol_bottom = &buf.get(x, area.y + area.height).symbol;
                            if current_symbol_bottom == &rect_boundary_chars.BOTTOM_LEFT || current_symbol_bottom == &rect_boundary_chars.BOTTOM_RIGHT || current_symbol_bottom == &rect_boundary_chars.HORIZONTAL {
                                buf.get_mut(x, area.y + area.height)
                                    .set_symbol(&rect_boundary_chars.HORIZONTAL_UP);
                            } else {
                                buf.get_mut(x, area.y + area.height)
                                    .set_symbol(&rect_boundary_chars.BOTTOM_RIGHT);
                            }
                        } else {
                            let current_symbol_top = &buf.get(x, area.y).symbol;
                            if current_symbol_top == &rect_boundary_chars.CROSS || current_symbol_top == &rect_boundary_chars.HORIZONTAL_UP {
                                // no-op
                            } else if current_symbol_top == &rect_boundary_chars.TOP_LEFT || current_symbol_top == &rect_boundary_chars.TOP_RIGHT {
                                buf.get_mut(x, area.y)
                                    .set_symbol(&rect_boundary_chars.HORIZONTAL_DOWN);
                            } else if current_symbol_top == &rect_boundary_chars.BOTTOM_LEFT || current_symbol_top == &rect_boundary_chars.BOTTOM_RIGHT {
                                buf.get_mut(x, area.y)
                                    .set_symbol(&rect_boundary_chars.HORIZONTAL_UP);
                            } else if current_symbol_top == &rect_boundary_chars.VERTICAL {
                                buf.get_mut(x, area.y)
                                    .set_symbol(&rect_boundary_chars.CROSS);
                            } else {
                                buf.get_mut(x, area.y)
                                    .set_symbol(&rect_boundary_chars.HORIZONTAL);
                            }
                            let current_symbol_bottom = &buf.get(x, area.y + area.height).symbol;
                            if current_symbol_bottom == &rect_boundary_chars.BOTTOM_LEFT || current_symbol_bottom == &rect_boundary_chars.BOTTOM_RIGHT {
                                buf.get_mut(x, area.y + area.height)
                                    .set_symbol(&rect_boundary_chars.HORIZONTAL_UP);
                            } else if current_symbol_bottom == &rect_boundary_chars.VERTICAL {
                                buf.get_mut(x, area.y + area.height)
                                    .set_symbol(&rect_boundary_chars.CROSS);
                            } else {
                                buf.get_mut(x, area.y + area.height)
                                    .set_symbol(&rect_boundary_chars.HORIZONTAL);
                            }
                        }
                    }

                    // sides
                    for y in (area.y + 1)..(area.y + area.height) {
                        let current_symbol_left = &buf.get(area.x, y).symbol;
                        if current_symbol_left == &rect_boundary_chars.HORIZONTAL {
                            buf.get_mut(area.x, y)
                                .set_symbol(&rect_boundary_chars.CROSS);
                        } else {
                            buf.get_mut(area.x, y)
                                .set_symbol(&rect_boundary_chars.VERTICAL);
                        }
                        let current_symbol_right = &buf.get(area.x + area.width, y).symbol;
                        if current_symbol_right == &rect_boundary_chars.HORIZONTAL {
                            buf.get_mut(area.x + area.width, y)
                                .set_symbol(&rect_boundary_chars.CROSS);
                        } else {
                            buf.get_mut(area.x + area.width, y)
                                .set_symbol(&rect_boundary_chars.VERTICAL);
                        }
                    }
    }
}
