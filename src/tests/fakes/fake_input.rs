use ::std::{thread, time};
use crossterm::event::Event;

pub struct TerminalEvents {
    pub events: Vec<Option<Event>>,
}

impl TerminalEvents {
    pub fn new(mut events: Vec<Option<Event>>) -> Self {
        events.reverse(); // this is so that we do not have to shift the array
        TerminalEvents { events }
    }
}
impl Iterator for TerminalEvents {
    type Item = Event;
    fn next(&mut self) -> Option<Event> {
        match self.events.pop() {
            Some(ev) => match ev {
                Some(ev) => Some(ev),
                None => {
                    thread::sleep(time::Duration::from_millis(200));
                    self.next()
                }
            },
            None => None,
        }
    }
}
