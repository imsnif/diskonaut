use tui::layout::Rect;
use tui::style::{Style, Color};
use ::tui::layout::Alignment;
use tui::widgets::{Widget};
use ::tui::terminal::Frame;
use ::tui::backend::Backend;
use std::path::PathBuf;

use ::tui::widgets::{Block, Borders, Paragraph, Text};

use crate::ui::DisplaySize;

pub struct TitleLine {
    current_path: String,
    size: DisplaySize,
}

impl TitleLine {
    pub fn new(current_path: &PathBuf, folder_size: u64) -> Self {
        let current_path = current_path.clone().into_os_string().into_string().expect("could not convert os string to string");
        let size = DisplaySize(folder_size as f64);
        Self { current_path, size }
    }
    pub fn render(&self, frame: &mut Frame<impl Backend>, rect: Rect) {
        let title_text = Text::styled(format!("{} ({})", self.current_path, self.size), Style::default().fg(Color::Green));
        let title_lines = [
            Text::styled("\n", Style::default()),
            title_text,
            Text::styled("\n", Style::default()),
        ];
        Paragraph::new(title_lines.iter())
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default())
            .alignment(Alignment::Center)
            .wrap(true)
            .render(frame, rect);
    }
}
