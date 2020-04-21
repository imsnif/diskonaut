use tui::layout::Rect;

use crate::ui::Tiles;
use crate::input::{FileOrFolder, Folder};
use std::path::PathBuf;

#[derive(Clone, Copy)]
pub enum UiMode {
    Loading,
    Normal,
    DeleteFile,
}

#[derive(Clone, Debug)]
pub enum FileType {
    File,
    Folder,
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub name: String,
    pub size: u64,
    pub descendants: Option<u64>,
    pub percentage: f64, // 1.0 is 100% (0.5 is 50%, etc.)
    pub file_type: FileType,
}

pub struct State {
    pub tiles: Tiles,
    pub base_folder: Folder,
    pub path_in_filesystem: String,
    pub current_folder_names: Vec<String>,
    pub space_freed: u64,
    pub ui_mode: UiMode,
}

impl State {
    pub fn new(base_folder: Folder, path_in_filesystem: String) -> Self {
        let file_list = calculate_utilization(&base_folder);
        let tiles = Tiles::new(file_list);
        Self {
            tiles,
            base_folder,
            path_in_filesystem,
            current_folder_names: Vec::new(),
            space_freed: 0,
            ui_mode: UiMode::Loading,
        }
    }
    pub fn get_total_size (&self) -> u64 {
        self.base_folder.size
        // self.base_folder.size()
    }
    pub fn get_current_folder_size (&self) -> u64 {
        if self.current_folder_names.is_empty() {
            self.base_folder.size
            // self.base_folder.size()
        } else if let Some(FileOrFolder::Folder(current_folder)) = self.base_folder.path(&self.current_folder_names) {
            current_folder.size
            // current_folder.size()
        } else {
            // here we have something in current_folder_names but the last
            // one is somehow not a folder... this is a corrupted state
            unreachable!("couldn't find current folder size")
        }
    }
    pub fn get_current_folder_percentage (&self) -> f64 {
        if self.current_folder_names.is_empty() {
            return 1.0 // 100%
        } else if let Some(FileOrFolder::Folder(current_folder)) = self.base_folder.path(&self.current_folder_names) {
            current_folder.size as f64 / self.base_folder.size as f64
            // current_folder.size() as f64 / self.base_folder.size() as f64
        } else {
            // here we have something in current_folder_names but the last
            // one is somehow not a folder... this is a corrupted state
            unreachable!("couldn't find current folder size")
        }
    }
    pub fn get_relative_path (&self) -> PathBuf {
        let mut full_path = PathBuf::new();
        for folder in &self.current_folder_names {
            full_path.push(&folder)
        }
        return full_path;
    }
    pub fn update_files(&mut self) {
        if self.current_folder_names.is_empty() {
            let file_list = calculate_utilization(&self.base_folder);
            self.tiles.change_files(file_list);
        } else if let Some(FileOrFolder::Folder(next_folder)) = self.base_folder.path(&self.current_folder_names) {
            let file_list = calculate_utilization(&next_folder);
            self.tiles.change_files(file_list);
        }
    }
    pub fn get_current_path(&self) -> PathBuf {
        let mut full_path = PathBuf::from(&self.path_in_filesystem);
        for folder in &self.current_folder_names {
            full_path.push(&folder)
        }
        return full_path;
    }
    pub fn get_path_of_file_to_delete(&self) -> Option<PathBuf> {
        let file_to_delete = &self.get_file_to_delete()?;
        let mut full_path = PathBuf::from(&self.path_in_filesystem);
        for folder in &self.current_folder_names {
            full_path.push(&folder);
        };
        full_path.push(file_to_delete.name());
        Some(full_path)
    }
    pub fn get_file_to_delete(&self) -> Option<&FileOrFolder> {
        if let Some(file_size_rect) = &self.tiles.currently_selected() {
            let path_to_selected = &mut self.current_folder_names.clone();
            path_to_selected.push(String::from(&file_size_rect.file_metadata.name));
            self.base_folder.path(&path_to_selected)
        } else {
            None
        }
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
    pub fn get_currently_selected(&self) -> Option<String> {
        if let Some(file_size_rect) = &self.tiles.currently_selected() {
            Some(String::from(&file_size_rect.file_metadata.name))
        } else {
            None
        }
    }
    pub fn enter_selected(&mut self) {
        if let Some(file_size_rect) = &self.tiles.currently_selected() {
            let path_to_selected = &mut self.current_folder_names.clone();
            path_to_selected.push(String::from(&file_size_rect.file_metadata.name));
            if let Some(file_or_folder) = self.base_folder.path(&path_to_selected) {
                // there is a folder at this path!
                match file_or_folder {
                    FileOrFolder::Folder(_) => {
                        self.current_folder_names.push(String::from(&file_size_rect.file_metadata.name));
                        self.tiles.reset_selected_index();
                        self.update_files();
                    },
                    FileOrFolder::File(_) => {}
                }
            }
        }
    }
    pub fn go_up(&mut self) {
        self.current_folder_names.pop();
        self.tiles.reset_selected_index();
        self.update_files();
    }
    pub fn prompt_file_deletion (&mut self) {
        if let Some(_) = self.get_file_to_delete() {
            self.ui_mode = UiMode::DeleteFile;
        }
    }
    pub fn normal_mode (&mut self) {
        self.ui_mode = UiMode::Normal;
    }
    pub fn delete_file (&mut self) {
        let path_to_delete = &mut self.current_folder_names.clone();
        let file_to_delete = self.get_file_to_delete().expect("could not find file to delete");
        path_to_delete.push(String::from(file_to_delete.name()));
        if let Some(file_or_folder_to_delete) = self.base_folder.path(&path_to_delete) {
            self.space_freed += match file_or_folder_to_delete {
                FileOrFolder::File(file) => file.size,
                FileOrFolder::Folder(folder) => folder.size,
                // FileOrFolder::Folder(folder) => folder.size(),
            };
        }
        self.base_folder.delete_path(&path_to_delete);
        self.tiles.reset_selected_index();
        self.update_files();
    }
}

pub fn calculate_utilization(folder: &Folder) -> Vec<FileMetadata> {
    let mut file_list = Vec::new();
    let total_size = folder.size;
    // let total_size = folder.size();
    for (name, file_or_folder) in &folder.contents {
        match file_or_folder {
            FileOrFolder::Folder(folder) => {
                let size = folder.size;
                // let size = folder.size();
                let descendants = Some(folder.num_descendants);
                // let descendants = Some(11);
                let percentage = size as f64 / total_size as f64;
                let file_metadata = FileMetadata {
                    name: String::from(name),
                    descendants,
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
                    descendants: None,
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
            a.name.partial_cmp(&b.name).expect("could not compare name")
        } else {
            b.percentage.partial_cmp(&a.percentage).expect("could not compare percentage")
        }
    });
    file_list
}
