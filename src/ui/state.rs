use tui::layout::Rect;

use crate::ui::Tiles;
use crate::input::{FileOrFolder, Folder};
use ::std::fmt;
use std::path::PathBuf;

pub struct DisplaySize(pub f64);

impl fmt::Display for DisplaySize{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 > 999_999_999.0 {
            write!(f, "{:.1}G", self.0 / 1_000_000_000.0)
        } else if self.0 > 999_999.0 {
            write!(f, "{:.1}M", self.0 / 1_000_000.0)
        } else if self.0 > 999.0 {
            write!(f, "{:.1}K", self.0 / 1000.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilePercentage {
    pub file_name: String,
    pub percentage: f64,
    pub actual_file_name: String,
}

pub struct State {
    pub tiles: Tiles,
    pub base_folder: Option<Folder>,
    pub path_in_filesystem: Option<String>,
    pub current_folder_names: Vec<String>,
}

impl State {
    pub fn new() -> Self {
        Self {
            tiles: Tiles::new(),
            base_folder: None,
            path_in_filesystem: None,
            current_folder_names: Vec::new(),
        }
    }
    pub fn set_base_folder(&mut self, base_folder: Folder, path_in_filesystem: String) {
        self.base_folder = Some(base_folder);
        self.path_in_filesystem = Some(path_in_filesystem);
        self.update_files();

    }
    pub fn update_files(&mut self) {
        if let Some(base_folder) = &self.base_folder {
            let current_folder = base_folder.path(&self.current_folder_names);
            let file_percentages = calculate_percentages(current_folder.expect("could not find current folder"));
            self.tiles.change_files(file_percentages);
        }
    }
    pub fn get_current_path(&self) -> Option<PathBuf> {
        if let Some(path_in_filesystem) = &self.path_in_filesystem {
            let mut full_path = PathBuf::from(&path_in_filesystem);
            for folder in &self.current_folder_names {
                full_path.push(&folder)
            }
            return Some(full_path);
        }
        None
    }
    pub fn change_size(&mut self, full_screen: Rect) {
        self.tiles.change_area(&full_screen); // TODO: move?
    }
    pub fn move_selected_right (&mut self) {
        self.tiles.move_selected_right();
    }
    pub fn move_selected_left(&mut self) {
        self.tiles.move_selected_left();
    }
    pub fn move_selected_down(&mut self) {
        self.tiles.move_selected_down();
    }
    pub fn move_selected_up(&mut self) {
        self.tiles.move_selected_up();
    }
    pub fn enter_selected(&mut self) {
        if let Some(base_folder) = &self.base_folder {
            if let Some(file_percentage) = &self.tiles.currently_selected() {
                let path_to_selected = &mut self.current_folder_names.clone();
                path_to_selected.push(String::from(&file_percentage.file_name));
                if let Some(_) = base_folder.path(&path_to_selected) {
                    // there is a folder at this path!
                    self.current_folder_names.push(String::from(&file_percentage.file_name));
                    self.update_files();
                    self.tiles.set_selected_index(&0);
                }
            }
        }
    }
    pub fn go_up(&mut self) {
        self.current_folder_names.pop();
        self.update_files();
    }
}

pub fn calculate_percentages (folder: &Folder) -> Vec<FilePercentage> {
    let mut file_percentages = Vec::new();
    let total_size = folder.size();
    for (name, file_or_folder) in &folder.contents {
        match file_or_folder {
            FileOrFolder::Folder(folder) => {
                let size = folder.size();
                let percentage = size as f64 / total_size as f64;
                let file_percentage = FilePercentage {
                    file_name: format!("{}/ {} ({:.0}%)", name, DisplaySize(size as f64),percentage * 100.0),
                    actual_file_name: String::from(name), // TODO: better
                    percentage,
                };
                file_percentages.push(file_percentage);
            },
            FileOrFolder::File(file) => {
                let size = file.size;
                let percentage = size as f64 / total_size as f64;
                let file_percentage = FilePercentage {
                    file_name: format!("{} {} ({:.0}%)", name, DisplaySize(size as f64),percentage * 100.0),
                    actual_file_name: String::from(name),
                    percentage,
                };
                file_percentages.push(file_percentage);
            }
        }
    }

    file_percentages.sort_by(|a, b| {
        if a.percentage == b.percentage {
            a.file_name.partial_cmp(&b.file_name).unwrap()
        } else {
            b.percentage.partial_cmp(&a.percentage).unwrap()
        }
    });
    file_percentages
}
