use ::tui::layout::Rect;
use ::tui::buffer::Buffer;
use ::tui::style::{Style, Color, Modifier};

use ::std::cmp::max;

use crate::ui::format::truncate_middle;

fn get_index_or_last <'a>(vec: &'a [CellSizeOpt], index: usize) -> &'a CellSizeOpt {
    match vec.get(index) {
        Some(item) => item,
        None => vec.last().expect("could not get last element of vec")
    }
}

pub type CollapsingCell = Vec<CellSizeOpt>;

pub struct CellSizeOpt {
    pub content: String,
    pub style: Option<Style>,
}

impl CellSizeOpt {
    pub fn new (content: String) -> Self {
        CellSizeOpt {
            content,
            style: None
        }
    }
    pub fn style (mut self, style: Style) -> Self {
        self.style = Some(style);
        self
    }
}

pub struct TitleTelescope {
    default_style: Style,
    left_side: Vec<CollapsingCell>,
    right_side: Vec<CollapsingCell>,
    loading: bool,
    loading_indicator: u64,
    path_error: bool,
    size_flash: bool,
}

impl TitleTelescope {
    pub fn new (default_style: Style) -> Self {
        TitleTelescope {
            default_style,
            left_side: vec![],
            right_side: vec![],
            loading: false,
            loading_indicator: 0,
            path_error: false,
            size_flash: false,
        }
    }
    pub fn append_to_left_side (&mut self, collapsing_cell: CollapsingCell) {
        self.left_side.push(collapsing_cell);
    }
    pub fn append_to_right_side (&mut self, collapsing_cell: CollapsingCell) {
        self.right_side.push(collapsing_cell);
    }
    pub fn loading (mut self, show_loading: bool, loading_indicator: u64) -> Self {
        self.loading = show_loading;
        self.loading_indicator = loading_indicator;
        self
    }
    pub fn path_error (mut self, should_show_path_error: bool) -> Self {
        self.path_error = should_show_path_error;
        self
    }
    pub fn size_flash (mut self, should_flash_size: bool) -> Self {
        self.size_flash = should_flash_size;
        self
    }
    pub fn render(&self, rect: Rect, buf: &mut Buffer) {
        let highest_collapse_count = max(
            self.left_side.iter().map(|c| c.len()).max().expect("could not get max cell value"),
            self.right_side.iter().map(|c| c.len()).max().expect("could not get max cell value"),
        );
        for i in 0..highest_collapse_count {
            let line_candidate_len = self.line_index_len(i);

            if (line_candidate_len as u16) < rect.width {
                self.render_line_index(i, rect, buf);
                return;
            }
        }
        self.render_truncated_line_index(highest_collapse_count, rect, buf);
    }
    fn left_side_candidate (&self, index: usize) -> Vec<&CellSizeOpt> {
        let mut left_side = vec![];
        for collapsing_cell in self.left_side.iter() {
            left_side.push(get_index_or_last(collapsing_cell, index));
        }
        left_side
    }
    fn right_side_candidate (&self, index: usize) -> Vec<&CellSizeOpt> {
        let mut right_side = vec![];
        for collapsing_cell in self.right_side.iter() {
            right_side.push(get_index_or_last(collapsing_cell, index));
        }
        right_side
    }
    fn style_of_left_side (&self, style: Option<Style>) -> Style {
        let style_if_size_flash = Style::default().bg(Color::Yellow).fg(Color::Black);
        self.condition_style_or_default(self.size_flash, style_if_size_flash, style)
    }
    fn style_of_right_side (&self, style: Option<Style>) -> Style {
        let style_if_path_error = Style::default().bg(Color::Red).fg(Color::White);
        self.condition_style_or_default(self.path_error, style_if_path_error, style)
    }
    fn condition_style_or_default(&self, condition: bool, condition_style: Style, style: Option<Style>) -> Style {
        match (condition, style) {
            (true, _) => condition_style,
            (_, Some(style)) => style,
            (_, _) => self.default_style
        }
    }
    fn render_left_side_cell(&self, cell: &CellSizeOpt, x: u16, y: u16, buf: &mut Buffer) {
        let style = self.style_of_left_side(cell.style);
        buf.set_string(x, y, &cell.content, style);
    }
    fn render_right_side_cell(&self, cell: &CellSizeOpt, x: u16, y: u16, buf: &mut Buffer) {
        let style = self.style_of_right_side(cell.style);
        buf.set_string(x, y, &cell.content, style);
    }
    fn render_pipe(&self, x: u16, y: u16, buf: &mut Buffer) {
        buf.set_string(x, y, " | ", self.default_style.fg(Color::White));
    }
    fn render_line_index(&self, i: usize, rect: Rect, buf: &mut Buffer) {
        let left_side = self.left_side_candidate(i);
        let right_side = self.right_side_candidate(i);
        let mut current_position = rect.x + 1;
        for cell_size_opt in &left_side {
            self.render_left_side_cell(cell_size_opt, current_position, rect.y, buf);
            current_position += cell_size_opt.content.len() as u16;
        }
        self.render_pipe(current_position, rect.y, buf);
        current_position += 3;
        for cell_size_opt in &right_side {
            self.render_right_side_cell(cell_size_opt, current_position, rect.y, buf);
            current_position += cell_size_opt.content.len() as u16;
        }
        if self.loading {
            let text_length = current_position - (rect.x + 1);
            self.draw_loading_chars(text_length, rect, buf);
        }
    }
    fn render_truncated_line_index(&self, index: usize, rect: Rect, buf: &mut Buffer) {
        let left_side = self.left_side_candidate(index);
        let right_side = self.right_side_candidate(index);
        let mut current_position = rect.x + 1;
        for cell_size_opt in &left_side {
            self.render_left_side_cell(cell_size_opt, current_position, rect.y, buf);
            current_position += cell_size_opt.content.len() as u16;
        }
        self.render_pipe(current_position, rect.y, buf);
        current_position += 3;
        let number_of_parts_to_truncate = right_side.len() as u16;
        for (index, cell_size_opt) in right_side.into_iter().enumerate() {
            let style = self.style_of_right_side(cell_size_opt.style);
            let truncated_cell = if index as u16 + 1 == number_of_parts_to_truncate {
                format!("{}", truncate_middle(&cell_size_opt.content, rect.width - 1 - current_position))
            } else {
                format!("{}", truncate_middle(&cell_size_opt.content, (rect.width - 1 - current_position ) / number_of_parts_to_truncate))
            };
            buf.set_string(current_position, rect.y, &truncated_cell, style);
            current_position += truncated_cell.chars().count() as u16;
        }
        if self.loading {
            let text_length = current_position - (rect.x + 1);
            self.draw_loading_chars(text_length, rect, buf);
        }
    }
    fn draw_loading_chars (&self, text_length: u16, rect: Rect, buf: &mut Buffer) {
        let index_in_text = (self.loading_indicator as u16 % (text_length)) as u16;
        buf.get_mut(rect.x + 1 + index_in_text, rect.y).set_modifier(Modifier::BOLD);
        if index_in_text >= text_length - 2 {
            buf.get_mut(rect.x + 1, rect.y).set_modifier(Modifier::BOLD);
            buf.get_mut(rect.x + 2, rect.y).set_modifier(Modifier::BOLD);
        } else {
            buf.get_mut(rect.x + 1 + index_in_text + 1, rect.y).set_modifier(Modifier::BOLD);
            buf.get_mut(rect.x + 1 + index_in_text + 2, rect.y).set_modifier(Modifier::BOLD);
        }
    }
    fn line_index_len (&self, i: usize) -> usize {
        let line_candidate_left = self.left_side_candidate(i);
        let line_candidate_right = self.right_side_candidate(i);
        let left_candidate_len = line_candidate_left
            .iter()
            .fold(0, |len, c| len + c.content.len());
        let right_candidate_len = line_candidate_right
            .iter()
            .fold(0, |len, c| len + c.content.len());
        let pipe_separator_len = 3;
        left_candidate_len + right_candidate_len + pipe_separator_len
    }
}
