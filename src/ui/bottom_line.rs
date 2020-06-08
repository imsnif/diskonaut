use tui::layout::Rect;
use tui::style::{Style, Color, Modifier};
use tui::widgets::Widget;

use tui::buffer::Buffer;

use crate::state::{FileRect, FileType};
use crate::ui::format::DisplaySize;

pub struct BottomLine <'a>{
    hide_delete: bool,
    currently_selected: Option<&'a FileRect>,
    failed_to_read: u64,
}

impl <'a>BottomLine <'a>{
    pub fn new(failed_to_read: u64, currently_selected: Option<&'a FileRect>) -> Self {
        Self { hide_delete: false, failed_to_read, currently_selected }
    }
    pub fn hide_delete(mut self) -> Self {
        self.hide_delete = true;
        self
    }
}

impl<'a> Widget for BottomLine <'a>{
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        // status line TODO: make own component
        let small_files_legend = "(x = Small files)";
        if let Some(currently_selected) = self.currently_selected {
            let file_name = currently_selected.file_metadata.name.to_string_lossy();
            let size = DisplaySize(currently_selected.file_metadata.size as f64);
            let descendants = currently_selected.file_metadata.descendants;
            let (style, lines) = match currently_selected.file_metadata.file_type {
                FileType::File => (
                    Style::default().modifier(Modifier::BOLD),
                    vec![
                        format!("SELECTED: {} ({})", file_name, size),
                        format!("SELECTED: {}", file_name),
                        format!("{}", file_name)
                    ]
                ),
                FileType::Folder => (
                    Style::default().fg(Color::Blue).modifier(Modifier::BOLD),
                    vec![
                        format!("SELECTED: {} ({}, {} files)", file_name, size, descendants.expect("a folder should have descendants")),
                        format!("SELECTED: {} ({})", file_name, size),
                        format!("SELECTED: {}", file_name),
                        format!("{}", file_name),
                    ]
                )
            };
            for line in lines {
                if (line.chars().count() as u16) < area.width - small_files_legend.chars().count() as u16 {
                    buf.set_string(1, area.y + area.height - 2, line, style);
                    break;
                }
            }
        } else if self.failed_to_read > 0 {
            // this line is (most likely!) less than 50 characters, so no need to telescope it
            buf.set_string(1, area.y + area.height - 2, format!("Failed to read {} files", self.failed_to_read), Style::default().fg(Color::Red));
        }

        let small_files_len = small_files_legend.chars().count() as u16;
        buf.set_string(area.width - small_files_len - 1, area.y + area.height - 2, small_files_legend, Style::default());
        let bottom_left_character = buf.get_mut(area.width - small_files_len, area.y + area.height - 2);
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
