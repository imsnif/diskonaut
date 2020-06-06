use tui::layout::Rect;
use tui::style::{Style, Color};
use tui::widgets::Widget;

use tui::buffer::Buffer;

pub struct BottomLine {
    hide_delete: bool,
    failed_to_read: u64,
}

impl BottomLine {
    pub fn new(failed_to_read: u64) -> Self {
        Self { hide_delete: false, failed_to_read }
    }
    pub fn hide_delete(mut self) -> Self {
        self.hide_delete = true;
        self
    }
}

impl<'a> Widget for BottomLine {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        if self.failed_to_read > 0 {
            buf.set_string(1, area.y + area.height - 2, format!("Failed to read {} files", self.failed_to_read), Style::default().fg(Color::Red));
        }

        buf.set_string(area.width - 18, area.y + area.height - 2, "(x = Small files)", Style::default());
        let bottom_left_character = buf.get_mut(area.width - 17, area.y + area.height - 2);
        bottom_left_character.set_style(Style::default().bg(Color::White).fg(Color::Black));

        let (long_controls_line, short_controls_line) = if self.hide_delete {
            (
                String::from("<hjkl> or <arrow keys> - move around, <ENTER> - enter folder, <ESC> - parent folder"),
                String::from("←↓↑→/<ENTER>/<ESC>: navigate")
            )
        } else {
            (
                String::from("<hjkl> or <arrow keys> - move around, <ENTER> - enter folder, <ESC> - parent folder, <Ctrl-D> - delete"),
                String::from("←↓↑→/<ENTER>/<ESC>: navigate, <Ctrl-D>: del")
            )
        };
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
