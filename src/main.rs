#[cfg(test)]
mod tests;

mod app;
mod state;
mod ui;
mod input;
mod events;

use ::std::env;
use ::std::io;
use ::std::{thread, time};
use ::std::thread::park_timeout;
use ::termion::event::{Event as TermionEvent};
use ::failure;
use ::termion::raw::IntoRawMode;
use ::tui::backend::TermionBackend;
use ::std::process;
use ::std::path::PathBuf;
use ::tui::backend::Backend;
use ::std::sync::{Arc, Mutex};
use ::walkdir::WalkDir;

use input::{
    KeyboardEvents,
    sigwinch,
    handle_keypress_loading_mode,
    handle_keypress_normal_mode,
    handle_keypress_delete_file_mode
};
use app::{App, UiMode};
use events::{Blinker, EventBus, Event};

#[cfg(not(test))]
const SHOULD_SHOW_LOADING_ANIMATION: bool = true;

#[cfg(test)]
const SHOULD_SHOW_LOADING_ANIMATION: bool = false;

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

pub fn start<B>(terminal_backend: B, keyboard_events: Box<dyn Iterator<Item = TermionEvent> + Send>, path: PathBuf)
where
    B: Backend + Send + 'static,
{
    let mut active_threads = vec![];

    let event_bus = Arc::new(Mutex::new(EventBus::new()));
    let app = Arc::new(Mutex::new(App::new(terminal_backend, path.clone(), event_bus.clone())));
    let blinker = Blinker::new(&app);
    {
        let mut event_bus = event_bus.lock().unwrap();
        event_bus.subscribe(Event::PathChange, blinker.blink_path_green());
        event_bus.subscribe(Event::PathError, blinker.blink_path_red());
        event_bus.subscribe(Event::FileDeleted, blinker.blink_space_freed());
    }

    let (on_sigwinch, cleanup) = sigwinch();

    active_threads.push(
        thread::Builder::new()
            .name("stdin_handler".to_string())
            .spawn({
                let app = app.clone();
                move || {
                    for evt in keyboard_events {
                        // TODO: consider abstracting this away with a weak pointer like with
                        // blinker
                        let mut app = app.lock().expect("could not get app");
                        match app.ui_mode {
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

    if SHOULD_SHOW_LOADING_ANIMATION {
        active_threads.push(
            thread::Builder::new()
                .name("loading_loop".to_string())
                .spawn({
                    let app = app.clone();
                    move || {
                        loop {
                            {
                                let mut app = app.lock().unwrap();
                                if let UiMode::Normal = app.ui_mode {
                                    break;
                                }
                                app.toggle_scanning_visual_indicator();
                                app.render_and_update_board();
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
