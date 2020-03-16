#[cfg(test)]
mod tests;

mod display;
mod input;

use ::std::env;
use ::std::io;
use ::std::thread;
use ::std::thread::park;
use ::termion::event::Event;
use ::std::sync::atomic::{AtomicBool, Ordering};
use ::failure;
use ::termion::raw::IntoRawMode;
use ::tui::backend::TermionBackend;
use ::tui::widgets::Widget;
use ::tui::Terminal;
use ::std::process;
use ::std::path::PathBuf;
use ::tui::backend::Backend;
use ::std::sync::{Arc, Mutex};
use ::tui::widgets::{Block, Borders, Paragraph, Text};
use ::tui::layout::{Layout, Constraint, Direction, Alignment};
use ::tui::style::{Color, Style};

use input::scan_folder;
use input::handle_keypress;
use input::KeyboardEvents;
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
                    // TODO: move below to render
                    terminal.draw(|mut f| {
                        let full_screen = f.size();
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

                        // TODO: find out how to get rid of these
                        chunks[1].width -= 1;
                        chunks[1].height -= 1;
                        state.lock().unwrap().set_tiles(chunks[1]);
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
                        RectangleGrid::new((*state.lock().unwrap().tiles).to_vec()).render(&mut f, full_screen);
                    }).expect("failed to draw");
                    park();
                }
                terminal.clear().unwrap();
            }
        })
        .unwrap();

    let stop_running = {
        let display_handler = display_handler.thread().clone();
        let running = running.clone();
        move || {
            running.store(false, Ordering::Release);
            display_handler.unpark();
        }
    };

    let render = {
        let display_handler = display_handler.thread().clone();
        move || {
            display_handler.unpark();
        }
    };

    active_threads.push(
        thread::Builder::new()
            .name("stdin_handler".to_string())
            .spawn({
                let state = state.clone();
                move || {
                    for evt in keyboard_events {
                        let mut state = state.lock().expect("could not get state");
                        handle_keypress(evt, &stop_running, &render, &mut state);
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
