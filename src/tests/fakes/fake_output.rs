use ::std::collections::HashMap;
use ::std::io;
use ::std::sync::{Arc, Mutex};
use ratatui::backend::{Backend, ClearType, WindowSize};
use ratatui::buffer::Cell;
use ratatui::layout::{Position, Size};

#[derive(Hash, Debug, PartialEq, Eq)]
pub enum TerminalEvent {
    Clear,
    HideCursor,
    ShowCursor,
    GetCursor,
    Flush,
    Draw,
}

pub struct TestBackend {
    pub events: Arc<Mutex<Vec<TerminalEvent>>>,
    pub draw_events: Arc<Mutex<Vec<String>>>,
    terminal_width: Arc<Mutex<u16>>,
    terminal_height: Arc<Mutex<u16>>,
}

impl TestBackend {
    pub const fn new(
        log: Arc<Mutex<Vec<TerminalEvent>>>,
        draw_log: Arc<Mutex<Vec<String>>>,
        terminal_width: Arc<Mutex<u16>>,
        terminal_height: Arc<Mutex<u16>>,
    ) -> Self {
        Self {
            events: log,
            draw_events: draw_log,
            terminal_width,
            terminal_height,
        }
    }
}

#[derive(Hash, Eq, PartialEq)]
struct Point {
    x: u16,
    y: u16,
}

impl Backend for TestBackend {
    type Error = io::Error;

    fn clear_region(&mut self, clear_type: ClearType) -> io::Result<()> {
        if clear_type == ClearType::All {
            self.clear()
        } else {
            Ok(())
        }
    }

    fn clear(&mut self) -> io::Result<()> {
        self.events
            .lock()
            .expect("Failed to lock mutex")
            .push(TerminalEvent::Clear);
        Ok(())
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        self.events
            .lock()
            .expect("Failed to lock mutex")
            .push(TerminalEvent::HideCursor);
        Ok(())
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        self.events
            .lock()
            .expect("Failed to lock mutex")
            .push(TerminalEvent::ShowCursor);
        Ok(())
    }

    fn get_cursor(&mut self) -> io::Result<(u16, u16)> {
        self.events
            .lock()
            .expect("Failed to lock mutex")
            .push(TerminalEvent::GetCursor);
        Ok((0, 0))
    }

    fn set_cursor(&mut self, _x: u16, _y: u16) -> io::Result<()> {
        Ok(())
    }

    fn draw<'a, I>(&mut self, content: I) -> io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a Cell)>,
    {
        self.events
            .lock()
            .expect("Failed to lock mutex")
            .push(TerminalEvent::Draw);
        let mut string = String::with_capacity(content.size_hint().0 * 3);
        let mut coordinates = HashMap::new();
        for (x, y, cell) in content {
            coordinates.insert(Point { x, y }, cell);
        }
        let terminal_height = self.terminal_height.lock().expect("Failed to lock mutex");
        let terminal_width = self.terminal_width.lock().expect("Failed to lock mutex");
        for y in 0..*terminal_height {
            for x in 0..*terminal_width {
                match coordinates.get(&Point { x, y }) {
                    Some(cell) => {
                        // this will contain no style information at all
                        // should be good enough for testing
                        string.push_str(cell.symbol());
                    }
                    None => {
                        string.push(' ');
                    }
                }
            }
            string.push('\n');
        }
        self.draw_events
            .lock()
            .expect("Failed to lock mutex")
            .push(string);
        Ok(())
    }

    fn size(&self) -> io::Result<Size> {
        let terminal_height = self.terminal_height.lock().expect("Failed to lock mutex");
        let terminal_width = self.terminal_width.lock().expect("Failed to lock mutex");

        Ok(Size::new(*terminal_width, *terminal_height))
    }

    fn get_cursor_position(&mut self) -> io::Result<Position> {
        Ok(Position::new(0, 0))
    }

    fn set_cursor_position<P: Into<Position>>(&mut self, _position: P) -> io::Result<()> {
        Ok(())
    }

    fn window_size(&mut self) -> io::Result<WindowSize> {
        let terminal_height = self.terminal_height.lock().expect("Failed to lock mutex");
        let terminal_width = self.terminal_width.lock().expect("Failed to lock mutex");

        Ok(WindowSize {
            columns_rows: Size::new(*terminal_width, *terminal_height),
            pixels: Size::new(0, 0),
        })
    }

    fn flush(&mut self) -> io::Result<()> {
        self.events
            .lock()
            .expect("Failed to lock mutex")
            .push(TerminalEvent::Flush);
        Ok(())
    }
}
