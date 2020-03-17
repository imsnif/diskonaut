use ::std::io::stdin;
use ::termion::input::TermRead;
use ::termion::event::Event;
use termion::event::Key;
use crate::App;

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


pub fn handle_keypress<B>(evt: Event, app: &mut App<B>)
where B: Backend // TODO: better
{
    match evt {
        Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) => {
            app.exit();
        }
        Event::Key(Key::Char('l')) => {
            app.move_selected_right();
        }
        Event::Key(Key::Char('h')) => {
            app.move_selected_left();
        }
        Event::Key(Key::Char('j')) => {
            app.move_selected_down();
        }
        Event::Key(Key::Char('k')) => {
            app.move_selected_up();
        }
        Event::Key(Key::Char('\n')) => {
            app.enter_selected();
        }
        Event::Key(Key::Esc) => {
            app.go_up();
        }
        _ => (),
    };
}
