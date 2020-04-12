#[cfg(test)]
mod tests;

mod ui;
mod input;

use ::std::env;
use ::std::io;
use ::std::thread;
use ::termion::event::Event;
use ::failure;
use ::termion::raw::IntoRawMode;
use ::tui::backend::TermionBackend;
use ::std::process;
use ::std::path::PathBuf;
use ::tui::backend::Backend;
use ::std::sync::{Arc, Mutex};

use input::scan_folder;
use input::{handle_keypress_normal_mode, handle_keypress_delete_file_mode};
use input::KeyboardEvents;
use input::sigwinch;
use ui::state::{State, UiMode};
use ui::Display;

use input::Folder;

use std::fs;

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
    pub ui_state: Option<State>,
    pub display: Display<B>,
}

impl <B>App <B>
where B: Backend
{
    pub fn new (terminal_backend: B) -> Self {
        let display = Display::new(terminal_backend);
        App { is_running: true, ui_state: None, display }
    }
    pub fn render (&mut self) {
        self.display.render(&mut self.ui_state.as_mut().expect("could not find ui state"));
    }
    pub fn start_ui(&mut self, base_folder: Folder, path: String) {
        let mut ui_state = State::new(base_folder, path);
        ui_state.update_files();
        self.ui_state = Some(ui_state);
        self.render();
    }
    pub fn reset_ui_mode (&mut self) {
        self.ui_state.as_mut().expect("could not find ui state").normal_mode();
    }
    pub fn exit (&mut self) {
        self.is_running = false;
    }
    pub fn move_selected_right (&mut self) {
        self.ui_state.as_mut().expect("could not find ui state").move_selected_right();
        self.render();
    }
    pub fn move_selected_left (&mut self) {
        self.ui_state.as_mut().expect("could not find ui state").move_selected_left();
        self.render();
    }
    pub fn move_selected_down (&mut self) {
        self.ui_state.as_mut().expect("could not find ui state").move_selected_down();
        self.render();
    }
    pub fn move_selected_up (&mut self) {
        self.ui_state.as_mut().expect("could not find ui state").move_selected_up();
        self.render();
    }
    pub fn enter_selected (&mut self) {
        self.ui_state.as_mut().expect("could not find ui state").enter_selected();
        self.render();
    }
    pub fn go_up (&mut self) {
        self.ui_state.as_mut().expect("could not find ui state").go_up();
        self.render();
    }
    pub fn prompt_file_deletion(&mut self) {
        self.ui_state.as_mut().expect("could not find ui state").prompt_file_deletion();
        self.render();
    }
    pub fn normal_mode(&mut self) {
        self.ui_state.as_mut().expect("could not find ui state").normal_mode();
        self.render();
    }
    pub fn delete_file(&mut self) {
        let file_to_delete = self.ui_state.as_mut().expect("could not find ui state").get_path_of_file_to_delete().expect("cannot find file to delete");
        let metadata = fs::metadata(&file_to_delete).expect("could not get file metadata");
        let file_type = metadata.file_type();
        if file_type.is_dir() {
            fs::remove_dir_all(file_to_delete).expect("failed to delete folder");
        } else {
            fs::remove_file(file_to_delete).expect("failed to delete file");
        }
        self.ui_state.as_mut().expect("could not find ui state").delete_file();
        self.ui_state.as_mut().expect("could not find ui state").normal_mode();
        self.render();
    }
}

pub fn start<B>(terminal_backend: B, keyboard_events: Box<dyn Iterator<Item = Event> + Send>, path: PathBuf)
where
    B: Backend + Send + 'static,
{
    let mut active_threads = vec![];

    let app = Arc::new(Mutex::new(App::new(terminal_backend)));

    let (on_sigwinch, cleanup) = sigwinch();

    active_threads.push(
        thread::Builder::new()
            .name("stdin_handler".to_string())
            .spawn({
                let app = app.clone();
                move || {
                    for evt in keyboard_events {
                        let mut app = app.lock().expect("could not get app");
                        match app.ui_state.as_ref().expect("could not find ui state").ui_mode {
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

    let base_folder = scan_folder(path.clone()); // TODO: better
    {
        let mut app = app.lock().unwrap();
        app.start_ui(base_folder, path.into_os_string().into_string().expect("could not convert path to string"));
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
