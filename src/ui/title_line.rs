use tui::layout::Rect;
use tui::style::{Style, Color, Modifier};
use ::tui::layout::Alignment;
use tui::widgets::{Widget};
use ::tui::terminal::Frame;
use ::tui::backend::Backend;
use ::tui::layout::{Layout, Constraint, Direction};
use std::path::PathBuf;

use ::tui::widgets::{Block, Borders, Paragraph, Text};

use crate::ui::DisplaySize;
use crate::ui::UiMode;

// TODO: merge with identical function elsewhere
fn truncate_middle(row: &str, max_length: u16) -> String {
    if max_length < 6 {
        String::from("") // TODO: make sure this never happens
    } else if row.len() as u16 > max_length {
        let first_slice = &row[0..(max_length as usize / 2) - 2];
        let second_slice = &row[(row.len() - (max_length / 2) as usize + 2)..row.len()];
        if max_length % 2 == 0 {
            format!("{}[...]{}", first_slice, second_slice)
        } else {
            format!("{}[..]{}", first_slice, second_slice)
        }
    } else {
        row.to_string()
    }
}

// these have to be macros because Text isn't Sized
macro_rules! render_boxless_title {
    (  $text: ident, $frame: ident, $rect: ident ) => {{
        Paragraph::new($text.iter())
            .block(Block::default().borders(Borders::NONE))
            .style(Style::default())
            .alignment(Alignment::Center)
            .render($frame, $rect);
    }};
}

macro_rules! render_boxed_title {
    (  $text: ident, $frame: ident, $rect: ident ) => {{
        Paragraph::new($text.iter())
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default())
            .alignment(Alignment::Center)
            .render($frame, $rect);
    }};
}

pub struct TitleLine {
    base_path: String,
    base_path_size: DisplaySize,
    current_path: String,
    current_path_size: DisplaySize,
    space_freed: DisplaySize,
    ui_mode: UiMode, // TODO: better, we should not know about this
    scan_boolean: bool,
}

impl TitleLine {
    pub fn new(base_path: &PathBuf, base_path_size: u64, current_path: &PathBuf, current_path_size: u64, space_freed: u64, ui_mode: UiMode, scan_boolean: bool) -> Self {
        let base_path = base_path.clone().into_os_string().into_string().expect("could not convert os string to string");
        let current_path = current_path.clone().into_os_string().into_string().expect("could not convert os string to string");
        let base_path_size = DisplaySize(base_path_size as f64);
        let current_path_size = DisplaySize(current_path_size as f64);
        let space_freed = DisplaySize(space_freed as f64);
        Self { base_path, base_path_size, current_path, current_path_size, space_freed, ui_mode, scan_boolean }
    }
    pub fn render(&self, frame: &mut Frame<impl Backend>, rect: Rect) {
        let current_path_text = format!("{} ({})", &self.current_path, &self.current_path_size);
        let current_path_display = [
            Text::raw("\n"), // this isn't inside a block, so it needs a gentle push to be centered vertically
            Text::styled(&current_path_text, Style::default().fg(Color::Green).modifier(Modifier::BOLD))
        ];
        let base_path_text = format!("{} ({})", &self.base_path, &self.base_path_size);
        let base_path_display = [
            Text::styled(&base_path_text, Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
        ];
        let space_freed_text = format!("Space freed: {}", &self.space_freed);
        let space_freed_display = [
            Text::styled(format!("Space freed: {}", &self.space_freed), Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
        ];

        let loading_text = String::from("Scanning folder...");
        let loading_display = if self.scan_boolean {
            [
                Text::styled(loading_text.clone(), Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
            ]
        } else {
            [
                Text::styled(loading_text.clone(), Style::default().fg(Color::Yellow))
            ]
        };

        let min_current_path_len = current_path_text.len() as u16 + 10;
        let min_base_path_len = base_path_text.len() as u16 + 10;
        let min_space_freed_text_len = space_freed_text.len() as u16 + 10;

        let min_loading_text_len = loading_text.len() as u16 + 10;
        
        if let UiMode::Loading = self.ui_mode {
            if min_current_path_len + min_loading_text_len <= rect.width {
                let remainder = rect.width - min_space_freed_text_len - min_current_path_len;
                let parts = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(min_current_path_len + remainder),
                            Constraint::Length(min_loading_text_len),
                        ].as_ref()
                    )
                    .split(rect);


                let (left, right) = (parts[0], parts[1]);
                render_boxless_title!(current_path_display, frame, left);
                render_boxed_title!(loading_display, frame, right);
            } else {
                // TODO: merge with below final else
                let current_path_size_len = &self.current_path_size.to_string().len() + 2; // 2 == two parentheses chars
                // TODO: truncate folder numes in full path a la fish
                let current_path_text = format!("{} ({})", truncate_middle(&self.current_path, rect.width - current_path_size_len as u16), &self.current_path_size);
                let current_path_display = [
                    Text::raw("\n"), // this isn't inside a block, so it needs a gentle push to be centered vertically
                    Text::styled(&current_path_text, Style::default().fg(Color::Green).modifier(Modifier::BOLD))
                ];
                render_boxless_title!(current_path_display, frame, rect);
            }
        } else if min_current_path_len + min_base_path_len + min_space_freed_text_len <= rect.width {
            let remainder = rect.width - min_space_freed_text_len - min_base_path_len - min_current_path_len;
            let parts = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints(
                    [
                        Constraint::Length(min_current_path_len + remainder),
                        Constraint::Length(min_base_path_len),
                        Constraint::Length(min_space_freed_text_len),
                    ].as_ref()
                )
                .split(rect);


            let (left, middle, right) = (parts[0], parts[1], parts[2]);
            render_boxless_title!(current_path_display, frame, left);
            render_boxed_title!(base_path_display, frame, middle);
            render_boxed_title!(space_freed_display, frame, right);
        } else if min_current_path_len + min_space_freed_text_len <= rect.width {
            let remainder = rect.width - min_space_freed_text_len - min_current_path_len;
            let parts = Layout::default()
                .direction(Direction::Horizontal)
                .margin(0)
                .constraints(
                    [
                        Constraint::Length(min_current_path_len + remainder),
                        Constraint::Length(min_space_freed_text_len),
                    ].as_ref()
                )
                .split(rect);


            let (left, right) = (parts[0], parts[1]);
            render_boxless_title!(current_path_display, frame, left);
            render_boxed_title!(space_freed_display, frame, right);
        } else {
            let current_path_size_len = &self.current_path_size.to_string().len() + 2; // 2 == two parentheses chars
            // TODO: truncate folder numes in full path a la fish
            let current_path_text = format!("{} ({})", truncate_middle(&self.current_path, rect.width - current_path_size_len as u16), &self.current_path_size);
            let current_path_display = [
                Text::raw("\n"), // this isn't inside a block, so it needs a gentle push to be centered vertically
                Text::styled(&current_path_text, Style::default().fg(Color::Green).modifier(Modifier::BOLD))
            ];
            render_boxless_title!(current_path_display, frame, rect);
        }
    }
}
