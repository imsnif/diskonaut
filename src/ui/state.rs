use tui::layout::Rect;

use crate::ui::Tiles;
use crate::input::{FileOrFolder, Folder};
use std::path::{Path, PathBuf};
use std::fs::Metadata;

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
    pub file_tree: FileTree,
    pub space_freed: u64,
    pub ui_mode: UiMode,
}

pub struct FileTree {
    base_folder: Folder,
    current_folder_names: Vec<String>,
    pub path_in_filesystem: PathBuf,
}

impl FileTree {
    pub fn new(base_folder: Folder, path_in_filesystem: PathBuf) -> Self {
        FileTree {
            base_folder,
            current_folder_names: Vec::new(),
            path_in_filesystem,
        }

    }
    pub fn get_total_size (&self) -> u64 {
        self.base_folder.size
    }
    pub fn get_current_folder (&self) -> &Folder {
        if self.current_folder_names.is_empty() {
            &self.base_folder
        } else if let Some(FileOrFolder::Folder(current_folder)) = self.base_folder.path(&self.current_folder_names) {
            &current_folder
        } else {
            // here we have something in current_folder_names but the last
            // one is somehow not a folder... this is a corrupted state
            unreachable!("couldn't find current folder size")
        }
    }
    pub fn get_current_folder_size (&self) -> u64 {
        self.get_current_folder().size
    }
    pub fn get_current_folder_percentage (&self) -> f64 {
        self.get_current_folder().size as f64 / self.base_folder.size as f64
    }
    pub fn get_relative_path (&self) -> PathBuf {
        let mut full_path = PathBuf::new();
        for folder in &self.current_folder_names {
            full_path.push(&folder)
        }
        return full_path;
    }
    pub fn get_current_path (&self) -> PathBuf {
        let mut full_path = PathBuf::from(&self.path_in_filesystem);
        for folder in &self.current_folder_names {
            full_path.push(&folder)
        }
        return full_path;
    }
    pub fn item_in_current_folder(&self, item_name: &str) -> Option<&FileOrFolder> {
        let current_folder = &self.get_current_folder();
        current_folder.path(&vec![String::from(item_name)])
    }
    pub fn enter_folder(&mut self, folder_name: &str) {
        self.current_folder_names.push(String::from(folder_name));
    }
    pub fn leave_folder(&mut self) {
        self.current_folder_names.pop();
    }
    pub fn delete_file(&mut self, file_name: &str) {
        let path_to_delete = &mut self.current_folder_names.clone();
        path_to_delete.push(String::from(file_name));
        self.base_folder.delete_path(&path_to_delete);
    }
    pub fn add_entry(&mut self, entry_metadata: &Metadata, entry_full_path: &Path, base_path_length: &usize) {
        self.base_folder.add_entry(entry_metadata, entry_full_path, base_path_length);
    }

}

impl State {
    pub fn new(path_in_filesystem: PathBuf) -> Self {
        let base_folder = Folder::new(&path_in_filesystem); // TODO: better
        let file_list = vec![];
        let tiles = Tiles::new(file_list);
        let file_tree = FileTree::new(base_folder, path_in_filesystem);
        Self {
            tiles,
            file_tree,
            space_freed: 0,
            ui_mode: UiMode::Loading,
        }
    }
    pub fn get_total_size (&self) -> u64 {
        self.file_tree.get_total_size()
    }
    pub fn get_current_folder_size (&self) -> u64 {
        self.file_tree.get_current_folder_size()
    }
    pub fn get_current_folder_percentage (&self) -> f64 {
        self.file_tree.get_current_folder_percentage()
    }
    pub fn get_relative_path (&self) -> PathBuf {
        self.file_tree.get_relative_path()
    }
    pub fn update_tiles(&mut self) {
        let current_folder = self.file_tree.get_current_folder();
        let file_list = calculate_utilization(current_folder);
        self.tiles.change_files(file_list); // TODO: rename to change_tiles
    }
    pub fn get_current_path(&self) -> PathBuf {
        self.file_tree.get_current_path()
    }
    pub fn get_path_of_file_to_delete(&self) -> Option<PathBuf> {
        let file_to_delete = self.get_file_to_delete()?;
        let mut path = self.file_tree.get_current_path();
        path.push(file_to_delete.name());
        Some(path)
    }
    pub fn get_file_to_delete(&self) -> Option<&FileOrFolder> {
        let currently_selected_name = &self.tiles.currently_selected()?.file_metadata.name;
        self.file_tree.item_in_current_folder(currently_selected_name)
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
            let selected_name = &file_size_rect.file_metadata.name;
            if let Some(file_or_folder) = self.file_tree.item_in_current_folder(&selected_name) {
                match file_or_folder {
                    FileOrFolder::Folder(_) => {
                        self.file_tree.enter_folder(&selected_name);
                        self.tiles.reset_selected_index();
                        self.update_tiles();
                    }
                    FileOrFolder::File(_) => {} // do not enter if currently_selected is a file
                }
            };
        }
    }
    pub fn go_up(&mut self) {
        self.file_tree.leave_folder();
        self.tiles.reset_selected_index();
        self.update_tiles();
    }
    pub fn prompt_file_deletion (&mut self) {
        if let Some(_) = self.get_file_to_delete() {
            self.ui_mode = UiMode::DeleteFile;
        }
    }
    pub fn normal_mode (&mut self) {
        self.ui_mode = UiMode::Normal;
    }
    pub fn reset_mode (&mut self) {
        match self.ui_mode {
            UiMode::Loading | UiMode::Normal => {},
            _ => self.normal_mode()
        };
    }
    pub fn delete_file (&mut self) {
        let currently_selected_name = &self.tiles.currently_selected().expect("could not find selected file to delete").file_metadata.name;
        let file_to_delete = &self.file_tree.item_in_current_folder(currently_selected_name).expect("could not find file to delete");
        self.space_freed += file_to_delete.size();
        self.file_tree.delete_file(currently_selected_name);
        self.tiles.reset_selected_index();
        self.update_tiles();
    }
    pub fn add_entry(&mut self, entry_metadata: &Metadata, entry_full_path: &Path, base_path_length: &usize) {
        self.file_tree.add_entry(entry_metadata, entry_full_path, base_path_length);
    }
}

pub fn calculate_utilization(folder: &Folder) -> Vec<FileMetadata> {
    let mut file_list = Vec::new();
    let total_size = folder.size;
    for (name, file_or_folder) in &folder.contents {
        file_list.push({
            let size = file_or_folder.size();
            let name = String::from(name);
            let (descendants, file_type) = match file_or_folder {
                FileOrFolder::Folder(folder) => (Some(folder.num_descendants), FileType::Folder),
                FileOrFolder::File(_file) => (None, FileType::File),
            };
            let percentage = size as f64 / total_size as f64;
            FileMetadata {
                size,
                name,
                descendants,
                percentage,
                file_type,
            }
        });
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
