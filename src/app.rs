use ::std::fs::{self, Metadata};
use ::std::mem::ManuallyDrop;
use ::std::path::PathBuf;
use ::std::sync::mpsc::{Receiver, SyncSender};
use ::tui::backend::Backend;

use crate::messages::{handle_instructions, Instruction};
use crate::state::files::{FileOrFolder, FileTree, Folder};
use crate::state::tiles::Board;
use crate::state::{FileToDelete, UiEffects};
use crate::ui::Display;
use crate::Event;

#[derive(Clone)]
pub enum UiMode {
    Loading,
    Normal,
    ScreenTooSmall,
    DeleteFile(FileToDelete),
    ErrorMessage(String),
}

pub struct App<B>
where
    B: Backend,
{
    pub is_running: bool,
    pub loaded: bool,
    pub ui_mode: UiMode,
    board: Board,
    file_tree: ManuallyDrop<FileTree>,
    display: Display<B>,
    event_sender: SyncSender<Event>,
    ui_effects: UiEffects,
}

impl<B> App<B>
where
    B: Backend,
{
    pub fn new(
        terminal_backend: B,
        path_in_filesystem: PathBuf,
        event_sender: SyncSender<Event>,
    ) -> Self {
        let display = Display::new(terminal_backend);
        let board = Board::new(&Folder::new(&path_in_filesystem));
        let base_folder = Folder::new(&path_in_filesystem);
        let file_tree = ManuallyDrop::new(FileTree::new(base_folder, path_in_filesystem));
        // we use ManuallyDrop here because otherwise the app takes forever to exit
        let ui_effects = UiEffects::new();
        App {
            is_running: true,
            loaded: false,
            board,
            file_tree,
            display,
            ui_mode: UiMode::Loading,
            event_sender,
            ui_effects,
        }
    }
    pub fn start(&mut self, receiver: Receiver<Instruction>) {
        handle_instructions(self, receiver);
        self.display.clear();
    }
    pub fn render_and_update_board(&mut self) {
        let current_folder = self.file_tree.get_current_folder();
        self.board.change_files(&current_folder);
        self.render();
    }
    pub fn increment_loading_progress_indicator(&mut self) {
        self.ui_effects.increment_loading_progress_indicator();
    }
    pub fn render(&mut self) {
        let full_screen_size = self.display.size();
        if full_screen_size.width < 50 || full_screen_size.height < 15 {
            self.ui_mode = UiMode::ScreenTooSmall;
        }
        self.display.render(
            &mut self.file_tree,
            &mut self.board,
            &self.ui_mode,
            &self.ui_effects,
        );
    }
    pub fn flash_space_freed(&mut self) {
        self.ui_effects.flash_space_freed = true;
    }
    pub fn unflash_space_freed(&mut self) {
        self.ui_effects.flash_space_freed = false;
    }
    pub fn set_path_to_red(&mut self) {
        self.ui_effects.current_path_is_red = true;
    }
    pub fn reset_current_path_color(&mut self) {
        self.ui_effects.current_path_is_red = false;
    }
    pub fn start_ui(&mut self) {
        self.ui_mode = UiMode::Normal;
        self.loaded = true;
        self.render_and_update_board();
    }
    pub fn add_entry_to_base_folder(&mut self, file_metadata: &Metadata, entry_path: PathBuf) {
        self.file_tree.add_entry(file_metadata, &entry_path);
        self.ui_effects.last_read_path = Some(entry_path);
    }
    pub fn reset_ui_mode(&mut self) {
        match self.ui_mode {
            UiMode::Loading | UiMode::Normal => {}
            _ => {
                self.ui_mode = {
                    if self.loaded {
                        UiMode::Normal
                    } else {
                        UiMode::Loading
                    }
                }
            }
        };
    }
    pub fn exit(&mut self) {
        self.is_running = false;
        // here we do a blocking send rather than a try_send
        // because we want to make sure that if the receiver
        // is active, it received this event so that the app
        // would exit cleanly
        let _ = self.event_sender.send(Event::AppExit);
    }
    pub fn handle_enter(&mut self) {
        if !self.board.has_selected_index() {
            self.board.move_to_largest_folder();
        }
        self.enter_selected();
    }
    pub fn move_selected_right(&mut self) {
        self.board.move_selected_right();
        self.render();
    }
    pub fn move_selected_left(&mut self) {
        self.board.move_selected_left();
        self.render();
    }
    pub fn move_selected_down(&mut self) {
        self.board.move_selected_down();
        self.render();
    }
    pub fn move_selected_up(&mut self) {
        self.board.move_selected_up();
        self.render();
    }
    pub fn enter_selected(&mut self) {
        if let Some(index) = &self.board.get_selected_index() {
            &self.board.push_previous_index(&index);
        }
        if let Some(tile) = &self.board.currently_selected() {
            let selected_name = &tile.name;
            if let Some(file_or_folder) = self.file_tree.item_in_current_folder(&selected_name) {
                match file_or_folder {
                    FileOrFolder::Folder(_) => {
                        self.file_tree.enter_folder(&selected_name);
                        self.board.reset_selected_index();
                        self.render_and_update_board();
                    }
                    FileOrFolder::File(_) => {} // do not enter if currently_selected is a file
                }
            };
        }
    }
    pub fn go_up(&mut self) {
        if let Some(tile) = self.board.pop_previous_index() {
            self.board.set_selected_index(&tile);
        }
        let succeeded = self.file_tree.leave_folder();
        self.render_and_update_board();
        if !succeeded {
            let _ = self.event_sender.try_send(Event::PathError);
        }
    }
    pub fn get_file_to_delete(&self) -> Option<FileToDelete> {
        let currently_selected = self.board.currently_selected()?;
        let mut path_to_file = self.file_tree.current_folder_names.clone();
        path_to_file.push(currently_selected.name.clone());
        let file_to_delete = FileToDelete {
            path_in_filesystem: self.file_tree.path_in_filesystem.clone(),
            path_to_file,
            file_type: currently_selected.file_type,
            num_descendants: currently_selected.descendants,
            size: currently_selected.size,
        };
        Some(file_to_delete)
    }
    pub fn prompt_file_deletion(&mut self) {
        if let Some(file_to_delete) = self.get_file_to_delete() {
            self.ui_mode = UiMode::DeleteFile(file_to_delete);
            self.render();
        }
    }
    pub fn normal_mode(&mut self) {
        self.ui_mode = UiMode::Normal;
        self.render_and_update_board();
    }
    pub fn delete_file(&mut self, file_to_delete: &FileToDelete) {
        self.ui_effects.deletion_in_progress = true;
        self.render();
        self.ui_effects.deletion_in_progress = false;

        let full_path = file_to_delete.full_path();
        match fs::metadata(&full_path) {
            Ok(metadata) => {
                let file_type = metadata.file_type();
                let file_removed = if file_type.is_dir() {
                    fs::remove_dir_all(&full_path)
                } else {
                    fs::remove_file(&full_path)
                };
                match file_removed {
                    Ok(_) => {
                        self.remove_file_from_ui(file_to_delete);
                        self.ui_mode = UiMode::Normal;
                        self.render_and_update_board();
                        let _ = self.event_sender.try_send(Event::FileDeleted);
                    }
                    Err(msg) => {
                        self.ui_mode = UiMode::ErrorMessage(format!("{}", msg));
                        self.render();
                    }
                };
            }
            Err(msg) => {
                self.ui_mode = UiMode::ErrorMessage(format!("{}", msg));
                self.render();
            }
        }
    }
    pub fn increment_failed_to_read(&mut self) {
        self.file_tree.failed_to_read += 1;
    }
    fn remove_file_from_ui(&mut self, file_to_delete: &FileToDelete) {
        self.file_tree.space_freed += file_to_delete.size;
        self.file_tree.delete_file(file_to_delete);
        self.board.reset_selected_index();
    }
}
