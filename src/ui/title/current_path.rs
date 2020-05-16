use ::tui::layout::Rect;
use ::tui::style::{Style, Color, Modifier};
use ::tui::layout::Alignment;
use ::tui::widgets::{Widget};
use ::tui::terminal::Frame;
use ::tui::backend::Backend;
use ::std::path::PathBuf;

use ::tui::widgets::{Block, Borders, Paragraph, Text};

use crate::ui::format::{DisplaySize, truncate_middle};

pub struct CurrentPath {
    path: String,
    size: DisplaySize,
    frame: bool,
    red: bool,
}

impl CurrentPath { 
    pub fn new (path: &PathBuf, size: u64) -> Self {
        let size = DisplaySize(size as f64);
        let path = path.clone().into_os_string().into_string().expect("could not convert os string to string");
        CurrentPath {
            path,
            size,
            frame: false,
            red: false,
        }
    }
    pub fn frame(mut self, should_have_frame: bool) -> Self {
        self.frame = should_have_frame;
        self
    }
    pub fn red(mut self, should_be_red: bool) -> Self {
        self.red = should_be_red;
        self
    }
    pub fn len (&self) -> usize {
        self.text(None).len()
    }
    fn text (&self, max_len: Option<u16>) -> String {
        let size_string_len = &self.size.to_string().len() + 2; // 2 == two parentheses chars
        // TODO: truncate folder numes in full path a la fish
        match max_len {
            Some(len) => format!("{} ({})", truncate_middle(&self.path, len - size_string_len as u16), &self.size),
            None => format!("{} ({})", &self.path, &self.size),
        }
    }
    pub fn render(&self, frame: &mut Frame<impl Backend>, rect: Rect) {
        let text = self.text(Some(rect.width - 10)); // 10 so that text will not overflow
        let color = if self.red {
            Color::Red
        } else {
            Color::Green
        };
        if self.frame {
            let text_display = [
                Text::styled(text, Style::default().fg(color).modifier(Modifier::BOLD))
            ];
            Paragraph::new(text_display.iter())
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(color).modifier(Modifier::BOLD)))
                .style(Style::default().fg(color))
                .alignment(Alignment::Center)
                .render(frame, rect);
        } else {
            let text_display = [
                Text::raw("\n"), // this isn't inside a block, so it needs a gentle push to be centered vertically
                Text::styled(text, Style::default().fg(color).modifier(Modifier::BOLD))
            ];
            Paragraph::new(text_display.iter())
                .block(Block::default().borders(Borders::NONE))
                .style(Style::default())
                .alignment(Alignment::Center)
                .render(frame, rect);
        };
    }
}
