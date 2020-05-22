use ::tui::layout::Rect;
use ::tui::style::{Style, Color, Modifier};
use ::tui::layout::Alignment;
use ::tui::widgets::{Widget};
use ::tui::terminal::Frame;
use ::tui::backend::Backend;
use ::std::path::PathBuf;

use ::tui::widgets::{Block, Borders, Paragraph, Text};

use crate::ui::format::{DisplaySize, truncate_middle};

pub struct BasePath {
    path: String,
    size: DisplaySize,
    num_descendants: u64,
    bold: bool,
    loading: bool,
}

impl BasePath { 
    pub fn new (path: &PathBuf, size: u64, num_descendants: u64) -> Self {
        let size = DisplaySize(size as f64);
        let path = path.clone().into_os_string().into_string().expect("could not convert os string to string");
        BasePath {
            path,
            size,
            num_descendants,
            bold: true,
            loading: false,
        }
    }
    pub fn bold(mut self, should_be_bold: bool) -> Self {
        if !self.loading {
            self.bold = true;
        } else {
            self.bold = should_be_bold;
        }
        self
    }
    pub fn loading(mut self, should_be_loading: bool) -> Self {
        if !should_be_loading { 
            self.bold = true;
        }
        self.loading = should_be_loading;
        self
    }
    pub fn len (&self) -> usize {
        self.text(None).len()
    }
    fn text (&self, max_len: Option<u16>) -> String {
        let info_string = format!(" | {} | +{} files", &self.size, &self.num_descendants);
        let path_text = match max_len {
            Some(len) => truncate_middle(&self.path, len - info_string.len() as u16),
            None => String::from(&self.path),
        };
        // TODO: truncate size and num_descendants before info_string
        // TODO: truncate folder numes in full path a la fish
        if self.loading {
            format!("Scanning: {}{}", path_text, info_string)
        } else {
            format!("Base: {}{}", path_text, info_string)
        }
    }
    pub fn render(&self, frame: &mut Frame<impl Backend>, rect: Rect) {
        let text = self.text(Some(rect.width - 10)); // 10 so that text will not overflow
        let text_display = if self.bold {
            [ Text::styled(text, Style::default().fg(Color::Yellow).modifier(Modifier::BOLD)) ]
        } else {
            [ Text::styled(text, Style::default().fg(Color::Yellow)) ]
        };
        Paragraph::new(text_display.iter())
            .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD)))
            .style(Style::default().fg(Color::Yellow))
            .alignment(Alignment::Center)
            .render(frame, rect);
    }
}
