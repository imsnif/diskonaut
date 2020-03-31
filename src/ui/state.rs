use tui::layout::Rect;

use crate::ui::Tiles;
use crate::input::{FileOrFolder, Folder};
use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum FileType {
    File,
    Folder,
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub name: String,
    pub size: u64,
    pub percentage: f64, // 1.0 is 100% (0.5 is 50%, etc.)
    pub file_type: FileType,
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
    pub fn get_current_folder_size (&self) -> Option<u64> {
        if let Some(base_folder) = &self.base_folder {
            let current_folder = base_folder.path(&self.current_folder_names);
            Some(current_folder?.size())
        } else {
            return None
        }
    }
    pub fn update_files(&mut self) {
        if let Some(base_folder) = &self.base_folder {
            let current_folder = base_folder.path(&self.current_folder_names);
            let file_list = calculate_utilization(current_folder.expect("could not find current folder"));
            self.tiles.change_files(file_list);
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
            if let Some(file_size_rect) = &self.tiles.currently_selected() {
                let path_to_selected = &mut self.current_folder_names.clone();
                path_to_selected.push(String::from(&file_size_rect.file_metadata.name));
                if let Some(_) = base_folder.path(&path_to_selected) {
                    // there is a folder at this path!
                    self.current_folder_names.push(String::from(&file_size_rect.file_metadata.name));
                    self.update_files();
                    self.tiles.reset_selected_index();
                }
            }
        }
    }
    pub fn go_up(&mut self) {
        self.current_folder_names.pop();
        self.update_files();
    }
}

pub fn calculate_utilization(folder: &Folder) -> Vec<FileMetadata> {
    let mut file_list = Vec::new();
    let total_size = folder.size();
    for (name, file_or_folder) in &folder.contents {
        match file_or_folder {
            FileOrFolder::Folder(folder) => {
                let size = folder.size();
                let percentage = size as f64 / total_size as f64;
                let file_metadata = FileMetadata {
                    name: String::from(name),
                    percentage,
                    size,
                    file_type: FileType::Folder
                };
                file_list.push(file_metadata);
            },
            FileOrFolder::File(file) => {
                let size = file.size;
                let percentage = size as f64 / total_size as f64;
                let file_metadata = FileMetadata {
                    name: String::from(name),
                    percentage,
                    size,
                    file_type: FileType::File
                };
                file_list.push(file_metadata);
            }
        }
    }

    file_list.sort_by(|a, b| {
        if a.percentage == b.percentage {
            a.name.partial_cmp(&b.name).unwrap()
        } else {
            b.percentage.partial_cmp(&a.percentage).unwrap()
        }
    });
    file_list
}
