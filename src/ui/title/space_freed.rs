use ::tui::layout::Rect;
use ::tui::style::{Style, Color, Modifier};
use ::tui::layout::Alignment;
use ::tui::widgets::{Widget};
use ::tui::terminal::Frame;
use ::tui::backend::Backend;

use ::tui::widgets::{Block, Borders, Paragraph, Text};

use crate::ui::DisplaySize;

pub struct SpaceFreed {
    size: DisplaySize,
    frame: bool,
}

impl SpaceFreed { 
    pub fn new (size: u64) -> Self {
        let size = DisplaySize(size as f64);
        SpaceFreed {
            size,
            frame: false,
        }
    }
    pub fn frame(mut self, should_have_frame: bool) -> Self {
        self.frame = should_have_frame;
        self
    }
    pub fn len (&self) -> usize {
        self.text().len()
    }
    fn text (&self) -> String {
        format!("Space freed: {}", self.size)
    }
    pub fn render(&self, frame: &mut Frame<impl Backend>, rect: Rect) {
        let text = self.text();
        if self.frame {
            let text_display = [
                Text::styled(text, Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
            ];
            Paragraph::new(text_display.iter())
                .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD)))
                .style(Style::default().fg(Color::Yellow))
                .alignment(Alignment::Center)
                .render(frame, rect);
        } else {
            let text_display = [
                Text::raw("\n"), // this isn't inside a block, so it needs a gentle push to be centered vertically
                Text::styled(text, Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
            ];
            Paragraph::new(text_display.iter())
                .block(Block::default().borders(Borders::NONE))
                .style(Style::default())
                .alignment(Alignment::Center)
                .render(frame, rect);
        };
    }
}
