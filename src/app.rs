use ::std::path::{Path, PathBuf};
use ::tui::backend::Backend;

use crate::input::{Folder, FileOrFolder};
use crate::ui::Display;
use crate::state::{FileTree, Tiles};

use std::fs;
use std::fs::Metadata;

#[derive(Clone, Copy)]
pub enum UiMode {
    Loading,
    Normal,
    DeleteFile,
}

pub struct App <B>
where B: Backend
{
    pub is_running: bool,
    pub tiles: Tiles,
    pub file_tree: FileTree,
    pub display: Display<B>,
    pub loaded: bool, // TODO: better
    pub ui_mode: UiMode,
}

impl <B>App <B>
where B: Backend
{
    pub fn new (terminal_backend: B, path_in_filesystem: PathBuf) -> Self {
        let display = Display::new(terminal_backend);
        let tiles = Tiles::new(&Folder::new(&path_in_filesystem));
        let base_folder = Folder::new(&path_in_filesystem); // TODO: better
        let file_tree = FileTree::new(base_folder, path_in_filesystem);
        App {
            is_running: true,
            tiles,
            file_tree,
            loaded: false,
            display,
            ui_mode: UiMode::Loading,
        }
    }
    pub fn render_and_update_tiles (&mut self) {
        let current_folder = self.file_tree.get_current_folder();
        self.tiles.change_files(&current_folder); // TODO: rename to change_tiles
        self.render();
    }
    pub fn render (&mut self) {
        self.display.render(&mut self.file_tree, &mut self.tiles, &self.ui_mode);
    }
    pub fn start_ui(&mut self) {
        self.loaded = true;
        self.ui_mode = UiMode::Normal;
        self.render_and_update_tiles();
    }
    pub fn add_entry_to_base_folder(&mut self, file_metadata: &Metadata, entry_path: &Path, path_length: &usize) {
        self.file_tree.add_entry(file_metadata, entry_path, path_length);
    }
    pub fn reset_ui_mode (&mut self) {
        match self.ui_mode {
            UiMode::Loading | UiMode::Normal => {},
            _ => self.ui_mode = UiMode::Normal,
        };
    }
    pub fn exit (&mut self) {
        self.is_running = false;
    }
    pub fn move_selected_right (&mut self) {
        self.tiles.move_selected_right();
        self.render();
    }
    pub fn move_selected_left (&mut self) {
        self.tiles.move_selected_left();
        self.render();
    }
    pub fn move_selected_down (&mut self) {
        self.tiles.move_selected_down();
        self.render();
    }
    pub fn move_selected_up (&mut self) {
        self.tiles.move_selected_up();
        self.render();
    }
    pub fn enter_selected (&mut self) {
        if let Some(file_size_rect) = &self.tiles.currently_selected() {
            let selected_name = &file_size_rect.file_metadata.name;
            if let Some(file_or_folder) = self.file_tree.item_in_current_folder(&selected_name) {
                match file_or_folder {
                    FileOrFolder::Folder(_) => {
                        self.file_tree.enter_folder(&selected_name);
                        self.tiles.reset_selected_index();
                        self.render_and_update_tiles();
                    }
                    FileOrFolder::File(_) => {} // do not enter if currently_selected is a file
                }
            };
        }
    }
    pub fn go_up (&mut self) {
        self.file_tree.leave_folder();
        self.tiles.reset_selected_index();
        self.render_and_update_tiles();
    }
    pub fn get_file_to_delete(&self) -> Option<&FileOrFolder> {
        let currently_selected_name = &self.tiles.currently_selected()?.file_metadata.name;
        self.file_tree.item_in_current_folder(currently_selected_name)
    }
    pub fn prompt_file_deletion(&mut self) {
        if let Some(_) = self.get_file_to_delete() {
            self.ui_mode = UiMode::DeleteFile;
            self.render();
        }
    }
    pub fn normal_mode(&mut self) {
        self.ui_mode = UiMode::Normal;
        self.render_and_update_tiles();
    }
    pub fn get_path_of_file_to_delete(&self) -> Option<PathBuf> {
        let file_to_delete = self.get_file_to_delete()?;
        let mut path = self.file_tree.get_current_path();
        path.push(file_to_delete.name());
        Some(path)
    }
    pub fn delete_file(&mut self) {
        let file_to_delete = self.get_path_of_file_to_delete().expect("cannot find file to delete");
        let metadata = fs::metadata(&file_to_delete).expect("could not get file metadata");
        let file_type = metadata.file_type();
        if file_type.is_dir() {
            fs::remove_dir_all(file_to_delete).expect("failed to delete folder");
        } else {
            fs::remove_file(file_to_delete).expect("failed to delete file");
        }
        let currently_selected_name = &self.tiles.currently_selected().expect("could not find selected file to delete").file_metadata.name;
        let file_to_delete = &self.file_tree.item_in_current_folder(currently_selected_name).expect("could not find file to delete");
        self.file_tree.space_freed += file_to_delete.size();
        self.file_tree.delete_file(currently_selected_name);
        self.tiles.reset_selected_index();
        self.ui_mode = UiMode::Normal;
        self.render_and_update_tiles();
    }
}
