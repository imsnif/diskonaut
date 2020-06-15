use ::tui::widgets::Widget;
use ::tui::Terminal;
use ::tui::backend::Backend;
use ::tui::layout::{Layout, Constraint, Direction, Rect};
use ::std::path::PathBuf;

use crate::state::files::FileTree;
use crate::ui::{TitleLine, BottomLine, MessageBox, ErrorBox, TermTooSmall};
use crate::ui::RectangleGrid;
use crate::state::{UiEffects, Board};
use crate::UiMode;

pub struct FolderInfo<'a> {
    pub path: &'a PathBuf,
    pub size: u64,
    pub num_descendants: u64,
}

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
    pub fn size (&self) -> Rect {
        self.terminal.size().expect("could not get terminal size")
    }
    pub fn render (&mut self, file_tree: &mut FileTree, board: &mut Board, ui_mode: &UiMode, ui_effects: &UiEffects) {
        self.terminal.draw(|mut f| {
            let full_screen = f.size();
            let current_path = file_tree.get_current_path();
            let current_path_size = file_tree.get_current_folder_size();
            let current_path_descendants = file_tree.get_current_folder().num_descendants;
            let base_path_size = file_tree.get_total_size();
            let base_path_descendants = file_tree.get_total_descendants();
            let current_path_info = FolderInfo { path: &current_path, size: current_path_size, num_descendants: current_path_descendants };
            let path_in_filesystem = &file_tree.path_in_filesystem;
            let base_path_info = FolderInfo { path: &path_in_filesystem, size: base_path_size, num_descendants: base_path_descendants };
            let mut chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(0)
                .constraints(
                    [
                        Constraint::Length(1),
                        Constraint::Min(10),
                        Constraint::Length(2),
                    ].as_ref()
                )
                .split(full_screen);

            // TODO: find out how to get rid of these
            chunks[1].width -= 1;
            chunks[1].height -= 1;
            board.change_area(&chunks[1]);
            match ui_mode {
                UiMode::Loading => {
                    TitleLine::new(base_path_info, current_path_info, file_tree.space_freed)
                        .progress_indicator(ui_effects.loading_progress_indicator)
                        .path_error(ui_effects.current_path_is_red)
                        .is_loading()
                        .render(&mut f, chunks[0]);
                    RectangleGrid::new(&board.tiles, board.unrenderable_tile_coordinates, board.selected_index).render(&mut f, chunks[1]);
                    BottomLine::new(file_tree.failed_to_read)
                        .currently_selected(board.currently_selected())
                        .last_read_path(ui_effects.last_read_path.as_ref())
                        .hide_delete()
                        .render(&mut f, chunks[2]);
                },
                UiMode::Normal => {
                    TitleLine::new(base_path_info, current_path_info, file_tree.space_freed)
                        .path_error(ui_effects.current_path_is_red)
                        .flash_space(ui_effects.frame_around_space_freed)
                        .render(&mut f, chunks[0]);
                    RectangleGrid::new(&board.tiles, board.unrenderable_tile_coordinates, board.selected_index).render(&mut f, chunks[1]);
                    BottomLine::new(file_tree.failed_to_read).currently_selected(board.currently_selected()).render(&mut f, chunks[2]);
                },
                UiMode::ScreenTooSmall => {
                    TermTooSmall::new().render(&mut f, full_screen);
                },
                UiMode::DeleteFile(file_to_delete) => {
                    TitleLine::new(base_path_info, current_path_info, file_tree.space_freed)
                        .path_error(ui_effects.current_path_is_red)
                        .render(&mut f, chunks[0]);
                    RectangleGrid::new(&board.tiles, board.unrenderable_tile_coordinates, board.selected_index).render(&mut f, chunks[1]);
                    BottomLine::new(file_tree.failed_to_read).currently_selected(board.currently_selected()).render(&mut f, chunks[2]);
                    MessageBox::new(file_to_delete, ui_effects.deletion_in_progress).render(&mut f, full_screen);
                },
                UiMode::ErrorMessage(message) => {
                    TitleLine::new(base_path_info, current_path_info, file_tree.space_freed)
                        .path_error(ui_effects.current_path_is_red)
                        .flash_space(ui_effects.frame_around_space_freed)
                        .render(&mut f, chunks[0]);
                    RectangleGrid::new(&board.tiles, board.unrenderable_tile_coordinates, board.selected_index).render(&mut f, chunks[1]);
                    BottomLine::new(file_tree.failed_to_read).currently_selected(board.currently_selected()).render(&mut f, chunks[2]);
                    ErrorBox::new(message).render(&mut f, full_screen);
                }
            };
        }).expect("failed to draw");
    }
    pub fn clear (&mut self) {
        self.terminal.clear().expect("failed to clear terminal");
        self.terminal.show_cursor().expect("failed to show cursor");
    }
}
