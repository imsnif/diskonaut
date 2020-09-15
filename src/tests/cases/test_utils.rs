use ::std::iter;
use ::std::sync::{Arc, Mutex};
use crossterm::event::KeyModifiers;
use crossterm::event::{Event, KeyCode, KeyEvent};

use crate::tests::fakes::{KeyboardEvents, TerminalEvent, TestBackend};

macro_rules! key {
    (char $x:expr) => {
        Event::Key(KeyEvent {
            code: KeyCode::Char($x),
            modifiers: KeyModifiers::NONE,
        })
    };
    (ctrl $x:expr) => {
        Event::Key(KeyEvent {
            code: KeyCode::Char($x),
            modifiers: KeyModifiers::CONTROL,
        })
    };
    ($x:ident) => {
        Event::Key(KeyEvent {
            code: KeyCode::$x,
            modifiers: KeyModifiers::NONE,
        })
    };
}

pub fn sleep_and_quit_events(sleep_num: usize, quit_after_confirm: bool) -> Box<KeyboardEvents> {
    let mut events: Vec<Option<Event>> = iter::repeat(None).take(sleep_num).collect();
    events.push(Some(key!(ctrl 'c')));
    if quit_after_confirm {
        events.push(None);
        events.push(Some(key!(char 'y')));
    }
    Box::new(KeyboardEvents::new(events))
}

type BackendWithStreams = (
    Arc<Mutex<Vec<TerminalEvent>>>,
    Arc<Mutex<Vec<String>>>,
    TestBackend,
);
pub fn test_backend_factory(w: u16, h: u16) -> BackendWithStreams {
    let terminal_events: Arc<Mutex<Vec<TerminalEvent>>> = Arc::new(Mutex::new(Vec::new()));
    let terminal_draw_events: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    let backend = TestBackend::new(
        terminal_events.clone(),
        terminal_draw_events.clone(),
        Arc::new(Mutex::new(w)),
        Arc::new(Mutex::new(h)),
    );
    (terminal_events, terminal_draw_events, backend)
}
