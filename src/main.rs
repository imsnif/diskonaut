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
use input::handle_keypress;
use input::KeyboardEvents;
use ui::state::State;
use ui::Display;

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
}

impl <B>App <B>
where B: Backend
{
    pub fn new (terminal_backend: B) -> Self {
        let ui_state = State::new();
        let display = Display::new(terminal_backend);
        App { is_running: true, ui_state, display }
    }
    pub fn render (&mut self) {
        self.display.render(&mut self.ui_state);
    }
    pub fn exit (&mut self) {
        self.is_running = false;
    }
    pub fn move_selected_right (&mut self) {
        self.ui_state.move_selected_right();
        self.display.render(&mut self.ui_state);
    }
    pub fn move_selected_left (&mut self) {
        self.ui_state.move_selected_left();
        self.display.render(&mut self.ui_state);
    }
    pub fn move_selected_down (&mut self) {
        self.ui_state.move_selected_down();
        self.display.render(&mut self.ui_state);
    }
    pub fn move_selected_up (&mut self) {
        self.ui_state.move_selected_up();
        self.display.render(&mut self.ui_state);
    }
    pub fn enter_selected (&mut self) {
        self.ui_state.enter_selected();
        self.display.render(&mut self.ui_state);
    }
    pub fn go_up (&mut self) {
        self.ui_state.go_up();
        self.display.render(&mut self.ui_state);
    }
}

pub fn start<B>(terminal_backend: B, keyboard_events: Box<dyn Iterator<Item = Event> + Send>, path: PathBuf)
where
    B: Backend + Send + 'static,
{
    let mut active_threads = vec![];

    let app = Arc::new(Mutex::new(App::new(terminal_backend)));

    active_threads.push(
        thread::Builder::new()
            .name("stdin_handler".to_string())
            .spawn({
                let app = app.clone();
                move || {
                    for evt in keyboard_events {
                        let mut app = app.lock().expect("could not get app");
                        handle_keypress(evt, &mut app);
                        if !app.is_running {
                            break;
                        }
                    }
                }
            })
            .unwrap(),
    );

    let file_sizes = scan_folder(path.clone()); // TODO: better
    {
        let mut app = app.lock().unwrap();
        app.ui_state.set_base_folder(file_sizes, path.into_os_string().into_string().expect("could not convert path to string"));
        app.render();
    }

    for thread_handler in active_threads {
        thread_handler.join().unwrap();
    }
}
