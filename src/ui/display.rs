
use ::tui::widgets::Widget;
use ::tui::Terminal;
use ::tui::backend::Backend;
use ::tui::layout::{Layout, Constraint, Direction};

use crate::state::files::FileTree;
use crate::ui::{TitleLine, BottomLine, MessageBox};
use crate::ui::RectangleGrid;
use crate::state::Tiles;
use crate::UiMode;

pub struct Display <B>
where B: Backend
{
    terminal: Terminal<B>,
    scan_boolean: bool, // TODO: elsewhere - this is to determine whether "Sanning folder..." should be bold or not
}

impl <B> Display<B>
where B: Backend
{
    pub fn new (terminal_backend: B) -> Self {
        let mut terminal = Terminal::new(terminal_backend).expect("failed to create terminal");
        terminal.clear().expect("failed to clear terminal");
        terminal.hide_cursor().expect("failed to hide cursor");
        Display { terminal, scan_boolean: true }
    }
    pub fn render (&mut self, file_tree: &mut FileTree, tiles: &mut Tiles, ui_mode: &UiMode) { // TODO: change name to render_ui
        let scan_boolean = self.scan_boolean;
        self.scan_boolean = !scan_boolean;
        self.terminal.draw(|mut f| {
            let current_path = file_tree.get_current_path();
            let current_path_size = file_tree.get_current_folder_size();
            let base_path_size = file_tree.get_total_size();
            let full_screen = f.size();
            let mut chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Min(10),
                        Constraint::Length(2),
                    ].as_ref()
                )
                .split(full_screen);

            // TODO: find out how to get rid of these
            chunks[1].width -= 1;
            chunks[1].height -= 1;
            tiles.change_area(&chunks[1]);
            let path_in_filesystem = &file_tree.path_in_filesystem;
            match ui_mode {
                UiMode::Loading => {
                    TitleLine::new(&path_in_filesystem, base_path_size, &current_path, current_path_size, file_tree.space_freed, scan_boolean).show_loading().render(&mut f, chunks[0]);
                    RectangleGrid::new((&tiles.rectangles).to_vec()).render(&mut f, chunks[1]);
                    BottomLine::new().hide_delete().render(&mut f, chunks[2]);
                },
                UiMode::Normal => {
                    TitleLine::new(&path_in_filesystem, base_path_size, &current_path, current_path_size, file_tree.space_freed, scan_boolean).render(&mut f, chunks[0]);
                    RectangleGrid::new((&tiles.rectangles).to_vec()).render(&mut f, chunks[1]);
                    BottomLine::new().render(&mut f, chunks[2]);
                }
                UiMode::DeleteFile => {
                    let currently_selected_name = &tiles.currently_selected().expect("could not find currently selected file to delete").file_metadata.name;
                    let file_to_delete = file_tree.item_in_current_folder(&currently_selected_name).expect("could not find file to delete in current folder");
                    TitleLine::new(&path_in_filesystem, base_path_size, &current_path, current_path_size, file_tree.space_freed, scan_boolean).render(&mut f, chunks[0]);
                    RectangleGrid::new((&tiles.rectangles).to_vec()).render(&mut f, chunks[1]);
                    BottomLine::new().render(&mut f, chunks[2]);
                    MessageBox::new(file_to_delete, &current_path).render(&mut f, full_screen);
                },
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
