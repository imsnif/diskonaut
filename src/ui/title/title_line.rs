use ::std::path::PathBuf;
use ::tui::buffer::Buffer;
use ::tui::layout::Rect;
use ::tui::style::{Color, Modifier, Style};
use ::tui::widgets::Widget;

use crate::ui::format::DisplaySize;
use crate::ui::title::{CellSizeOpt, TitleTelescope};
use crate::ui::FolderInfo;

use nix::unistd::geteuid;

pub struct TitleLine<'a> {
    base_path_info: FolderInfo<'a>,
    current_path_info: FolderInfo<'a>,
    space_freed: u64,
    show_loading: bool,
    progress_indicator: u64,
    read_errors: Option<u64>,
    flash_space: bool,
    path_error: bool,
    zoom_level: Option<usize>,
}

impl<'a> TitleLine<'a> {
    pub fn new(
        base_path_info: FolderInfo<'a>,
        current_path_info: FolderInfo<'a>,
        space_freed: u64,
    ) -> Self {
        Self {
            base_path_info,
            current_path_info,
            space_freed,
            progress_indicator: 0,
            read_errors: None,
            show_loading: false,
            flash_space: false,
            path_error: false,
            zoom_level: None,
        }
    }
    pub fn show_loading(mut self) -> Self {
        self.show_loading = true;
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
    pub fn read_errors(mut self, read_errors: u64) -> Self {
        if read_errors > 0 {
            self.read_errors = Some(read_errors);
        }
        self
    }
    pub fn zoom_level(mut self, zoom_level: usize) -> Self {
        if zoom_level > 0 {
            self.zoom_level = Some(zoom_level);
        }
        self
    }
}

impl<'a> Widget for TitleLine<'a> {
    fn render(self, rect: Rect, buf: &mut Buffer) {
        let base_path = &self
            .base_path_info
            .path
            .clone()
            .into_os_string()
            .into_string()
            .expect("could not convert os string to string");
        let current_path = {
            let mut current_path_relative_to_base = PathBuf::new();
            let base_path_len = self.base_path_info.path.iter().count();
            for folder in self.current_path_info.path.iter().skip(base_path_len) {
                current_path_relative_to_base.push(folder);
            }
            current_path_relative_to_base.to_string_lossy().into_owned()
        };

        let separator = if base_path.ends_with('/') {
            // eg. if base_path is "/", we don't want current path to
            // also start with "/" otherwise we'll have "//path_to_my/location"
            // instead of "/path_to_my/location"
            format!("")
        } else {
            format!("{}", ::std::path::MAIN_SEPARATOR)
        };
        let total_size = DisplaySize(self.base_path_info.size as f64);
        let total_descendants = &self.base_path_info.num_descendants;
        let current_folder_size = DisplaySize(self.current_path_info.size as f64);
        let current_folder_descendants = self.current_path_info.num_descendants;
        let space_freed = DisplaySize(self.space_freed as f64);

        let mut default_style = Style::default().fg(Color::Yellow);
        if !self.show_loading {
            default_style = default_style.modifier(Modifier::BOLD);
        };
        let mut title_telescope = TitleTelescope::new(default_style);
        if self.show_loading {
            title_telescope.append_to_left_side(vec![
                CellSizeOpt::new(format!(
                    "Scanning: {} ({} files)",
                    total_size, total_descendants
                )),
                CellSizeOpt::new(format!("Scanning: {}", total_size)),
                CellSizeOpt::new(format!("{}", total_size)),
            ]);
        } else {
            title_telescope.append_to_left_side(vec![
                CellSizeOpt::new(format!(
                    "Total: {} ({} files), freed: {}",
                    total_size, total_descendants, space_freed
                )),
                CellSizeOpt::new(format!("Total: {}, freed: {}", total_size, space_freed)),
                CellSizeOpt::new(format!("Total: {}", total_size)),
                CellSizeOpt::new(format!("{}", total_size)),
            ]);
        };
        if let Some(read_errors) = self.read_errors {
            title_telescope.append_to_left_side(vec![
                CellSizeOpt::new(format!(" (failed to read {} files)", read_errors))
                    .style(default_style.fg(Color::Red)),
                CellSizeOpt::new(format!(" ({} errors)", read_errors))
                    .style(default_style.fg(Color::Red)),
                CellSizeOpt::new(" (errors)".to_string()).style(default_style.fg(Color::Red)),
            ]);
        }
        if geteuid().is_root() {
            title_telescope.append_to_left_side(vec![
                CellSizeOpt::new(format!(" (CAUTION: running as root)"))
                    .style(default_style.fg(Color::Red)),
                CellSizeOpt::new(format!(" (running as root)")).style(default_style.fg(Color::Red)),
                CellSizeOpt::new(" (root)".to_string()).style(default_style.fg(Color::Red)),
            ]);
        }
        title_telescope.append_to_right_side(vec![CellSizeOpt::new(base_path.to_string())]);
        if !current_path.is_empty() {
            title_telescope.append_to_right_side(vec![
                CellSizeOpt::new(format!(
                    "{}{} ({}, {} files)",
                    separator, current_path, current_folder_size, current_folder_descendants
                ))
                .style(default_style.fg(Color::Green)),
                CellSizeOpt::new(format!(
                    "{}{} ({})",
                    separator, current_path, current_folder_size
                ))
                .style(default_style.fg(Color::Green)),
                CellSizeOpt::new(format!("{}{}", separator, current_path))
                    .style(default_style.fg(Color::Green)),
            ]);
        }
        if let Some(zoom_level) = self.zoom_level {
            title_telescope.append_to_right_side(vec![
                CellSizeOpt::new(format!(
                    " (+{} larger file(s), zoom out to show)",
                    zoom_level
                ))
                .style(default_style.fg(Color::Green)),
                CellSizeOpt::new(format!(
                    " (+{} larger file(s))",
                    zoom_level
                ))
                .style(default_style.fg(Color::Green)),
            ]);
        }

        title_telescope
            .loading(self.show_loading, self.progress_indicator)
            .path_error(self.path_error)
            .size_flash(self.flash_space)
            .render(rect, buf);
    }
}
