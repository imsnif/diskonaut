use ::std::io::stdin;
use ::termion::input::TermRead;
use ::termion::event::Event;
use termion::event::Key;
use crate::App;
use crate::app::FileToDelete;

use ::tui::backend::Backend;

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


pub fn handle_keypress_loading_mode<B>(evt: Event, app: &mut App<B>)
where B: Backend // TODO: better
{
    match evt {
        Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) => {
            app.exit();
        }
        Event::Key(Key::Char('l')) | Event::Key(Key::Right) => {
            app.move_selected_right();
        }
        Event::Key(Key::Char('h')) | Event::Key(Key::Left) => {
            app.move_selected_left();
        }
        Event::Key(Key::Char('j')) | Event::Key(Key::Down) => {
            app.move_selected_down();
        }
        Event::Key(Key::Char('k')) | Event::Key(Key::Up) => {
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

pub fn handle_keypress_normal_mode<B>(evt: Event, app: &mut App<B>)
where B: Backend // TODO: better
{
    match evt {
        Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) => {
            app.exit();
        }
        Event::Key(Key::Ctrl('d')) => {
            app.prompt_file_deletion();
        }
        Event::Key(Key::Char('l')) | Event::Key(Key::Right) => {
            app.move_selected_right();
        }
        Event::Key(Key::Char('h')) | Event::Key(Key::Left) => {
            app.move_selected_left();
        }
        Event::Key(Key::Char('j')) | Event::Key(Key::Down) => {
            app.move_selected_down();
        }
        Event::Key(Key::Char('k')) | Event::Key(Key::Up) => {
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

pub fn handle_keypress_delete_file_mode<B>(evt: Event, app: &mut App<B>, file_to_delete: FileToDelete)
where B: Backend // TODO: better
{
    match evt {
        Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) | Event::Key(Key::Esc) | Event::Key(Key::Backspace) | Event::Key(Key::Char('n')) => {
            app.normal_mode();
        }
        Event::Key(Key::Char('y')) => {
            app.delete_file(&file_to_delete);
        }
        _ => (),
    };
}

pub fn handle_keypress_error_message<B>(evt: Event, app: &mut App<B>)
where B: Backend // TODO: better
{
    match evt {
        Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) | Event::Key(Key::Esc) | Event::Key(Key::Backspace) => {
            app.normal_mode();
        }
        _ => (),
    };
}

pub fn handle_keypress_screen_too_small<B>(evt: Event, app: &mut App<B>)
where B: Backend // TODO: better
{
    match evt {
        Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) => {
            app.exit();
        }
        _ => (),
    };
}
