use ::tui::buffer::Buffer;
use ::tui::layout::Rect;
use ::tui::style::{Color, Modifier, Style};
use ::tui::widgets::Widget;

use crate::ui::format::truncate_middle;
use crate::ui::grid::draw_filled_rect;

fn render_confirm_prompt(buf: &mut Buffer, confirm_rect: &Rect, confirm_msg: &str) {
    let text_style = Style::default()
        .bg(Color::Black)
        .fg(Color::White)
        .modifier(Modifier::BOLD);

    let text_max_length = confirm_rect.width - 4;
    let confirm_text = truncate_middle(confirm_msg, text_max_length);
    let confirm_text_start_position =
        ((confirm_rect.width - confirm_text.len() as u16) as f64 / 2.0).ceil() as u16
            + confirm_rect.x;
    let y_n_line = "(y/n)";
    let y_n_line_start_position =
        ((confirm_rect.width - y_n_line.len() as u16) as f64 / 2.0).ceil() as u16 + confirm_rect.x;

    buf.set_string(
        confirm_text_start_position,
        confirm_rect.y + confirm_rect.height / 2 - 2,
        confirm_text,
        text_style,
    );
    buf.set_string(
        y_n_line_start_position,
        confirm_rect.y + confirm_rect.height / 2,
        y_n_line,
        text_style,
    );
}

pub struct ConfirmBox<'a> {
    confirm_msg: &'a str,
}

impl<'a> ConfirmBox<'a> {
    pub fn new(confirm_msg: &'a str) -> Self {
        Self { confirm_msg }
    }
}

impl<'a> Widget for ConfirmBox<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (width, height) = if area.width > 150 {
            (150, 10)
        } else if area.width > 50 {
            (area.width / 2, 10)
        } else {
            unreachable!("app should not be rendered if window is so small")
        };

        // position self in the middle of the self
        let x = ((area.x + area.width) / 2) - width / 2;
        let y = ((area.y + area.height) / 2) - height / 2;

        let confirm_rect = Rect {
            x,
            y,
            width,
            height,
        };
        let fill_style = Style::default()
            .bg(Color::Black)
            .fg(Color::White)
            .modifier(Modifier::BOLD);

        draw_filled_rect(buf, fill_style, &confirm_rect);

        render_confirm_prompt(buf, &confirm_rect, &self.confirm_msg);
    }
}
