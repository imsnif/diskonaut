use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Style, Color, Modifier};
use tui::widgets::{Widget};

use crate::ui::{draw_symbol_with_style, boundaries};
use crate::ui::format::truncate_end;

pub struct ErrorBox<'a> {
    error_message: &'a str 
}

impl <'a>ErrorBox <'a>{
    pub fn new (error_message: &'a str) -> Self {
        Self { error_message }
    }
}

// TODO: merge with identical function elsewhere
fn draw_rect_on_grid (buf: &mut Buffer, rect: Rect) {
    // top and bottom
    for x in rect.x..(rect.x + rect.width + 1) {
        if x == rect.x {
            draw_symbol_with_style(buf, x, rect.y, &boundaries::TOP_LEFT, Style::default().bg(Color::Black));
            draw_symbol_with_style(buf, x, rect.y + rect.height, &boundaries::BOTTOM_LEFT, Style::default().bg(Color::Black));
        } else if x == rect.x + rect.width {
            draw_symbol_with_style(buf, x, rect.y, &boundaries::TOP_RIGHT, Style::default().bg(Color::Black));
            draw_symbol_with_style(buf, x, rect.y + rect.height, &boundaries::BOTTOM_RIGHT, Style::default().bg(Color::Black));
        } else {
            draw_symbol_with_style(buf, x, rect.y, &boundaries::HORIZONTAL, Style::default().bg(Color::Black));
            draw_symbol_with_style(buf, x, rect.y + rect.height, &boundaries::HORIZONTAL, Style::default().bg(Color::Black));
        }
    }

    // left and right
    for y in (rect.y + 1)..(rect.y + rect.height) {
        draw_symbol_with_style(buf, rect.x, y, &boundaries::VERTICAL, Style::default().bg(Color::Black));
        draw_symbol_with_style(buf, rect.x + rect.width, y, &boundaries::VERTICAL, Style::default().bg(Color::Black));
    }
}

fn fill_rect(rect: &Rect, style: Style, buf: &mut Buffer) {
    for x in rect.x + 1..(rect.x + rect.width) {
        for y in rect.y + 1..(rect.y + rect.height) {
            let cell = buf.get_mut(x, y);
            cell.set_symbol(" ");
            cell.set_style(style);
        }
    }
}

impl <'a>Widget for ErrorBox<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
         let (width, height) = if area.width > 150 {
             (150, 10)
         } else if area.width > 50 {
             (area.width / 2, 10)
         } else {
             unreachable!("app should not be rendered if window is so small")
         };

        // position self in the middle of the rect
        let x = ((area.x + area.width) / 2) - width / 2;
        let y = ((area.y + area.height) / 2) - height / 2;

        let message_rect = Rect { x, y, width, height };
        let fill_style = Style::default().bg(Color::Black).fg(Color::Black);

        draw_rect_on_grid(buf, message_rect);
        fill_rect(&message_rect, fill_style, buf);

        let text_style = Style::default().bg(Color::Black).fg(Color::Red).modifier(Modifier::BOLD);
        let text_max_length = message_rect.width - 4;

        let error_text = truncate_end(self.error_message, text_max_length);
        let error_text_start_position = ((message_rect.width - error_text.chars().count() as u16) as f64 / 2.0).ceil() as u16 + message_rect.x;
        buf.set_string(error_text_start_position, message_rect.y + message_rect.height / 2 - 2, error_text, text_style);

        let controls_text = [
            "(Press <ESC> to dismiss)",
            "(<ESC> to dismiss)",
        ];
        for line in controls_text.iter() {
            if text_max_length >= line.chars().count() as u16 {
                let start_position = ((message_rect.width - line.chars().count() as u16) as f64 / 2.0).ceil() as u16 + message_rect.x;
                buf.set_string(start_position, message_rect.y + message_rect.height / 2 + 2, line, text_style);
                break;
            }
        }
    }
}
