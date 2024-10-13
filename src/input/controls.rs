use ::tui::backend::Backend;
use crossterm::event::Event;
use crossterm::event::KeyModifiers;
use crossterm::event::{read, KeyCode, KeyEvent};

use crate::state::FileToDelete;
use crate::App;

#[derive(Clone)]
pub struct TerminalEvents;

impl Iterator for TerminalEvents {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        Some(read().unwrap())
    }
}
macro_rules! key {
    (char $x:expr) => {
        Event::Key(KeyEvent {
            code: KeyCode::Char($x),
            modifiers: KeyModifiers::NONE,
            ..
        })
    };
    (shift $x:expr) => {
        Event::Key(KeyEvent {
            code: KeyCode::Char($x),
            modifiers: KeyModifiers::SHIFT,
            ..
        })
    };
    (ctrl $x:expr) => {
        Event::Key(KeyEvent {
            code: KeyCode::Char($x),
            modifiers: KeyModifiers::CONTROL,
            ..
        })
    };
    ($x:ident) => {
        Event::Key(KeyEvent {
            code: KeyCode::$x,
            modifiers: KeyModifiers::NONE,
            ..
        })
    };
}

pub fn handle_keypress_loading_mode<B: Backend>(evt: Event, app: &mut App<B>) {
    match evt {
        key!(ctrl 'c') | key!(char 'q') => {
            app.prompt_exit();
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
        key!(char '+') | key!(shift '+') => {
            app.zoom_in();
        }
        key!(char '-') => {
            app.zoom_out();
        }
        key!(char '0') => {
            app.reset_zoom();
        }
        key!(char '\n') | key!(Enter) => {
            app.handle_enter();
        }
        key!(Backspace) => {
            app.show_warning_modal();
        }
        key!(Esc) => {
            app.go_up();
        }
        _ => (),
    };
}

pub fn handle_keypress_normal_mode<B: Backend>(evt: Event, app: &mut App<B>) {
    match evt {
        key!(ctrl 'c') | key!(char 'q') => {
            app.prompt_exit();
        }
        key!(Backspace) => {
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
        key!(char '+') | key!(shift '+') => {
            app.zoom_in();
        }
        key!(char '-') => {
            app.zoom_out();
        }
        key!(char '0') => {
            app.reset_zoom();
        }
        key!(char '\n') | key!(Enter) => {
            app.handle_enter();
        }
        key!(Esc) => {
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
        key!(ctrl 'c') | key!(char 'q') | key!(Esc) | key!(char 'n') => {
            app.normal_mode();
        }
        key!(char 'y') => {
            app.delete_file(&file_to_delete);
        }
        _ => (),
    };
}

pub fn handle_keypress_error_message<B: Backend>(evt: Event, app: &mut App<B>) {
    match evt {
        key!(ctrl 'c') | key!(char 'q') | key!(Esc) => {
            app.normal_mode();
        }
        _ => (),
    };
}

pub fn handle_keypress_screen_too_small<B: Backend>(evt: Event, app: &mut App<B>) {
    match evt {
        key!(ctrl 'c') | key!(char 'q') => {
            app.exit();
        }
        _ => (),
    };
}

pub fn handle_keypress_exiting_mode<B: Backend>(evt: Event, app: &mut App<B>) {
    match evt {
        key!(ctrl 'c') | key!(char 'q') | key!(Esc) | key!(char 'n') => {
            app.reset_ui_mode();
            // we have to manually call render here to make sure ui gets updated
            // because reset_ui_mode does not call it itself
            app.render();
        }
        key!(char 'y') => {
            app.exit();
        }
        _ => (),
    };
}

pub fn handle_keypress_warning_message<B: Backend>(evt: Event, app: &mut App<B>) {
    match evt {
        _ => {
            app.reset_ui_mode();
        }
    }
}
