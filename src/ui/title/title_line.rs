use ::tui::layout::Rect;
use ::tui::widgets::Widget;
use ::tui::buffer::Buffer;
use ::tui::style::{Style, Color, Modifier};

use ::std::path::PathBuf;

use crate::ui::FolderInfo;
use crate::ui::format::DisplaySize;
use crate::ui::title::{TitleText, CellSizeOpt};

pub struct TitleLine <'a> {
    base_path_info: FolderInfo<'a>,
    current_path_info: FolderInfo<'a>,
    space_freed: u64,
    is_loading: bool,
    progress_indicator: u64,
    flash_space: bool,
    path_error: bool,
}

impl <'a>TitleLine<'a> {
    pub fn new(base_path_info: FolderInfo<'a>, current_path_info: FolderInfo<'a>, space_freed: u64) -> Self {
        Self {
            base_path_info,
            current_path_info,
            space_freed,
            progress_indicator: 0,
            is_loading: false,
            flash_space: false,
            path_error: false,
        }
    }
    pub fn is_loading(mut self) -> Self {
        self.is_loading = true;
        self
    }
    pub fn flash_space(mut self, flash_space: bool) -> Self {
        self.flash_space = flash_space;
        self
    }
    pub fn path_error(mut self, path_error: bool) -> Self {
        self.path_error = path_error;
        self
    }
    pub fn progress_indicator(mut self, progress_indicator: u64) -> Self {
        self.progress_indicator = progress_indicator;
        self
    }
}

impl <'a>Widget for TitleLine <'a>{
    fn draw(&mut self, rect: Rect, buf: &mut Buffer) {
        let base_path = &self.base_path_info.path.clone().into_os_string().into_string().expect("could not convert os string to string");
        let current_path = {
            let mut current_path_relative_to_base = PathBuf::new();
            let base_path_len = self.base_path_info.path.iter().count();
            for folder in self.current_path_info.path.iter().skip(base_path_len) {
                current_path_relative_to_base.push(folder);
            }
            current_path_relative_to_base.to_string_lossy().into_owned()
        };

        let separator = ::std::path::MAIN_SEPARATOR;
        let total_size = DisplaySize(self.base_path_info.size as f64);
        let total_descendants = &self.base_path_info.num_descendants;
        let current_folder_size = DisplaySize(self.current_path_info.size as f64);
        let current_folder_descendants = self.current_path_info.num_descendants;
        let space_freed = DisplaySize(self.space_freed as f64);

        let mut default_style = Style::default().fg(Color::Yellow);
        if !self.is_loading {
            default_style = default_style.modifier(Modifier::BOLD);
        };
        let mut title_text = TitleText::new(default_style);
        if self.is_loading {
            title_text.append_to_left_side(vec![
                CellSizeOpt::new(format!("Scanning: {} ({} files)", total_size, total_descendants)),
                CellSizeOpt::new(format!("Scanning: {}", total_size)),
                CellSizeOpt::new(format!("{}", total_size)),
            ]);
        } else {
            title_text.append_to_left_side(vec![
                CellSizeOpt::new(format!("Total: {} ({} files), freed: {}", total_size, total_descendants, space_freed)),
                CellSizeOpt::new(format!("Total: {}, freed: {}", total_size, space_freed)),
                CellSizeOpt::new(format!("Total: {}", total_size)),
                CellSizeOpt::new(format!("{}", total_size)),
            ]);
        };
        title_text.append_to_right_side(vec![
            CellSizeOpt::new(format!("{}", base_path)),
        ]);
        if current_path.len() > 0 {
            title_text.append_to_right_side(vec![
                CellSizeOpt::new(format!("{}{} ({}, {} files)", separator, current_path, current_folder_size, current_folder_descendants)).style(default_style.fg(Color::Green)),
                CellSizeOpt::new(format!("{}{} ({})", separator, current_path, current_folder_size)).style(default_style.fg(Color::Green)),
                CellSizeOpt::new(format!("{}{}", separator, current_path)).style(default_style.fg(Color::Green)),
            ]);
        }

        title_text
            .loading(self.is_loading, self.progress_indicator)
            .path_error(self.path_error)
            .size_flash(self.flash_space)
            .render(rect, buf);
    }
}
