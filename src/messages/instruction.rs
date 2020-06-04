use ::std::sync::mpsc::Receiver;
use ::std::fs::Metadata;
use ::tui::backend::Backend;
use ::termion::event::{Event as TermionEvent};
use ::walkdir::DirEntry;

use crate::{App, UiMode};

use crate::input::{
    handle_keypress_loading_mode,
    handle_keypress_normal_mode,
    handle_keypress_screen_too_small,
    handle_keypress_delete_file_mode,
    handle_keypress_error_message,
};

pub enum Instruction {
    SetFrameAroundCurrentPath,
    RemoveFrameAroundCurrentPath,
    SetPathToRed,
    ResetCurrentPathColor,
    SetFrameAroundSpaceFreed,
    RemoveFrameAroundSpaceFreed,
    AddEntryToBaseFolder((Metadata, DirEntry, usize)),
    StartUi,
    ToggleScanningVisualIndicator,
    RenderAndUpdateBoard,
    Render,
    ResetUiMode,
    Keypress(TermionEvent),
    IncrementFailedToRead,
}

pub fn handle_instructions<B> (app: &mut App<B>, receiver: Receiver<Instruction>)
where B: Backend
{
    loop {
        let instruction = receiver.recv().expect("failed to receive instruction on channel");
        match instruction {
            Instruction::SetFrameAroundCurrentPath => {
                app.set_frame_around_current_path();
            }
            Instruction::RemoveFrameAroundCurrentPath => {
                app.remove_frame_around_current_path();
            }
            Instruction::SetPathToRed => {
                app.set_path_to_red();
            }
            Instruction::ResetCurrentPathColor => {
                app.reset_current_path_color();
            }
            Instruction::SetFrameAroundSpaceFreed => {
                app.set_frame_around_space_freed();
            }
            Instruction::RemoveFrameAroundSpaceFreed => {
                app.remove_frame_around_space_freed();
            }
            Instruction::AddEntryToBaseFolder((file_metadata, entry, path_length)) => {
                // TODO: consider placing path_in_filesystem on app and using it instead of
                // receiving path_length in the instruction
                let entry_path = entry.path();
                app.add_entry_to_base_folder(&file_metadata, &entry_path, &path_length);
            }
            Instruction::StartUi => {
                app.start_ui();
            }
            Instruction::ToggleScanningVisualIndicator => {
                app.increment_loading_progress_indicator();
            }
            Instruction::RenderAndUpdateBoard => {
                app.render_and_update_board();
            }
            Instruction::Render => {
                app.render();
            }
            Instruction::ResetUiMode => {
                app.reset_ui_mode();
            }
            Instruction::Keypress(evt) => {
                match &app.ui_mode {
                    UiMode::Loading => {
                        handle_keypress_loading_mode(evt, app);
                    },
                    UiMode::Normal => {
                        handle_keypress_normal_mode(evt, app);
                    },
                    UiMode::ScreenTooSmall => {
                        handle_keypress_screen_too_small(evt, app);

                    },
                    UiMode::DeleteFile(file_to_delete) => {
                        let file_to_delete = file_to_delete.clone();
                        handle_keypress_delete_file_mode(evt, app, file_to_delete);
                    },
                    UiMode::ErrorMessage(_) => {
                        handle_keypress_error_message(evt, app);
                    }
                }
                if !app.is_running {
                    break;
                }
            }
            Instruction::IncrementFailedToRead => {
                app.increment_failed_to_read();
            }
        }
    }
}
