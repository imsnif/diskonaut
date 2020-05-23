#[cfg(test)]
mod tests;

mod app;
mod state;
mod ui;
mod input;
mod messages;

use ::std::env;
use ::std::io;
use ::std::{thread, time};
use ::std::thread::park_timeout;
use ::std::sync::mpsc::{SyncSender, Sender, Receiver};
use ::std::sync::mpsc;
use ::termion::event::{Event as TermionEvent, Key};
use ::failure;
use ::termion::raw::IntoRawMode;
use ::tui::backend::TermionBackend;
use ::std::process;
use ::std::path::PathBuf;
use ::tui::backend::Backend;
use ::std::sync::atomic::{AtomicBool, Ordering};
use ::std::sync::Arc;
use ::walkdir::WalkDir;

use input::{KeyboardEvents,sigwinch};
use app::{App, UiMode};
use messages::{Event, Instruction, handle_events};

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
    let (on_sigwinch, cleanup) = sigwinch();

    let (event_sender, event_receiver): (Sender<Event>, Receiver<Event>) = mpsc::channel();
    let (instruction_sender, instruction_receiver): (SyncSender<Instruction>, Receiver<Instruction>) = mpsc::sync_channel(100);

    let running = Arc::new(AtomicBool::new(true));
    let loaded = Arc::new(AtomicBool::new(false));

    active_threads.push(
        thread::Builder::new()
            .name("event_executer".to_string())
            .spawn({
                let instruction_sender = instruction_sender.clone();
                || handle_events(event_receiver, instruction_sender)
            }).unwrap(),
    );

    active_threads.push(
        thread::Builder::new()
            .name("stdin_handler".to_string())
            .spawn({
                let instruction_sender = instruction_sender.clone();
                move || {
                    for evt in keyboard_events {
                        if let TermionEvent::Key(Key::Ctrl('c')) | TermionEvent::Key(Key::Char('q')) = evt {
                            // not ideal, but works in a pinch
                            let _ = instruction_sender.send(Instruction::Keypress(evt));
                            break;
                        }
                        match instruction_sender.send(Instruction::Keypress(evt)) {
                            Err(_) => break,
                            _ => {}
                        };
                    }
                }
            })
            .unwrap(),
    );

    active_threads.push(
        thread::Builder::new()
            .name("hd_scanner".to_string())
            .spawn({
                let path = path.clone();
                let instruction_sender = instruction_sender.clone();
                let loaded = loaded.clone();
                move || {
                    let path_length = path.components().count();

                    'scanning: for entry in WalkDir::new(&path).into_iter().filter_map(|e| e.ok()) {
                        if let Ok(file_metadata) = entry.metadata() {
                            match instruction_sender.send(Instruction::AddEntryToBaseFolder((file_metadata, entry, path_length))) {
                                Err(_) => break 'scanning,
                                _ => {}
                            };
                        }
                    }
                    let _ = instruction_sender.send(Instruction::StartUi);
                    loaded.store(true, Ordering::Release);
                }
            })
            .unwrap()
    );

    if SHOULD_SHOW_LOADING_ANIMATION {
        active_threads.push(
            thread::Builder::new()
                .name("loading_loop".to_string())
                .spawn({
                    let instruction_sender = instruction_sender.clone();
                    let loaded = loaded.clone();
                    let running = running.clone();
                    move || {
                        while running.load(Ordering::Acquire) && !loaded.load(Ordering::Acquire) {
                            let _ = instruction_sender.send(Instruction::ToggleScanningVisualIndicator);
                            let _ = instruction_sender.send(Instruction::RenderAndUpdateBoard);
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
                let instruction_sender = instruction_sender.clone();
                move || {
                    on_sigwinch(Box::new(move || {
                        let _ = instruction_sender.send(Instruction::ResetUiMode);
                        let _ = instruction_sender.send(Instruction::Render);
                    }));
                }
            })
            .unwrap(),
    );

    let mut app = App::new(terminal_backend, path.clone(), event_sender.clone());
    app.start(instruction_receiver);
    std::mem::forget(app); // dropping app is a long process and at this point we really don't need it
    running.store(false, Ordering::Release);
    cleanup();

    for thread_handler in active_threads {
        thread_handler.join().unwrap();
    }
}
