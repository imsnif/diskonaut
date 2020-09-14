use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::Widget;

use crate::ui::format::truncate_end;
use crate::ui::grid::draw_filled_rect;

pub struct WarningBox {}

impl<'a> WarningBox {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> Widget for WarningBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (width, height) = if area.width > 150 {
            (150, 10)
        } else if area.width >= 50 {
            (area.width / 2, 10)
        } else {
            unreachable!("app should not be rendered if window is so small")
        };

        // position self in the middle of the rect
        let x = ((area.x + area.width) / 2) - width / 2;
        let y = ((area.y + area.height) / 2) - height / 2;

        let warning_rect = Rect {
            x,
            y,
            width,
            height,
        };
        let fill_style = Style::default()
            .bg(Color::Black)
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD);
        let text_max_length = warning_rect.width - 4;
        let mut warning_text_start_position: u16 = 0;

        let possible_warning_texts = [
            "Sorry, deletion is only allowed once the scanning has completed",
            "Deletion is not allowed while scanning",
            "Can't delete while scanning",
            "Can't delete now",
        ];
        // set default value of the warning_text
        // to the longest one from possible_warning_texts array
        let mut warning_text = String::from(possible_warning_texts[0]);
        for line in possible_warning_texts.iter() {
            // "+5" here is to make sure confirm message has always some padding
            if warning_rect.width >= (line.chars().count() as u16) + 5 {
                // here we truncate the end and not the middle because
                // when dealing with warning messages, the beginning tends
                // to be the important part
                warning_text = truncate_end(line, text_max_length);
                warning_text_start_position =
                    ((warning_rect.width - warning_text.len() as u16) as f64 / 2.0).ceil() as u16
                        + warning_rect.x;
                break;
            }
        }

        let controls_text = ["(Press any key to dismiss)", "(any key to dismiss)"];

        draw_filled_rect(buf, fill_style, &warning_rect);
        buf.set_string(
            warning_text_start_position,
            warning_rect.y + warning_rect.height / 2 - 2,
            warning_text,
            fill_style,
        );

        for line in controls_text.iter() {
            if text_max_length >= line.chars().count() as u16 {
                let start_position =
                    ((warning_rect.width - line.chars().count() as u16) as f64 / 2.0).ceil() as u16
                        + warning_rect.x;
                buf.set_string(
                    start_position,
                    warning_rect.y + warning_rect.height / 2 + 2,
                    line,
                    fill_style,
                );
                break;
            }
        }
    }
}
