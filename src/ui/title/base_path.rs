use ::tui::layout::Rect;
use ::tui::style::{Style, Color, Modifier};
use ::tui::widgets::Widget;
use ::std::path::PathBuf;
use ::tui::buffer::Buffer;

use crate::ui::{draw_symbol_with_style, boundaries};
use crate::ui::format::{DisplaySize, truncate_middle};

// TODO: use external method
fn draw_rect_on_grid_with_style (buf: &mut Buffer, rect: Rect, style: Style) {
    // top and bottom
    for x in rect.x..(rect.x + rect.width) {
        if x == rect.x {
            draw_symbol_with_style(buf, x, rect.y, &boundaries::TOP_LEFT, style);
            draw_symbol_with_style(buf, x, rect.y + rect.height - 1, &boundaries::BOTTOM_LEFT, style);
        } else if x == rect.x + rect.width - 1 {
            draw_symbol_with_style(buf, x, rect.y, &boundaries::TOP_RIGHT, style);
            draw_symbol_with_style(buf, x, rect.y + rect.height - 1, &boundaries::BOTTOM_RIGHT, style);
        } else {
            draw_symbol_with_style(buf, x, rect.y, &boundaries::HORIZONTAL, style);
            draw_symbol_with_style(buf, x, rect.y + rect.height - 1, &boundaries::HORIZONTAL, style);
        }
    }

    // left and right
    for y in (rect.y + 1)..(rect.y + rect.height - 1) {
        draw_symbol_with_style(buf, rect.x, y, &boundaries::VERTICAL, style);
        draw_symbol_with_style(buf, rect.x + rect.width - 1, y, &boundaries::VERTICAL, style);
    }
}

pub struct BasePath {
    path: String,
    size: DisplaySize,
    num_descendants: u64,
    visual_indicator: u64,
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
            visual_indicator: 0,
            loading: false,
        }
    }
    pub fn visual_indicator(mut self, visual_indicator: u64) -> Self {
        self.visual_indicator = visual_indicator;
        self
    }
    pub fn loading(mut self, should_be_loading: bool) -> Self {
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
}

impl<'a> Widget for BasePath {
    fn draw(&mut self, rect: Rect, buf: &mut Buffer) {
        let text = self.text(Some(rect.width - 10)); // 10 so that text will not overflow
        let text_start_position = (rect.width / 2) - (text.chars().count() / 2) as u16;

        if self.loading {
            let text_length = text.chars().count();
            let index_in_text = (self.visual_indicator as usize % text_length) as u16;
            let marked_style = Style::default().fg(Color::Yellow).modifier(Modifier::BOLD);
            draw_rect_on_grid_with_style(buf, rect, Style::default().fg(Color::Yellow));
            buf.set_string(text_start_position, rect.y + rect.height - 2, text.clone(), Style::default().fg(Color::Yellow));
            buf.get_mut(text_start_position + index_in_text, rect.y + 1).set_style(marked_style);
            buf.get_mut(text_start_position + index_in_text + 1, rect.y + 1).set_style(marked_style);
            buf.get_mut(text_start_position + index_in_text + 2, rect.y + 1).set_style(marked_style);
        } else {
            draw_rect_on_grid_with_style(buf, rect, Style::default().fg(Color::Yellow).modifier(Modifier::BOLD));
            buf.set_string(text_start_position, rect.y + rect.height - 2, text.clone(), Style::default().fg(Color::Yellow).modifier(Modifier::BOLD));
        }
    }
}
