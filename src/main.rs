#[allow(dead_code)]
mod util;

#[cfg(test)]
mod tests;

use std::env;
use std::io;
use ::std::thread;
use ::std::thread::park;

use ::std::io::stdin;
use ::termion::input::TermRead;
use ::termion::event::Event;
use ::std::sync::atomic::{AtomicBool, Ordering};

use failure;

use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::widgets::Widget;
use tui::Terminal;

use std::process;

use std::path::PathBuf;

use ::tui::backend::Backend;
use ::std::sync::{Arc, Mutex};

mod filesystem;
mod display;

use filesystem::scan_folder;
use display::state::State;
use display::RectangleGrid;

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


#[derive(Clone)]
pub struct KeyboardEvents;

impl Iterator for KeyboardEvents {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        match stdin().events().next() {
            Some(Ok(ev)) => Some(ev),
            _ => None,
        }
    }
}

use ::tui::widgets::{Block, Borders, Paragraph, Text};
use ::tui::layout::{Layout, Constraint, Direction, Alignment};
use ::tui::style::{Color, Style};

pub fn start<B>(terminal_backend: B, keyboard_events: Box<dyn Iterator<Item = Event> + Send>, path: PathBuf)
where
    B: Backend + Send + 'static,
{
    let mut active_threads = vec![];
    let running = Arc::new(AtomicBool::new(true));
    let mut terminal = Terminal::new(terminal_backend).expect("failed to create terminal");
    terminal.clear().expect("failed to clear terminal");
    terminal.hide_cursor().expect("failed to hide cursor");

    let state = Arc::new(Mutex::new(State::new()));

    let display_handler = thread::Builder::new()
        .name("display_handler".to_string())
        .spawn({
            let running = running.clone();
            let state = state.clone();
            move || {
                park();
                while running.load(Ordering::Acquire) {
                    terminal.draw(|mut f| {
                        let mut full_screen = f.size();
                        let mut chunks = Layout::default()
                            .direction(Direction::Vertical)
                            .margin(0)
                            .constraints(
                                [
                                    Constraint::Length(3),
                                    Constraint::Length(10),
                                ].as_ref()
                            )
                            .split(full_screen);

                        full_screen.width -= 1;
                        full_screen.height -= 1;
                        chunks[1].width -= 1;
                        chunks[1].height -= 1;
//                        println!("chunks[1] {:?}", chunks[1]);
//                        println!("chunks[1] {:?}", chunks[1]);
//                        println!("full_screen {:?}", full_screen);
//                        use std::process;
//                        process::exit(2);
                        state.lock().unwrap().set_tiles(chunks[1]);
                        // state.lock().unwrap().set_tiles(full_screen);
                        // TODO:
                        // * make a layout that would place the RectangleGrid in the middle of the
                        // screen
                        // * place a text box above it with the current path
                        let current_path = if let Some(current_path) = state.lock().unwrap().get_current_path() {
                            current_path.into_os_string().into_string().expect("could not convert os string to string")
                        } else {
                            String::from("N/A")
                        };
                        let text = [
                            Text::styled("\n", Style::default()),
                            Text::styled(current_path, Style::default().fg(Color::Green)),
                            Text::styled("\n", Style::default()),
                        ];
                        Paragraph::new(text.iter())
                            .block(Block::default().borders(Borders::NONE))
                            .style(Style::default())
                            .alignment(Alignment::Center)
                            .wrap(true)
                            .render(&mut f, chunks[0]);
                        // RectangleGrid::new((*state.lock().unwrap().tiles).to_vec()).render(&mut f, chunks[1]);
                        RectangleGrid::new((*state.lock().unwrap().tiles).to_vec()).render(&mut f, full_screen);
                    }).expect("failed to draw");
                    park();
                }
                terminal.clear().unwrap();
            }
        })
        .unwrap();

    active_threads.push(
        thread::Builder::new()
            .name("stdin_handler".to_string())
            .spawn({
                let running = running.clone();
                let state = state.clone();
                let display_handler = display_handler.thread().clone();
                move || {
                    for evt in keyboard_events {
                        match evt {
                            Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) => {
                                running.store(false, Ordering::Release);
                                display_handler.unpark();
                                break;
                            }
                            Event::Key(Key::Char('l')) => {
                                state.lock().unwrap().move_selected_right();
                                display_handler.unpark();
                            }
                            Event::Key(Key::Char('h')) => {
                                state.lock().unwrap().move_selected_left();
                                display_handler.unpark();
                            }
                            Event::Key(Key::Char('j')) => {
                                state.lock().unwrap().move_selected_down();
                                display_handler.unpark();
                            }
                            Event::Key(Key::Char('k')) => {
                                state.lock().unwrap().move_selected_up();
                                display_handler.unpark();
                            }
                            Event::Key(Key::Char('\n')) => {
                                state.lock().unwrap().enter_selected();
                                // TODO: do not unpark display_handler if the state did not change
                                // eg. we tried to enter a file
                                display_handler.unpark();
                            }
                            Event::Key(Key::Esc) => {
                                state.lock().unwrap().go_up();
                                display_handler.unpark();
                            }
                            _ => (),
                        };
                    }
                }
            })
            .unwrap(),
    );
    let display_handler_thread = display_handler.thread().clone(); // TODO: better
    active_threads.push(display_handler);

    let file_sizes = scan_folder(path.clone()); // TODO: better
    state.lock().unwrap().set_base_folder(file_sizes, path.into_os_string().into_string().expect("could not convert path to string"));
    display_handler_thread.unpark();
    for thread_handler in active_threads {
        thread_handler.join().unwrap()
    }

}
