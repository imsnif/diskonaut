use ::tui::buffer::Buffer;
use ::tui::layout::Rect;
use ::tui::style::{Color, Modifier, Style};
use ::tui::widgets::Widget;

use crate::ui::format::truncate_middle;
use crate::ui::grid::draw_filled_rect;

fn render_confirm_prompt(buf: &mut Buffer, confirm_rect: &Rect) {
    let text_style = Style::default()
        .bg(Color::Black)
        .fg(Color::White)
        .add_modifier(Modifier::BOLD);

    let possible_confirm_texts = [
        "Are you sure you want to quit?",
        "Sure you want to quit?",
        "Really quit?",
        "Quit?",
    ];
    // set default value of the confirm_text
    // to the longest one from possible_confirm_text array
    let mut confirm_text = String::from(possible_confirm_texts[0]);
    let mut confirm_text_start_position: u16 = 0;
    let text_max_length = confirm_rect.width - 4;
    for line in possible_confirm_texts.iter() {
        // "+10" here is to make sure confirm message has always some padding
        if confirm_rect.width >= (line.chars().count() as u16) + 10 {
            confirm_text = truncate_middle(line, text_max_length);
            confirm_text_start_position =
                ((confirm_rect.width - confirm_text.len() as u16) as f64 / 2.0).ceil() as u16
                    + confirm_rect.x;
            break;
        }
    }

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
        confirm_rect.y + confirm_rect.height / 2 + 3,
        y_n_line,
        text_style,
    );
}

pub struct ConfirmBox {}

impl ConfirmBox {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> Widget for ConfirmBox {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let (width, height) = if area.width > 150 {
            (150, 10)
        } else if area.width >= 50 {
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
            .add_modifier(Modifier::BOLD);

        draw_filled_rect(buf, fill_style, &confirm_rect);

        render_confirm_prompt(buf, &confirm_rect);
    }
}
