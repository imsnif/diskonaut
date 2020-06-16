use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;

use crate::ui::format::truncate_end;
use crate::ui::grid::draw_filled_rect;

pub struct ErrorBox<'a> {
    error_message: &'a str,
}

impl<'a> ErrorBox<'a> {
    pub fn new(error_message: &'a str) -> Self {
        Self { error_message }
    }
}

impl<'a> Widget for ErrorBox<'a> {
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

        let message_rect = Rect {
            x,
            y,
            width,
            height,
        };
        let fill_style = Style::default()
            .bg(Color::Black)
            .fg(Color::Red)
            .modifier(Modifier::BOLD);
        let text_max_length = message_rect.width - 4;

        // here we truncate the end and not the middle because
        // when dealing with error messages, the beginning tends
        // to be the important part
        let error_text = truncate_end(self.error_message, text_max_length);
        let error_text_start_position =
            ((message_rect.width - error_text.chars().count() as u16) as f64 / 2.0).ceil() as u16
                + message_rect.x;

        let controls_text = ["(Press <ESC> to dismiss)", "(<ESC> to dismiss)"];

        draw_filled_rect(buf, fill_style, &message_rect);
        buf.set_string(
            error_text_start_position,
            message_rect.y + message_rect.height / 2 - 2,
            error_text,
            fill_style,
        );

        for line in controls_text.iter() {
            if text_max_length >= line.chars().count() as u16 {
                let start_position =
                    ((message_rect.width - line.chars().count() as u16) as f64 / 2.0).ceil() as u16
                        + message_rect.x;
                buf.set_string(
                    start_position,
                    message_rect.y + message_rect.height / 2 + 2,
                    line,
                    fill_style,
                );
                break;
            }
        }
    }
}
