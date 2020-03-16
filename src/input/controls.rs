use ::std::io::stdin;
use ::termion::input::TermRead;
use ::termion::event::Event;
use termion::event::Key;
use crate::display::state::State;

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


pub fn handle_keypress(evt: Event, stop_running: &dyn Fn(), render: &dyn Fn(), state: &mut State) {
    match evt {
        Event::Key(Key::Ctrl('c')) | Event::Key(Key::Char('q')) => {
            stop_running();
        }
        Event::Key(Key::Char('l')) => {
            state.move_selected_right();
            render();
        }
        Event::Key(Key::Char('h')) => {
            state.move_selected_left();
            render();
        }
        Event::Key(Key::Char('j')) => {
            state.move_selected_down();
            render();
        }
        Event::Key(Key::Char('k')) => {
            state.move_selected_up();
            render();
        }
        Event::Key(Key::Char('\n')) => {
            state.enter_selected();
            // TODO: do not unpark display_handler if the state did not change
            // eg. we tried to enter a file
            render();
        }
        Event::Key(Key::Esc) => {
            state.go_up();
            render();
        }
        _ => (),
    };
}
