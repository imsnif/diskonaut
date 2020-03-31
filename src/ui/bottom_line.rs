use tui::layout::Rect;
use tui::style::{Style, Color};
use tui::widgets::Widget;

use tui::buffer::Buffer;

pub struct BottomLine {}

impl BottomLine {
    pub fn new() -> Self {
        Self {}
    }
}

impl<'a> Widget for BottomLine {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let bottom_left_character = buf.get_mut(1, area.y + area.height - 1);
        bottom_left_character.set_symbol("x");
        bottom_left_character.set_style(Style::default().bg(Color::White).fg(Color::Black));
        buf.set_string(3, area.y + area.height - 1, "= Small files", Style::default());
    }
}
