use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Style, Color, Modifier};
use tui::widgets::{Widget};
use std::path::PathBuf;

use crate::ui::{draw_symbol_with_style, boundaries};
use crate::state::files::FileOrFolder;

pub struct MessageBox<'a> {
    current_path: &'a PathBuf,
    file_to_delete: &'a FileOrFolder,
}

impl <'a>MessageBox<'a> {
    pub fn new (file_to_delete: &'a FileOrFolder, current_path: &'a PathBuf) -> Self {
        Self { current_path, file_to_delete }
    }
}

// TODO: merge with identical function elsewhere
fn draw_rect_on_grid (buf: &mut Buffer, rect: Rect) {
    // top and bottom
    for x in rect.x..(rect.x + rect.width + 1) {
        if x == rect.x {
            draw_symbol_with_style(buf, x, rect.y, &boundaries::TOP_LEFT, Style::default().bg(Color::Black));
            draw_symbol_with_style(buf, x, rect.y + rect.height, &boundaries::BOTTOM_LEFT, Style::default().bg(Color::Black));
        } else if x == rect.x + rect.width {
            draw_symbol_with_style(buf, x, rect.y, &boundaries::TOP_RIGHT, Style::default().bg(Color::Black));
            draw_symbol_with_style(buf, x, rect.y + rect.height, &boundaries::BOTTOM_RIGHT, Style::default().bg(Color::Black));
        } else {
            draw_symbol_with_style(buf, x, rect.y, &boundaries::HORIZONTAL, Style::default().bg(Color::Black));
            draw_symbol_with_style(buf, x, rect.y + rect.height, &boundaries::HORIZONTAL, Style::default().bg(Color::Black));
        }
    }

    // left and right
    for y in (rect.y + 1)..(rect.y + rect.height) {
        draw_symbol_with_style(buf, rect.x, y, &boundaries::VERTICAL, Style::default().bg(Color::Black));
        draw_symbol_with_style(buf, rect.x + rect.width, y, &boundaries::VERTICAL, Style::default().bg(Color::Black));
    }
}

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

impl<'a> Widget for MessageBox<'a> {
    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
         let (width, height) = if area.width > 150 {
             (150, 10)
         } else if area.width > 50 {
             (area.width / 2, 10)
         } else {
             (10, 10) // TODO: do not render app if it's so small
         };

        // position self in the middle of the rect
        let x = ((area.x + area.width) / 2) - width / 2;
        let y = ((area.y + area.height) / 2) - height / 2;


        let message_rect = Rect { x, y, width, height };
        draw_rect_on_grid(buf, message_rect);

        for x in message_rect.x..(message_rect.x + message_rect.width) {
            if x > message_rect.x && x < area.x + area.width {
                for y in message_rect.y..(message_rect.y + message_rect.height) {
                    if y > message_rect.y && y < area.y + area.height {
                        let buf = buf.get_mut(x, y);
                        buf.set_symbol(" ");
                        buf.set_style(Style::default().bg(Color::Black).fg(Color::Black));
                    }
                }
            }
        }
        let text_style = Style::default().bg(Color::Black).fg(Color::Red).modifier(Modifier::BOLD);
        let text_length = message_rect.width - 4; // TODO: do not render app if it's so small

        let question_line = match &self.file_to_delete {
            FileOrFolder::File(_) => {
                if text_length >= 17 {
                    format!("Delete this file?")
                } else if text_length >= 3 {
                    format!("Delete?")
                } else {
                    format!("?!?!?! {}", text_length) // TODO: donot render app if it's so small
                }
            },
            FileOrFolder::Folder(folder) => {
                let children = folder.num_descendants;
                let full_line = format!("Delete folder with {} children?", children);
                let short_line = format!("Delete folder?");
                if text_length >= full_line.len() as u16 {
                    full_line
                } else if text_length >= short_line.len() as u16 {
                    format!("Delete folder?")
                } else {
                    format!("?!?!?! {}", text_length) // TODO: donot render app if it's so small
                }
            }
        };

        let mut full_path = PathBuf::from(self.current_path);
        full_path.push(&self.file_to_delete.name());

        let full_path = full_path.into_os_string().into_string().expect("could not convert os string to string");
        let file_name = &self.file_to_delete.name();

        let full_path_display = String::from(full_path);
        let file_name_line = if text_length > full_path_display.len() as u16 {
            full_path_display
        } else {
            truncate_middle(file_name, text_length)
        };
        let y_n_line = "(y/n)";
        let question_line_start_position = ((message_rect.width - question_line.len() as u16) as f64 / 2.0).ceil() as u16 + message_rect.x;
        let file_name_line_start_position = ((message_rect.width - file_name_line.len() as u16) as f64 / 2.0).ceil() as u16 + message_rect.x;
        let y_n_line_start_position = ((message_rect.width - y_n_line.len() as u16) as f64 / 2.0).ceil() as u16 + message_rect.x;
        buf.set_string(question_line_start_position, message_rect.y + message_rect.height / 2 - 3, question_line, text_style);
        buf.set_string(file_name_line_start_position, message_rect.y + message_rect.height / 2, file_name_line, text_style);
        buf.set_string(y_n_line_start_position, message_rect.y + message_rect.height / 2 + 3, y_n_line, text_style);

    }

}
