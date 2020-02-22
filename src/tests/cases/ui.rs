use crate::tests::fakes::TerminalEvent::*;
use ::insta::assert_snapshot;

use crate::tests::cases::test_utils::{test_backend_factory, sleep_and_quit_events};
use std::path::PathBuf;

use crate::start;

use std::env;
use std::fs::{File, create_dir};
use std::io::prelude::*;
use uuid::Uuid;

fn create_temp_dir () -> Result<PathBuf, failure::Error> {
    let mut dir = env::temp_dir();
    let temp_dir_name = Uuid::new_v4();
    dir.push(temp_dir_name.to_string());

    create_dir(&dir)?;
    Ok(dir)
}

fn create_temp_file (path: PathBuf, size: usize) -> Result<(), failure::Error> {
    let mut file = File::create(path)?;
    let mut pos = 0;
    while pos < size {
        let bytes_written = file.write(b"W")?;
        pos += bytes_written;
    }
    Ok(())
}

#[test]
fn two_large_files_one_small_file () {

    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);
    let keyboard_events = sleep_and_quit_events(2);
    let temp_dir_path = create_temp_dir().expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 4000).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 5000).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 5000).expect("failed to create temp file");

    start(backend, keyboard_events, temp_dir_path.clone());
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();

    let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Clear, ShowCursor];

    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 2);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
}

#[test]
fn eleven_files () {

    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);
    let keyboard_events = sleep_and_quit_events(2);
    let temp_dir_path = create_temp_dir().expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 5000).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 5000).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 5000).expect("failed to create temp file");

    let mut file_4_path = PathBuf::from(&temp_dir_path);
    file_4_path.push("file4");
    create_temp_file(file_4_path, 5000).expect("failed to create temp file");

    let mut file_5_path = PathBuf::from(&temp_dir_path);
    file_5_path.push("file5");
    create_temp_file(file_5_path, 5000).expect("failed to create temp file");

    let mut file_6_path = PathBuf::from(&temp_dir_path);
    file_6_path.push("file6");
    create_temp_file(file_6_path, 50000).expect("failed to create temp file");

    let mut file_7_path = PathBuf::from(&temp_dir_path);
    file_7_path.push("file7");
    create_temp_file(file_7_path, 150000).expect("failed to create temp file");

    let mut file_8_path = PathBuf::from(&temp_dir_path);
    file_8_path.push("file8");
    create_temp_file(file_8_path, 50000).expect("failed to create temp file");

    let mut file_9_path = PathBuf::from(&temp_dir_path);
    file_9_path.push("file9");
    create_temp_file(file_9_path, 50000).expect("failed to create temp file");

    let mut file_10_path = PathBuf::from(&temp_dir_path);
    file_10_path.push("file10");
    create_temp_file(file_10_path, 50000).expect("failed to create temp file");

    let mut file_11_path = PathBuf::from(&temp_dir_path);
    file_11_path.push("file11");
    create_temp_file(file_11_path, 50000).expect("failed to create temp file");

    start(backend, keyboard_events, temp_dir_path.clone());
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();

    let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Clear, ShowCursor];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 2);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
}
