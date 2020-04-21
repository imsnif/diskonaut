use tui::layout::Rect;
use tui::style::{Style, Color};
use tui::widgets::Widget;

use tui::buffer::Buffer;

use crate::ui::UiMode;

pub struct BottomLine {
    ui_mode: UiMode, // TODO: better, we should not know about this
}

impl BottomLine {
    pub fn new(ui_mode: UiMode) -> Self {
        Self {ui_mode}
    }
}

impl<'a> Widget for BottomLine {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let bottom_left_character = buf.get_mut(1, area.y + area.height - 2);
        bottom_left_character.set_symbol("x");
        bottom_left_character.set_style(Style::default().bg(Color::White).fg(Color::Black));
        buf.set_string(3, area.y + area.height - 2, "= Small files", Style::default());
        
        let long_controls_line = match self.ui_mode {
            UiMode::Loading => String::from("<hjkl> or <arrow keys> - move around, <ENTER> - enter folder, <ESC> - parent folder"),
            _ => String::from("<hjkl> or <arrow keys> - move around, <ENTER> - enter folder, <ESC> - parent folder, <Ctrl-D> - delete")
        };
        let short_controls_line = String::from("←↓↑→/<ENTER>/<ESC>: navigate, <Ctrl-D>: del");
        let too_small_line = "(...)";
        if area.width >= long_controls_line.chars().count() as u16 {
            buf.set_string(1, area.y + area.height - 1, long_controls_line, Style::default());
        } else if area.width >= short_controls_line.chars().count() as u16 {
            buf.set_string(1, area.y + area.height - 1, short_controls_line, Style::default());
        } else {
            buf.set_string(1, area.y + area.height - 1, too_small_line, Style::default());
        }
    }
}
