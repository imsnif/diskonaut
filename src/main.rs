#[cfg(test)]
mod tests;

mod ui;
mod input;

use ::std::env;
use ::std::io;
use ::std::{thread, time};
use ::std::thread::park_timeout;
use ::termion::event::Event;
use ::failure;
use ::termion::raw::IntoRawMode;
use ::tui::backend::TermionBackend;
use ::std::process;
use ::std::path::{Path, PathBuf};
use ::tui::backend::Backend;
use ::std::sync::{Arc, Mutex};

use input::{handle_keypress_loading_mode, handle_keypress_normal_mode, handle_keypress_delete_file_mode};
use input::KeyboardEvents;
use input::sigwinch;
use ui::state::{State, UiMode};
use ui::Display;

use input::Folder;
use walkdir::WalkDir;

use std::fs;
use std::fs::Metadata;

#[cfg(not(test))]
const SHOULD_SHOW_LOADING_SCREEN: bool = true;

#[cfg(test)]
const SHOULD_SHOW_LOADING_SCREEN: bool = false;

fn main() {
    if let Err(err) = try_main() {
        println!("Error: {}", err);
        process::exit(2);
    }
}

fn try_main() -> Result<(), failure::Error> {
    match io::stdout().into_raw_mode() {
        Ok(stdout) => {
            let terminal_backend = TermionBackend::new(stdout);
            let keyboard_events = KeyboardEvents {};
            start(terminal_backend, Box::new(keyboard_events), env::current_dir()?);
        }
        Err(_) => failure::bail!(
            "Failed to get stdout: if you are trying to pipe 'bandwhich' you should use the --raw flag"
        ),
    }
    Ok(())
}

pub struct App <B>
where B: Backend
{
    pub is_running: bool,
    pub ui_state: State,
    pub display: Display<B>,
    pub size_read: u64, // TODO: move elsewhere
    pub files_read: u64, // TODO: move elsewhere
    pub last_file_read: Option<PathBuf>, // TODO: move elsewhere
    pub loaded: bool, // TODO: better
}

impl <B>App <B>
where B: Backend
{
    pub fn new (terminal_backend: B, path: PathBuf) -> Self {
        let display = Display::new(terminal_backend);
        let base_folder = Folder::new(&path); // TODO: better
        let path_in_filesystem = path.into_os_string().into_string().expect("could not convert path to string"); // TODO: no clone
        let ui_state = State::new(base_folder, path_in_filesystem); // TODO: change name to UiState
        App { loaded: false, is_running: true, ui_state, display, size_read: 0, files_read: 0, last_file_read: None }
    }
    pub fn render_and_update_files (&mut self) {
        self.ui_state.update_files();
        self.render();
    }
    pub fn render (&mut self) {
        self.display.render(&mut self.ui_state);
    }
    pub fn start_ui(&mut self) {
        self.loaded = true;
        self.ui_state.normal_mode();
        self.render_and_update_files();
    }
    pub fn add_entry_to_base_folder(&mut self, file_metadata: &Metadata, entry_path: &Path, path_length: &usize) {
        self.ui_state.base_folder.add_entry(file_metadata, entry_path, path_length);
    }
    pub fn reset_ui_mode (&mut self) {
        self.ui_state.reset_mode();
    }
    pub fn exit (&mut self) {
        self.is_running = false;
    }
    pub fn move_selected_right (&mut self) {
        self.ui_state.move_selected_right();
        self.render();
    }
    pub fn move_selected_left (&mut self) {
        self.ui_state.move_selected_left();
        self.render();
    }
    pub fn move_selected_down (&mut self) {
        self.ui_state.move_selected_down();
        self.render();
    }
    pub fn move_selected_up (&mut self) {
        self.ui_state.move_selected_up();
        self.render();
    }
    pub fn enter_selected (&mut self) {
        self.ui_state.enter_selected();
        self.render_and_update_files();
    }
    pub fn go_up (&mut self) {
        self.ui_state.go_up();
        self.render_and_update_files();
    }
    pub fn prompt_file_deletion(&mut self) {
        self.ui_state.prompt_file_deletion();
        self.render();
    }
    pub fn normal_mode(&mut self) {
        self.ui_state.normal_mode();
        self.render_and_update_files();
    }
    pub fn delete_file(&mut self) {
        let file_to_delete = self.ui_state.get_path_of_file_to_delete().expect("cannot find file to delete");
        let metadata = fs::metadata(&file_to_delete).expect("could not get file metadata");
        let file_type = metadata.file_type();
        if file_type.is_dir() {
            fs::remove_dir_all(file_to_delete).expect("failed to delete folder");
        } else {
            fs::remove_file(file_to_delete).expect("failed to delete file");
        }
        self.ui_state.delete_file();
        self.ui_state.normal_mode();
        self.render_and_update_files();
    }
}

pub fn start<B>(terminal_backend: B, keyboard_events: Box<dyn Iterator<Item = Event> + Send>, path: PathBuf)
where
    B: Backend + Send + 'static,
{
    let mut active_threads = vec![];

    let app = Arc::new(Mutex::new(App::new(terminal_backend, path.clone())));

    let (on_sigwinch, cleanup) = sigwinch();

    active_threads.push(
        thread::Builder::new()
            .name("stdin_handler".to_string())
            .spawn({
                let app = app.clone();
                move || {
                    for evt in keyboard_events {
                        let mut app = app.lock().expect("could not get app");
                        match app.ui_state.ui_mode {
                            UiMode::Loading => {
                                handle_keypress_loading_mode(evt, &mut app);
                            },
                            UiMode::Normal => {
                                handle_keypress_normal_mode(evt, &mut app);
                            },
                            UiMode::DeleteFile => {
                                handle_keypress_delete_file_mode(evt, &mut app);
                            }
                        }
                        if !app.is_running {
                            cleanup();
                            break;
                        }
                    }
                }
            })
            .unwrap(),
    );


    active_threads.push(
        thread::Builder::new()
            .name("hd_scanner".to_string())
            .spawn({
                let app = app.clone();
                let path = path.clone();
                move || {
                    let path_length = path.components().count();

                    for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                        if let Ok(file_metadata) = entry.metadata() {
                            let entry_path = entry.path();
                            app.lock().unwrap().add_entry_to_base_folder(&file_metadata, &entry_path, &path_length);
                        }
                    }
                    app.lock().unwrap().start_ui();
                }
            })
            .unwrap()
    );

    if SHOULD_SHOW_LOADING_SCREEN {
        active_threads.push(
            thread::Builder::new()
                .name("loading_loop".to_string())
                .spawn({
                    let app = app.clone();
                    move || {
                        loop {
                            {
                                let mut app = app.lock().unwrap();
                                if app.loaded {
                                    break;
                                }
                                app.render();
                            }
                            park_timeout(time::Duration::from_millis(100));
                        }
                    }
                })
                .unwrap()
        );
    }

    active_threads.push(
        thread::Builder::new()
            .name("resize_handler".to_string())
            .spawn({
                let app = app.clone();
                move || {
                    on_sigwinch(Box::new(move || { 
                        let mut app = app.lock().unwrap();
                        app.reset_ui_mode();
                        app.render();
                    }));
                }
            })
            .unwrap(),
    );

    for thread_handler in active_threads {
        thread_handler.join().unwrap();
    }
}
