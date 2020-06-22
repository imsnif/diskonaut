use ::std::io::stdin;
use ::termion::event::Event;
use ::termion::event::Key;
use ::termion::input::TermRead;
use ::tui::backend::Backend;

use crate::state::FileToDelete;
use crate::App;

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

macro_rules! key {
    (char $x:expr) => {
        Event::Key(Key::Char($x))
    };
    (ctrl $x:expr) => {
        Event::Key(Key::Ctrl($x))
    };
    ($x:ident) => {
        Event::Key(Key::$x)
    };
}

pub fn handle_keypress_loading_mode<B: Backend>(evt: Event, app: &mut App<B>) {
    match evt {
        Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) => {
            app.exit();
        }
        key!(char 'l') | key!(Right) | key!(ctrl 'f') => {
            app.move_selected_right();
        }
        key!(char 'h') | key!(Left) | key!(ctrl 'b') => {
            app.move_selected_left();
        }
        key!(char 'j') | key!(Down) | key!(ctrl 'n') => {
            app.move_selected_down();
        }
        key!(char 'k') | key!(Up) | key!(ctrl 'p') => {
            app.move_selected_up();
        }
        Event::Key(Key::Char('\n')) => {
            app.enter_selected();
        }
        Event::Key(Key::Esc) | Event::Key(Key::Backspace) => {
            app.go_up();
        }
        _ => (),
    };
}

pub fn handle_keypress_normal_mode<B: Backend>(evt: Event, app: &mut App<B>) {
    match evt {
        Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) => {
            app.exit();
        }
        Event::Key(Key::Ctrl('d')) => {
            app.prompt_file_deletion();
        }
        key!(char 'l') | key!(Right) | key!(ctrl 'f') => {
            app.move_selected_right();
        }
        key!(char 'h') | key!(Left) | key!(ctrl 'b') => {
            app.move_selected_left();
        }
        key!(char 'j') | key!(Down) | key!(ctrl 'n') => {
            app.move_selected_down();
        }
        key!(char 'k') | key!(Up) | key!(ctrl 'p') => {
            app.move_selected_up();
        }
        Event::Key(Key::Char('\n')) => {
            app.enter_selected();
        }
        Event::Key(Key::Esc) | Event::Key(Key::Backspace) => {
            app.go_up();
        }
        _ => (),
    };
}

pub fn handle_keypress_delete_file_mode<B: Backend>(
    evt: Event,
    app: &mut App<B>,
    file_to_delete: FileToDelete,
) {
    match evt {
        Event::Key(Key::Ctrl('c'))
        | Event::Key(Key::Char('q'))
        | Event::Key(Key::Esc)
        | Event::Key(Key::Backspace)
        | Event::Key(Key::Char('n')) => {
            app.normal_mode();
        }
        Event::Key(Key::Char('y')) => {
            app.delete_file(&file_to_delete);
        }
        _ => (),
    };
}

pub fn handle_keypress_error_message<B: Backend>(evt: Event, app: &mut App<B>) {
    match evt {
        Event::Key(Key::Ctrl('c'))
        | Event::Key(Key::Char('q'))
        | Event::Key(Key::Esc)
        | Event::Key(Key::Backspace) => {
            app.normal_mode();
        }
        _ => (),
    };
}

pub fn handle_keypress_screen_too_small<B: Backend>(evt: Event, app: &mut App<B>) {
    match evt {
        Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) => {
            app.exit();
        }
        _ => (),
    };
}
