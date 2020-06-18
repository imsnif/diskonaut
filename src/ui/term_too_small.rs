use ::tui::buffer::Buffer;
use ::tui::layout::Rect;
use ::tui::style::{Modifier, Style};
use ::tui::widgets::Widget;

pub struct TermTooSmall {}

impl TermTooSmall {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> Widget for TermTooSmall {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let text = [
            "Terminal window is too small ¯\\_(ツ)_/¯",
            "Window too small ¯\\_(ツ)_/¯",
            "too small ¯\\_(ツ)_/¯",
            "¯\\_(ツ)_/¯",
            "!!!",
        ];
        for line in text.iter() {
            if area.width >= line.chars().count() as u16 {
                buf.set_string(
                    ((area.x + area.width) / 2) as u16 - ((line.chars().count() / 2) as u16),
                    area.y + area.height / 2,
                    line,
                    Style::default().modifier(Modifier::BOLD),
                );
                break;
            }
        }
    }
}
