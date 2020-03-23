
use ::tui::widgets::Widget;
use ::tui::Terminal;
use ::tui::backend::Backend;
use ::tui::widgets::{Block, Borders, Paragraph, Text};
use ::tui::layout::{Layout, Constraint, Direction, Alignment};
use ::tui::style::{Color, Style};

use crate::ui::state::State;
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
            state.set_tiles(chunks[1]);
            let current_path = if let Some(current_path) = state.get_current_path() {
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
            RectangleGrid::new((&state.tiles.as_ref().expect("fix this").rectangles).to_vec()).render(&mut f, chunks[1]);
            // RectangleGrid::new((*state.tiles).to_vec()).render(&mut f, full_screen);
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
