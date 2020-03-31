
use ::tui::widgets::Widget;
use ::tui::Terminal;
use ::tui::backend::Backend;
use ::tui::layout::{Layout, Constraint, Direction};

use crate::ui::state::State;
use crate::ui::{TitleLine, BottomLine};
use crate::ui::RectangleGrid;

pub struct Display <B>
where B: Backend
{
    terminal: Terminal<B>
}

impl <B> Display<B>
where B: Backend
{
    pub fn new (terminal_backend: B) -> Self {
        let mut terminal = Terminal::new(terminal_backend).expect("failed to create terminal");
        terminal.clear().expect("failed to clear terminal");
        terminal.hide_cursor().expect("failed to hide cursor");
        Display { terminal }
    }
    pub fn render (&mut self, state: &mut State) {
        self.terminal.draw(|mut f| {
            if let (
                Some(current_path),
                Some(folder_size)
            ) = (state.get_current_path(), state.get_current_folder_size()) {
                let full_screen = f.size();
                let mut chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(0)
                    .constraints(
                        [
                            Constraint::Length(3),
                            Constraint::Min(10),
                            Constraint::Length(1),
                        ].as_ref()
                    )
                    .split(full_screen);

                // TODO: find out how to get rid of these
                chunks[1].width -= 1;
                chunks[1].height -= 1;
                state.change_size(chunks[1]);
                TitleLine::new(&current_path, folder_size).render(&mut f, chunks[0]);
                RectangleGrid::new((&state.tiles.rectangles).to_vec()).render(&mut f, chunks[1]);
                BottomLine::new().render(&mut f, chunks[2]);
            };
        }).expect("failed to draw");
    }
}

impl <B> Drop for Display<B>
where B: Backend
{
    fn drop(&mut self) {
        self.terminal.clear().expect("failed to clear terminal");
        self.terminal.show_cursor().expect("failed to show cursor");
    }
}
