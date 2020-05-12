use ::std::sync::{Mutex, Arc};
use ::std::path::{Path, PathBuf};
use ::std::fs::{self, Metadata};
use ::tui::backend::Backend;

use crate::{EventBus, Event};
use crate::state::files::{Folder, FileOrFolder};
use crate::ui::Display;
use crate::state::Board;
use crate::state::files::FileTree;

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
    pub board: Board,
    pub file_tree: FileTree,
    pub display: Arc<Mutex<Display<B>>>,
    pub loaded: bool, // TODO: better
    pub ui_mode: UiMode,
    pub event_bus: Arc<Mutex<EventBus>>,
}

impl <B>App <B>
where B: Backend
{
    pub fn new (terminal_backend: B, path_in_filesystem: PathBuf, event_bus: Arc<Mutex<EventBus>>) -> Self {
        let display = Arc::new(Mutex::new(Display::new(terminal_backend)));
        let board = Board::new(&Folder::new(&path_in_filesystem));
        let base_folder = Folder::new(&path_in_filesystem); // TODO: better
        let file_tree = FileTree::new(base_folder, path_in_filesystem);
        App {
            is_running: true,
            board,
            file_tree,
            loaded: false,
            display,
            ui_mode: UiMode::Loading,
            event_bus,
        }
    }
    pub fn render_and_update_board (&mut self) {
        let current_folder = self.file_tree.get_current_folder();
        self.board.change_files(&current_folder); // TODO: rename to change_tiles
        self.render();
    }
    pub fn render (&mut self) {
        let path_should_blink = false;
        self.display.lock().unwrap().render(&mut self.file_tree, &mut self.board, &self.ui_mode, path_should_blink);
    }
    pub fn render_blinking_path(&mut self) {
        let path_should_blink = true;
        self.display.lock().unwrap().render(&mut self.file_tree, &mut self.board, &self.ui_mode, path_should_blink);
    }
    pub fn set_path_to_red(&mut self) {
        self.display.lock().unwrap().set_path_to_red();
    }
    pub fn reset_path_color(&mut self) {
        self.display.lock().unwrap().reset_path_color();
    }
    pub fn stop_blinking_path(&mut self) {
        let path_should_blink = false;
        let mut display = self.display.lock().unwrap();
        display.render(&mut self.file_tree, &mut self.board, &self.ui_mode, path_should_blink);
    }
    pub fn start_ui(&mut self) {
        self.loaded = true;
        self.ui_mode = UiMode::Normal;
        self.render_and_update_board();
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
        self.board.move_selected_right();
        self.render();
    }
    pub fn move_selected_left (&mut self) {
        self.board.move_selected_left();
        self.render();
    }
    pub fn move_selected_down (&mut self) {
        self.board.move_selected_down();
        self.render();
    }
    pub fn move_selected_up (&mut self) {
        self.board.move_selected_up();
        self.render();
    }
    pub fn enter_selected (&mut self) {
        if let Some(file_size_rect) = &self.board.currently_selected() {
            let selected_name = &file_size_rect.file_metadata.name;
            if let Some(file_or_folder) = self.file_tree.item_in_current_folder(&selected_name) {
                match file_or_folder {
                    FileOrFolder::Folder(_) => {
                        self.file_tree.enter_folder(&selected_name);
                        self.board.reset_selected_index();
                        self.render_and_update_board();
                        self.event_bus.lock().unwrap().publish(Event::PathChange);
                    }
                    FileOrFolder::File(_) => {} // do not enter if currently_selected is a file
                }
            };
        }
    }
    pub fn go_up (&mut self) {
        let succeeded = self.file_tree.leave_folder();
        self.board.reset_selected_index();
        self.render_and_update_board();
        if succeeded {
            self.event_bus.lock().unwrap().publish(Event::PathChange);
        } else {
            self.event_bus.lock().unwrap().publish(Event::PathError);
        }
    }
    pub fn get_file_to_delete(&self) -> Option<&FileOrFolder> {
        let currently_selected_name = &self.board.currently_selected()?.file_metadata.name;
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
        self.render_and_update_board();
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
        let currently_selected_name = &self.board.currently_selected().expect("could not find selected file to delete").file_metadata.name;
        let file_to_delete = &self.file_tree.item_in_current_folder(currently_selected_name).expect("could not find file to delete");
        self.file_tree.space_freed += file_to_delete.size();
        self.file_tree.delete_file(currently_selected_name);
        self.board.reset_selected_index();
        self.ui_mode = UiMode::Normal;
        self.render_and_update_board();
    }
}
