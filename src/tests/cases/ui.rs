use ::std::env;
use ::std::fs::{create_dir, create_dir_all, remove_dir_all, File};
use ::std::io::prelude::*;
use ::std::iter;
use ::std::path::{Path, PathBuf};

use ::insta::assert_snapshot;
use crossterm::event::KeyModifiers;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyEventState};

use crate::start;
use crate::tests::cases::test_utils::*;
use crate::tests::fakes::TerminalEvent::*;
use crate::tests::fakes::TerminalEvents;

macro_rules! key {
    (char $x:expr) => {
        Event::Key(KeyEvent {
            code: KeyCode::Char($x),
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        })
    };
    (ctrl $x:expr) => {
        Event::Key(KeyEvent {
            code: KeyCode::Char($x),
            modifiers: KeyModifiers::CONTROL,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        })
    };
    ($x:ident) => {
        Event::Key(KeyEvent {
            code: KeyCode::$x,
            modifiers: KeyModifiers::NONE,
            kind: KeyEventKind::Press,
            state: KeyEventState::empty(),
        })
    };
}
// this means we ask diskonaut to show the actual file size rather than the size taken on disk
//
// this is in order to make the tests more possible, so they will show the same result
// on filesystems with and without compression
const SHOW_APPARENT_SIZE: bool = true;

// This leaves delete confirmations enabled (The default behaviour).
const DELETE_CONFIRMATION_ENABLED: bool = false;
const DELETE_CONFIRMATION_DISABLED: bool = true;

fn create_root_temp_dir(name: &str) -> Result<PathBuf, failure::Error> {
    let mut dir = PathBuf::new();
    dir.push(String::from("/tmp/diskonaut_tests")); // TODO: fix this for other platforms
    dir.push(name.to_string());

    remove_dir_all(&dir).ok(); // atomic remove
    create_dir_all(&dir)?;
    Ok(dir)
}

fn create_temp_file<P: AsRef<Path>>(path: P, size: usize) -> Result<(), failure::Error> {
    let mut file = File::create(path)?;
    let mut pos = 0;
    while pos < size {
        let bytes_written = file.write(b"W")?;
        pos += bytes_written;
    }
    Ok(())
}

#[test]
fn two_large_files_one_small_file() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);
    let keyboard_events = sleep_and_quit_events(1, true);
    let temp_dir_path =
        create_root_temp_dir("two_large_files_one_small_file").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 8192).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 8192).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();
    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 2);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
}

#[test]
fn medium_width() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(60, 50);
    let keyboard_events = sleep_and_quit_events(1, true);
    let temp_dir_path = create_root_temp_dir("medium_width").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 8192).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 8192).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();
    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 2);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
}

#[test]
fn small_width() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(50, 50);
    let keyboard_events = sleep_and_quit_events(1, true);
    let temp_dir_path = create_root_temp_dir("small_width").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 8192).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 8192).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();
    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 2);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
}

#[test]
fn small_width_long_folder_name() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(50, 50);
    let keyboard_events = sleep_and_quit_events(1, true);
    let temp_dir_path =
        create_root_temp_dir("small_width_long_folder_name").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 8192).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 8192).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();
    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 2);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
}

#[test]
fn too_small_width_one() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(49, 50);
    let keyboard_events = sleep_and_quit_events(1, false);
    let temp_dir_path =
        create_root_temp_dir("too_small_width_one").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 8192).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 8192).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();
    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 1);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
}

#[test]
fn too_small_width_two() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(26, 50);
    let keyboard_events = sleep_and_quit_events(1, false);
    let temp_dir_path =
        create_root_temp_dir("too_small_width_two").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 8192).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 8192).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();
    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 1);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
}

#[test]
fn too_small_width_three() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(20, 50);
    let keyboard_events = sleep_and_quit_events(1, false);
    let temp_dir_path =
        create_root_temp_dir("too_small_width_three").expect("failed to create temp dir");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();
    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 1);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
}

#[test]
fn too_small_width_four() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(15, 50);
    let keyboard_events = sleep_and_quit_events(1, false);
    let temp_dir_path =
        create_root_temp_dir("too_small_width_four").expect("failed to create temp dir");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();
    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 1);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
}

#[test]
fn too_small_width_five() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(5, 50);
    let keyboard_events = sleep_and_quit_events(1, false);
    let temp_dir_path =
        create_root_temp_dir("too_small_width_five").expect("failed to create temp dir");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();
    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 1);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
}

#[test]
fn too_small_height() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 14);
    let keyboard_events = sleep_and_quit_events(1, false);
    let temp_dir_path =
        create_root_temp_dir("too_small_height").expect("failed to create temp dir");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();
    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 1);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
}

#[test]
fn eleven_files() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);
    let keyboard_events = sleep_and_quit_events(1, true);
    let temp_dir_path = create_root_temp_dir("eleven_files").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 8192).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 8192).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 8192).expect("failed to create temp file");

    let mut file_4_path = PathBuf::from(&temp_dir_path);
    file_4_path.push("file4");
    create_temp_file(file_4_path, 8192).expect("failed to create temp file");

    let mut file_5_path = PathBuf::from(&temp_dir_path);
    file_5_path.push("file5");
    create_temp_file(file_5_path, 8192).expect("failed to create temp file");

    let mut file_6_path = PathBuf::from(&temp_dir_path);
    file_6_path.push("file6");
    create_temp_file(file_6_path, 53248).expect("failed to create temp file");

    let mut file_7_path = PathBuf::from(&temp_dir_path);
    file_7_path.push("file7");
    create_temp_file(file_7_path, 151552).expect("failed to create temp file");

    let mut file_8_path = PathBuf::from(&temp_dir_path);
    file_8_path.push("file8");
    create_temp_file(file_8_path, 53248).expect("failed to create temp file");

    let mut file_9_path = PathBuf::from(&temp_dir_path);
    file_9_path.push("file9");
    create_temp_file(file_9_path, 53248).expect("failed to create temp file");

    let mut file_10_path = PathBuf::from(&temp_dir_path);
    file_10_path.push("file10");
    create_temp_file(file_10_path, 53248).expect("failed to create temp file");

    let mut file_11_path = PathBuf::from(&temp_dir_path);
    file_11_path.push("file11");
    create_temp_file(file_11_path, 53248).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 2);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
}

#[test]
fn enter_folder() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'j'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char '\n')));
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path = create_root_temp_dir("enter_folder").expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(file_1_path, 8192).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 4);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
}

#[test]
fn enter_folder_medium_width() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(90, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'j'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char '\n')));
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path =
        create_root_temp_dir("enter_folder_medium_width").expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(file_1_path, 8192).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];

    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 4);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
}

#[test]
fn enter_folder_small_width() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(60, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'j'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char '\n')));
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path =
        create_root_temp_dir("enter_folder_small_width").expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder_with_quite_a_long_name");
    create_dir(subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder_with_quite_a_long_name");
    file_1_path.push("file1");
    create_temp_file(file_1_path, 8192).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 4);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
}

#[test]
fn small_files() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);
    let keyboard_events = sleep_and_quit_events(1, true);
    let temp_dir_path = create_root_temp_dir("small_files").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 401408).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 1000000).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 1000000).expect("failed to create temp file");

    let mut file_4_path = PathBuf::from(&temp_dir_path);
    file_4_path.push("file4");
    create_temp_file(file_4_path, 8192).expect("failed to create temp file");

    let mut file_5_path = PathBuf::from(&temp_dir_path);
    file_5_path.push("file5");
    create_temp_file(file_5_path, 8192).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 2);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
}

#[test]
fn zoom_into_small_files() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);
    let mut events: Vec<Option<Event>> = iter::repeat(None).take(2).collect();
    events.push(Some(key!(char '+')));
    events.push(None);
    events.push(Some(key!(char '+')));
    events.push(None);
    events.push(Some(key!(char '+')));
    events.push(None);
    events.push(Some(key!(char '+')));
    events.push(None);
    events.push(Some(key!(char '-')));
    events.push(None);
    events.push(Some(key!(char '-')));
    events.push(None);
    events.push(Some(key!(char '0')));
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));
    let temp_dir_path =
        create_root_temp_dir("zoom_into_small_files").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 401408).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 1000000).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 1000000).expect("failed to create temp file");

    let mut file_4_path = PathBuf::from(&temp_dir_path);
    file_4_path.push("file4");
    create_temp_file(file_4_path, 8192).expect("failed to create temp file");

    let mut file_5_path = PathBuf::from(&temp_dir_path);
    file_5_path.push("file5");
    create_temp_file(file_5_path, 8192).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw,
        HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 9);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
    assert_snapshot!(&terminal_draw_events_mirror[5]);
    assert_snapshot!(&terminal_draw_events_mirror[6]);
    assert_snapshot!(&terminal_draw_events_mirror[7]);
}

#[test]
fn cannot_move_into_small_files() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(2).collect();
    events.push(Some(key!(char 'j'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char 'l')));
    events.push(None);
    events.push(Some(key!(char 'j')));
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path =
        create_root_temp_dir("cannot_move_into_small_files").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 401408).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 1000000).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 1000000).expect("failed to create temp file");

    let mut file_4_path = PathBuf::from(&temp_dir_path);
    file_4_path.push("file4");
    create_temp_file(file_4_path, 4096).expect("failed to create temp file");

    let mut file_5_path = PathBuf::from(&temp_dir_path);
    file_5_path.push("file5");
    create_temp_file(file_5_path, 4096).expect("failed to create temp file");

    let mut file_6_path = PathBuf::from(&temp_dir_path);
    file_6_path.push("file6");
    create_temp_file(file_6_path, 4096).expect("failed to create temp file");

    let mut file_7_path = PathBuf::from(&temp_dir_path);
    file_7_path.push("file7");
    create_temp_file(file_7_path, 4096).expect("failed to create temp file");

    let mut file_8_path = PathBuf::from(&temp_dir_path);
    file_8_path.push("file8");
    create_temp_file(file_8_path, 4096).expect("failed to create temp file");

    let mut file_9_path = PathBuf::from(&temp_dir_path);
    file_9_path.push("file9");
    create_temp_file(file_9_path, 4096).expect("failed to create temp file");

    let mut file_10_path = PathBuf::from(&temp_dir_path);
    file_10_path.push("file10");
    create_temp_file(file_10_path, 4096).expect("failed to create temp file");

    let mut file_11_path = PathBuf::from(&temp_dir_path);
    file_11_path.push("file11");
    create_temp_file(file_11_path, 4096).expect("failed to create temp file");

    let mut file_12_path = PathBuf::from(&temp_dir_path);
    file_12_path.push("file12");
    create_temp_file(file_12_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 5);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
}

#[test]
fn minimum_tile_sides() {
    // here we test that tiles are not created with a side_length (height in this case)
    // that is too small to render while not being designated as a "small file"
    //
    // the only case in which this can happen if this is the last tile to be placed
    // this case might in the future be solved by artificially increasing its size
    // to the minimum with some sort of asterisk to explain

    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);
    let keyboard_events = sleep_and_quit_events(1, true);
    let temp_dir_path =
        create_root_temp_dir("minimum_tile_sides").expect("failed to create temp dir");

    for i in 0..7 {
        let mut file_path = PathBuf::from(&temp_dir_path);
        file_path.push(format!("big_file{}", i));
        create_temp_file(file_path, 135168).expect("failed to create temp file");
    }

    for i in 0..2 {
        let mut file_path = PathBuf::from(&temp_dir_path);
        file_path.push(format!("medium_file{}", i));
        create_temp_file(file_path, 8192).expect("failed to create temp file");
    }

    for i in 0..50 {
        let mut file_path = PathBuf::from(&temp_dir_path);
        file_path.push(format!("file{}", i));
        create_temp_file(file_path, 4096).expect("failed to create temp file");
    }

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];

    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 2);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
}

#[test]
fn move_down_and_enter_folder() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(2).collect();
    events.push(Some(key!(char 'j'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char 'j')));
    events.push(None);
    events.push(Some(key!(char '\n')));
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path =
        create_root_temp_dir("move_down_and_enter_folder").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 8192).expect("failed to create temp file");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(subfolder_1_path).expect("failed to create temporary directory");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("subfolder1");
    file_3_path.push("file3");
    create_temp_file(file_3_path, 8192).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 5);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
}

#[test]
fn noop_when_entering_file() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'j'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char 'j')));
    events.push(None);
    events.push(Some(key!(char '\n')));
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path =
        create_root_temp_dir("noop_when_entering_file").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 8192).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 8192).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];

    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 4);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
}

#[test]
fn move_up_and_enter_folder() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'j'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char 'j')));
    events.push(None);
    events.push(Some(key!(char 'k')));
    events.push(None);
    events.push(Some(key!(char '\n')));
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path =
        create_root_temp_dir("move_up_and_enter_folder").expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(file_1_path, 12288).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 8192).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear,
        ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 6);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
    assert_snapshot!(&terminal_draw_events_mirror[5]);
}

#[test]
fn move_right_and_enter_folder() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char 'l')));
    events.push(None);
    events.push(Some(key!(char '\n')));
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path =
        create_root_temp_dir("move_right_and_enter_folder").expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 5);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
}

#[test]
fn move_left_and_enter_folder() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char 'l')));
    events.push(None);
    events.push(Some(key!(char 'h')));
    events.push(None);
    events.push(Some(key!(char '\n')));
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path =
        create_root_temp_dir("move_left_and_enter_folder").expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(file_1_path, 8192).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear,
        ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 6);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
    assert_snapshot!(&terminal_draw_events_mirror[5]);
}

#[test]
fn enter_largest_folder_with_no_selected_tile() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char '\n')));
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path = create_root_temp_dir("enter_largest_folder_with_no_selected_tile")
        .expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(file_1_path, 8192).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Clear, ShowCursor,
    ];

    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 3);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
}

#[test]
fn clear_selection_when_moving_off_screen_edges() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char 'l')));
    events.push(None);
    events.push(Some(key!(char 'l')));
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path = create_root_temp_dir("noop_when_moving_off_screen_edges")
        .expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 5);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
}

#[test]
fn esc_to_go_up() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char 'l')));
    events.push(None);
    events.push(Some(key!(char '\n')));
    events.push(None);
    events.push(Some(key!(Esc)));
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path = create_root_temp_dir("esc_to_go_up").expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear,
        ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 6);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
    assert_snapshot!(&terminal_draw_events_mirror[5]);
}

#[test]
fn noop_when_pressing_esc_at_base_folder() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char 'l')));
    events.push(None);
    events.push(Some(key!(char '\n')));
    events.push(None);
    events.push(Some(key!(Esc)));
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(Esc)));
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path = create_root_temp_dir("noop_when_pressing_esc_at_base_folder")
        .expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw,
        HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 9);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
    assert_snapshot!(&terminal_draw_events_mirror[5]);
    assert_snapshot!(&terminal_draw_events_mirror[6]);
    assert_snapshot!(&terminal_draw_events_mirror[7]);
    assert_snapshot!(&terminal_draw_events_mirror[8]);
}

#[test]
fn delete_file() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(Backspace)));
    events.push(None);
    events.push(Some(key!(char 'y')));
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path = create_root_temp_dir("delete_file").expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(&subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(&file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(&file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(&file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw,
        HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );
    assert_eq!(
        std::fs::metadata(&file_2_path).is_err(),
        true,
        "file successfully deleted"
    );
    assert_eq!(
        std::fs::metadata(&subfolder_1_path).is_ok(),
        true,
        "different folder stayed the same"
    );
    assert_eq!(
        std::fs::metadata(&file_1_path).is_ok(),
        true,
        "different file was untoucehd"
    );
    assert_eq!(
        std::fs::metadata(&file_3_path).is_ok(),
        true,
        "second different file was untouched"
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

    assert_eq!(terminal_draw_events_mirror.len(), 8);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
    assert_snapshot!(&terminal_draw_events_mirror[5]);
    assert_snapshot!(&terminal_draw_events_mirror[6]);
}

#[test]
fn delete_file_no_confirmation() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(Backspace)));
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path =
        create_root_temp_dir("delete_file_no_confirmation").expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(&subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(&file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(&file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(&file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_DISABLED,
    );
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw,
        HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );
    assert_eq!(
        std::fs::metadata(&file_2_path).is_err(),
        true,
        "file successfully deleted"
    );
    assert_eq!(
        std::fs::metadata(&subfolder_1_path).is_ok(),
        true,
        "different folder stayed the same"
    );
    assert_eq!(
        std::fs::metadata(&file_1_path).is_ok(),
        true,
        "different file was untoucehd"
    );
    assert_eq!(
        std::fs::metadata(&file_3_path).is_ok(),
        true,
        "second different file was untouched"
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

    assert_eq!(terminal_draw_events_mirror.len(), 7);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
    assert_snapshot!(&terminal_draw_events_mirror[5]);
    assert_snapshot!(&terminal_draw_events_mirror[6]);
}

#[test]
fn cant_delete_file_with_term_too_small() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(49, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(Backspace)));
    events.push(None);
    events.push(Some(key!(char 'y')));
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path = create_root_temp_dir("cant_delete_file_with_term_too_small")
        .expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(&subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(&file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(&file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(&file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );
    assert_eq!(
        std::fs::metadata(&file_2_path).is_ok(),
        true,
        "file not deleted"
    );
    assert_eq!(
        std::fs::metadata(&subfolder_1_path).is_ok(),
        true,
        "different folder stayed the same"
    );
    assert_eq!(
        std::fs::metadata(&file_1_path).is_ok(),
        true,
        "different file was untoucehd"
    );
    assert_eq!(
        std::fs::metadata(&file_3_path).is_ok(),
        true,
        "second different file was untouched"
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

    assert_eq!(terminal_draw_events_mirror.len(), 1);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
}

#[test]
fn delete_folder() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char 'l')));
    events.push(None);
    events.push(Some(key!(Backspace)));
    events.push(None);
    events.push(Some(key!(char 'y')));
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path = create_root_temp_dir("delete_folder").expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(&subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(&file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(&file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(&file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw,
        HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );
    assert_eq!(
        std::fs::metadata(&subfolder_1_path).is_err(),
        true,
        "folder successfully deleted"
    );
    assert_eq!(
        std::fs::metadata(&file_1_path).is_err(),
        true,
        "internal file successfully deleted"
    ); // can't really fail on its own, but left here for clarity
    assert_eq!(
        std::fs::metadata(&file_2_path).is_ok(),
        true,
        "different file was untouched"
    );
    assert_eq!(
        std::fs::metadata(&file_3_path).is_ok(),
        true,
        "second different file was untouched"
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

    assert_eq!(terminal_draw_events_mirror.len(), 9);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
    assert_snapshot!(&terminal_draw_events_mirror[5]);
    assert_snapshot!(&terminal_draw_events_mirror[6]);
    assert_snapshot!(&terminal_draw_events_mirror[7]);
    assert_snapshot!(&terminal_draw_events_mirror[8]);
}

#[test]
fn delete_folder_no_confirmation() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char 'l')));
    events.push(None);
    events.push(Some(key!(Backspace)));
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path =
        create_root_temp_dir("delete_folder_no_confirmation").expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(&subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(&file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(&file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(&file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_DISABLED,
    );
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw,
        HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );
    assert_eq!(
        std::fs::metadata(&subfolder_1_path).is_err(),
        true,
        "folder successfully deleted"
    );
    assert_eq!(
        std::fs::metadata(&file_1_path).is_err(),
        true,
        "internal file successfully deleted"
    ); // can't really fail on its own, but left here for clarity
    assert_eq!(
        std::fs::metadata(&file_2_path).is_ok(),
        true,
        "different file was untouched"
    );
    assert_eq!(
        std::fs::metadata(&file_3_path).is_ok(),
        true,
        "second different file was untouched"
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

    assert_eq!(terminal_draw_events_mirror.len(), 8);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
    assert_snapshot!(&terminal_draw_events_mirror[5]);
    assert_snapshot!(&terminal_draw_events_mirror[6]);
    assert_snapshot!(&terminal_draw_events_mirror[7]);
}

#[test]
fn delete_folder_small_window() {
    // terminal window with a width of 60 (shorter message window layout)
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(60, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'j'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char 'j')));
    events.push(None);
    events.push(Some(key!(char 'j')));
    events.push(None);
    events.push(Some(key!(Backspace)));
    events.push(None);
    events.push(Some(key!(char 'y')));
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path =
        create_root_temp_dir("delete_folder_small_window").expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(&subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(&file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(&file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(&file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw,
        HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );
    assert_eq!(
        std::fs::metadata(&file_2_path).is_ok(),
        true,
        "different file was untouched"
    );
    assert_eq!(
        std::fs::metadata(&subfolder_1_path).is_err(),
        true,
        "file successfully deleted"
    );
    assert_eq!(
        std::fs::metadata(&file_1_path).is_err(),
        true,
        "file in folder deleted"
    );
    assert_eq!(
        std::fs::metadata(&file_3_path).is_ok(),
        true,
        "second different file was untouched"
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

    assert_eq!(terminal_draw_events_mirror.len(), 10);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
    assert_snapshot!(&terminal_draw_events_mirror[5]);
    assert_snapshot!(&terminal_draw_events_mirror[6]);
    assert_snapshot!(&terminal_draw_events_mirror[7]);
    assert_snapshot!(&terminal_draw_events_mirror[8]);
    assert_snapshot!(&terminal_draw_events_mirror[9]);
}

#[test]
fn delete_folder_small_window_no_confirmation() {
    // terminal window with a width of 60 (shorter message window layout)
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(60, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'j'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char 'j')));
    events.push(None);
    events.push(Some(key!(char 'j')));
    events.push(None);
    events.push(Some(key!(Backspace)));
    events.push(None);
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path = create_root_temp_dir("delete_folder_small_window_no_confirmation")
        .expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(&subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(&file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(&file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(&file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_DISABLED,
    );
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw,
        HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );
    assert_eq!(
        std::fs::metadata(&file_2_path).is_ok(),
        true,
        "different file was untouched"
    );
    assert_eq!(
        std::fs::metadata(&subfolder_1_path).is_err(),
        true,
        "file successfully deleted"
    );
    assert_eq!(
        std::fs::metadata(&file_1_path).is_err(),
        true,
        "file in folder deleted"
    );
    assert_eq!(
        std::fs::metadata(&file_3_path).is_ok(),
        true,
        "second different file was untouched"
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

    assert_eq!(terminal_draw_events_mirror.len(), 9);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
    assert_snapshot!(&terminal_draw_events_mirror[5]);
    assert_snapshot!(&terminal_draw_events_mirror[6]);
    assert_snapshot!(&terminal_draw_events_mirror[7]);
    assert_snapshot!(&terminal_draw_events_mirror[8]);
}

#[test]
fn delete_folder_with_multiple_children() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char 'l')));
    events.push(None);
    events.push(Some(key!(Backspace)));
    events.push(None);
    events.push(Some(key!(char 'y')));
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path = create_root_temp_dir("delete_folder_with_multiple_children")
        .expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(&file_1_path, 16384).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(&file_2_path, 16384).expect("failed to create temp file");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(&subfolder_1_path).expect("failed to create temporary directory");

    let mut subfolder_2_path = PathBuf::from(&temp_dir_path);
    subfolder_2_path.push("subfolder1");
    subfolder_2_path.push("subfolder2");
    create_dir(&subfolder_2_path).expect("failed to create temporary directory");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("subfolder1");
    file_3_path.push("subfolder2");
    file_3_path.push("file3");
    create_temp_file(&file_3_path, 4096).expect("failed to create temp file");

    let mut file_4_path = PathBuf::from(&temp_dir_path);
    file_4_path.push("subfolder1");
    file_4_path.push("subfolder2");
    file_4_path.push("file4");
    create_temp_file(&file_4_path, 4096).expect("failed to create temp file");

    let mut file_5_path = PathBuf::from(&temp_dir_path);
    file_5_path.push("subfolder1");
    file_5_path.push("file5");
    create_temp_file(&file_5_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw,
        HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];

    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );
    assert_eq!(
        std::fs::metadata(&subfolder_1_path).is_err(),
        true,
        "folder successfully deleted"
    );
    assert_eq!(
        std::fs::metadata(&subfolder_2_path).is_err(),
        true,
        "folder inside deleted folder successfully deleted"
    );
    assert_eq!(
        std::fs::metadata(&file_1_path).is_ok(),
        true,
        "different file was untouched"
    );
    assert_eq!(
        std::fs::metadata(&file_2_path).is_ok(),
        true,
        "different file was untouched"
    );
    assert_eq!(
        std::fs::metadata(&file_3_path).is_err(),
        true,
        "internal file in folder deleted"
    );
    assert_eq!(
        std::fs::metadata(&file_4_path).is_err(),
        true,
        "internal file in folder deleted"
    );
    assert_eq!(
        std::fs::metadata(&file_5_path).is_err(),
        true,
        "internal file in folder deleted"
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

    assert_eq!(terminal_draw_events_mirror.len(), 9);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
    assert_snapshot!(&terminal_draw_events_mirror[5]);
    assert_snapshot!(&terminal_draw_events_mirror[6]);
    assert_snapshot!(&terminal_draw_events_mirror[7]);
    assert_snapshot!(&terminal_draw_events_mirror[8]);
}

#[test]
fn delete_folder_with_multiple_children_no_confirmation() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char 'l')));
    events.push(None);
    events.push(Some(key!(Backspace)));
    events.push(None);
    // here we sleep extra to allow the blink events to happen and be tested before the app exits
    // with the following ctrl-c
    events.push(None);
    events.push(None);
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path =
        create_root_temp_dir("delete_folder_with_multiple_children_no_confirmation")
            .expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(&file_1_path, 16384).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(&file_2_path, 16384).expect("failed to create temp file");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(&subfolder_1_path).expect("failed to create temporary directory");

    let mut subfolder_2_path = PathBuf::from(&temp_dir_path);
    subfolder_2_path.push("subfolder1");
    subfolder_2_path.push("subfolder2");
    create_dir(&subfolder_2_path).expect("failed to create temporary directory");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("subfolder1");
    file_3_path.push("subfolder2");
    file_3_path.push("file3");
    create_temp_file(&file_3_path, 4096).expect("failed to create temp file");

    let mut file_4_path = PathBuf::from(&temp_dir_path);
    file_4_path.push("subfolder1");
    file_4_path.push("subfolder2");
    file_4_path.push("file4");
    create_temp_file(&file_4_path, 4096).expect("failed to create temp file");

    let mut file_5_path = PathBuf::from(&temp_dir_path);
    file_5_path.push("subfolder1");
    file_5_path.push("file5");
    create_temp_file(&file_5_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_DISABLED,
    );
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw,
        HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );
    assert_eq!(
        std::fs::metadata(&subfolder_1_path).is_err(),
        true,
        "folder successfully deleted"
    );
    assert_eq!(
        std::fs::metadata(&subfolder_2_path).is_err(),
        true,
        "folder inside deleted folder successfully deleted"
    );
    assert_eq!(
        std::fs::metadata(&file_1_path).is_ok(),
        true,
        "different file was untouched"
    );
    assert_eq!(
        std::fs::metadata(&file_2_path).is_ok(),
        true,
        "different file was untouched"
    );
    assert_eq!(
        std::fs::metadata(&file_3_path).is_err(),
        true,
        "internal file in folder deleted"
    );
    assert_eq!(
        std::fs::metadata(&file_4_path).is_err(),
        true,
        "internal file in folder deleted"
    );
    assert_eq!(
        std::fs::metadata(&file_5_path).is_err(),
        true,
        "internal file in folder deleted"
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

    assert_eq!(terminal_draw_events_mirror.len(), 8);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
    assert_snapshot!(&terminal_draw_events_mirror[5]);
    assert_snapshot!(&terminal_draw_events_mirror[6]);
    assert_snapshot!(&terminal_draw_events_mirror[7]);
}

#[test]
fn pressing_delete_with_no_selected_tile() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(Backspace)));
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path = create_root_temp_dir("pressing_delete_with_no_selected_tile")
        .expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(&subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(&file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(&file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(&file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );
    assert_eq!(
        std::fs::metadata(&file_2_path).is_ok(),
        true,
        "file not deleted"
    );
    assert_eq!(
        std::fs::metadata(&subfolder_1_path).is_ok(),
        true,
        "different folder stayed the same"
    );
    assert_eq!(
        std::fs::metadata(&file_1_path).is_ok(),
        true,
        "different file was untoucehd"
    );
    assert_eq!(
        std::fs::metadata(&file_3_path).is_ok(),
        true,
        "second different file was untouched"
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

    assert_eq!(terminal_draw_events_mirror.len(), 2);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
}

#[test]
fn delete_file_press_n() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(Backspace)));
    events.push(None);
    events.push(Some(key!(char 'n')));
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path =
        create_root_temp_dir("delete_file_press_n").expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(&subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(&file_1_path, 4096).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(&file_2_path, 4096).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(&file_3_path, 4096).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );
    assert_eq!(
        std::fs::metadata(&file_2_path).is_ok(),
        true,
        "file not deleted"
    );
    assert_eq!(
        std::fs::metadata(&subfolder_1_path).is_ok(),
        true,
        "different folder stayed the same"
    );
    assert_eq!(
        std::fs::metadata(&file_1_path).is_ok(),
        true,
        "different file was untoucehd"
    );
    assert_eq!(
        std::fs::metadata(&file_3_path).is_ok(),
        true,
        "second different file was untouched"
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

    assert_eq!(terminal_draw_events_mirror.len(), 5);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
}

#[test]
fn files_with_size_zero() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);
    let keyboard_events = sleep_and_quit_events(1, true);
    let temp_dir_path =
        create_root_temp_dir("files_with_size_zero").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 0).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 0).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 0).expect("failed to create temp file");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();
    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 2);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
}

#[test]
fn empty_folder() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);
    let keyboard_events = sleep_and_quit_events(1, true);
    let temp_dir_path = create_root_temp_dir("empty_folder").expect("failed to create temp dir");

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();
    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 2);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
}
#[cfg(not(target_os = "windows"))]
#[test]
fn permission_denied_when_deleting() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char '\n')));
    events.push(None);
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(Backspace)));
    events.push(None);
    events.push(Some(key!(char 'y')));
    events.push(None);
    events.push(Some(key!(Esc)));
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path =
        create_root_temp_dir("permission_denied_when_deleting").expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(&subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(&file_1_path, 4096).expect("failed to create temp file");

    let mut perms = std::fs::metadata(&subfolder_1_path).unwrap().permissions();
    perms.set_readonly(true);
    std::fs::set_permissions(&subfolder_1_path, perms.clone()).unwrap();

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    assert_eq!(
        std::fs::metadata(&file_1_path).is_ok(),
        true,
        "file was not deleted"
    ); // can't really fail on its own, but left here for clarity
    assert_eq!(
        std::fs::metadata(&subfolder_1_path).is_ok(),
        true,
        "containing folder was not deleted"
    );

    perms.set_readonly(false);
    std::fs::set_permissions(&subfolder_1_path, perms).unwrap();
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw,
        HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];

    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 9);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
    assert_snapshot!(&terminal_draw_events_mirror[5]);
    assert_snapshot!(&terminal_draw_events_mirror[6]);
    assert_snapshot!(&terminal_draw_events_mirror[7]);
    assert_snapshot!(&terminal_draw_events_mirror[8]);
}
#[cfg(not(target_os = "windows"))]
#[test]
fn permission_denied_when_deleting_no_confirmation() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(char '\n')));
    events.push(None);
    events.push(Some(key!(char 'l'))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(key!(Backspace)));
    events.push(None);
    events.push(Some(key!(Esc)));
    events.push(None);
    events.push(Some(key!(ctrl 'c')));
    events.push(None);
    events.push(Some(key!(char 'y')));
    let keyboard_events = Box::new(TerminalEvents::new(events));

    let temp_dir_path = create_root_temp_dir("permission_denied_when_deleting_no_confirmation")
        .expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(&subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(&file_1_path, 4096).expect("failed to create temp file");

    let mut perms = std::fs::metadata(&subfolder_1_path).unwrap().permissions();
    perms.set_readonly(true);
    std::fs::set_permissions(&subfolder_1_path, perms.clone()).unwrap();

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_DISABLED,
    );
    let terminal_draw_events_mirror = terminal_draw_events
        .lock()
        .expect("could not acquire lock on terminal events");

    assert_eq!(
        std::fs::metadata(&file_1_path).is_ok(),
        true,
        "file was not deleted"
    ); // can't really fail on its own, but left here for clarity
    assert_eq!(
        std::fs::metadata(&subfolder_1_path).is_ok(),
        true,
        "containing folder was not deleted"
    );

    perms.set_readonly(false);
    std::fs::set_permissions(&subfolder_1_path, perms).unwrap();
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor,
        Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Draw,
        HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];

    assert_eq!(
        &terminal_events
            .lock()
            .expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 8);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
    assert_snapshot!(&terminal_draw_events_mirror[4]);
    assert_snapshot!(&terminal_draw_events_mirror[5]);
    assert_snapshot!(&terminal_draw_events_mirror[6]);
    assert_snapshot!(&terminal_draw_events_mirror[7]);
}

#[test]
fn small_files_with_y_as_zero() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);
    let keyboard_events = sleep_and_quit_events(1, true);
    let temp_dir_path =
        create_root_temp_dir("small_files_with_y_as_zero").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 1048576).expect("failed to create temp file");

    for i in 1..100 {
        let mut small_file_path = PathBuf::from(&temp_dir_path);
        small_file_path.push(format!("small_file{}", i));
        create_temp_file(small_file_path, 4096).expect("failed to create temp file");
    }

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();
    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 2);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
}

#[test]
fn small_files_with_x_as_zero() {
    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(50, 50);
    let keyboard_events = sleep_and_quit_events(1, true);
    let temp_dir_path =
        create_root_temp_dir("small_files_with_x_as_zero").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 1048576).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 1048576).expect("failed to create temp file");

    for i in 1..100 {
        let mut small_file_path = PathBuf::from(&temp_dir_path);
        small_file_path.push(format!("small_file{}", i));
        create_temp_file(small_file_path, 4096).expect("failed to create temp file");
    }

    start(
        backend,
        keyboard_events,
        temp_dir_path.clone(),
        SHOW_APPARENT_SIZE,
        DELETE_CONFIRMATION_ENABLED,
    );
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();
    let expected_terminal_events = vec![
        Clear, HideCursor, Draw, HideCursor, Flush, Draw, HideCursor, Flush, Clear, ShowCursor,
    ];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 2);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
}
