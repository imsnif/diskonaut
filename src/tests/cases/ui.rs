use crate::tests::fakes::TerminalEvent::*;
use ::insta::assert_snapshot;

use crate::tests::cases::test_utils::{test_backend_factory, sleep_and_quit_events};
use std::path::{PathBuf, Path};

use crate::start;

use std::env;
use std::fs::{File, create_dir, create_dir_all, remove_dir_all};
use std::io::prelude::*;

use std::iter;
use ::termion::event::{Event, Key};
use crate::tests::fakes::{KeyboardEvents};

fn create_root_temp_dir (name: &str) -> Result<PathBuf, failure::Error> {
    let mut dir = env::temp_dir();
    dir.push(String::from("bandwhich_tests"));
    dir.push(name.to_string());

    remove_dir_all(&dir).ok(); // atomic remove
    create_dir_all(&dir)?;
    Ok(dir)
}

fn create_temp_file <P: AsRef<Path>>(path: P, size: usize) -> Result<(), failure::Error> {
    let mut file = File::create(path)?;
    let mut pos = 0;
    while pos < size {
        let bytes_written = file.write(b"W")?;
        pos += bytes_written;
    }
    Ok(())
}

// TODO: also test with immediately pressing j after start without sleep to make sure we do not
// panic

// TODO: adjust tests for other platforms (currently the snapshots include the /tmp folder which
// might not work when running on mac/windows)

#[test]
fn two_large_files_one_small_file () {

    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);
    let keyboard_events = sleep_and_quit_events(1);
    let temp_dir_path = create_root_temp_dir("two_large_files_one_small_file").expect("failed to create temp dir");

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
    println!("terminal_draw_events_mirror[0] {:?}", terminal_draw_events_mirror[0]);

    let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Clear, ShowCursor];

    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 1);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
}

#[test]
fn medium_width () {

    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(100, 50);
    let keyboard_events = sleep_and_quit_events(1);
    let temp_dir_path = create_root_temp_dir("medium_width").expect("failed to create temp dir");

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
    println!("terminal_draw_events_mirror[0] {:?}", terminal_draw_events_mirror[0]);

    let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Clear, ShowCursor];

    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 1);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
}

#[test]
fn small_width () {

    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(50, 50);
    let keyboard_events = sleep_and_quit_events(1);
    let temp_dir_path = create_root_temp_dir("small_width").expect("failed to create temp dir");

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
    println!("terminal_draw_events_mirror[0] {:?}", terminal_draw_events_mirror[0]);

    let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Clear, ShowCursor];

    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 1);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
}

#[test]
fn eleven_files () {

    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);
    let keyboard_events = sleep_and_quit_events(1);
    let temp_dir_path = create_root_temp_dir("eleven_files").expect("failed to create temp dir");

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

    // let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Clear, ShowCursor];
    let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Clear, ShowCursor];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 1);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
}

#[test]
fn enter_folder() {

    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
    events.push(Some(Event::Key(Key::Char('j')))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(Event::Key(Key::Char('\n'))));
    events.push(None);
    events.push(Some(Event::Key(Key::Ctrl('c'))));
    let keyboard_events = Box::new(KeyboardEvents::new(events));

    let temp_dir_path = create_root_temp_dir("enter_folder").expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(file_1_path, 8000).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 4000).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 4000).expect("failed to create temp file");

    start(backend, keyboard_events, temp_dir_path.clone());
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Draw, Flush, Clear, ShowCursor];
    assert_eq!(
        &terminal_events.lock().expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 3);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
}

#[test]
fn small_files () {

    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);
    let keyboard_events = sleep_and_quit_events(1);
    let temp_dir_path = create_root_temp_dir("small_files").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 400000).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 1000000).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 1000000).expect("failed to create temp file");

    let mut file_4_path = PathBuf::from(&temp_dir_path);
    file_4_path.push("file4");
    create_temp_file(file_4_path, 8000).expect("failed to create temp file");

    let mut file_5_path = PathBuf::from(&temp_dir_path);
    file_5_path.push("file5");
    create_temp_file(file_5_path, 8000).expect("failed to create temp file");

    start(backend, keyboard_events, temp_dir_path.clone());
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();

    let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Clear, ShowCursor];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 1);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
}

#[test]
fn cannot_move_into_small_files () {

    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(2).collect();
    events.push(Some(Event::Key(Key::Char('j')))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(Event::Key(Key::Char('l'))));
    events.push(None);
    events.push(Some(Event::Key(Key::Char('j'))));
    events.push(None);
    events.push(Some(Event::Key(Key::Ctrl('c'))));
    let keyboard_events = Box::new(KeyboardEvents::new(events));

    let temp_dir_path = create_root_temp_dir("cannot_move_into_small_files").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 400000).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 1000000).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 1000000).expect("failed to create temp file");

    let mut file_4_path = PathBuf::from(&temp_dir_path);
    file_4_path.push("file4");
    create_temp_file(file_4_path, 8000).expect("failed to create temp file");

    let mut file_5_path = PathBuf::from(&temp_dir_path);
    file_5_path.push("file5");
    create_temp_file(file_5_path, 8000).expect("failed to create temp file");

    let mut file_6_path = PathBuf::from(&temp_dir_path);
    file_6_path.push("file6");
    create_temp_file(file_6_path, 8000).expect("failed to create temp file");

    let mut file_7_path = PathBuf::from(&temp_dir_path);
    file_7_path.push("file7");
    create_temp_file(file_7_path, 8000).expect("failed to create temp file");

    let mut file_8_path = PathBuf::from(&temp_dir_path);
    file_8_path.push("file8");
    create_temp_file(file_8_path, 8000).expect("failed to create temp file");

    let mut file_9_path = PathBuf::from(&temp_dir_path);
    file_9_path.push("file9");
    create_temp_file(file_9_path, 8000).expect("failed to create temp file");

    start(backend, keyboard_events, temp_dir_path.clone());
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();

    let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Clear, ShowCursor];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 4);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
}

#[test]
fn minimum_tile_sides () {
    // here we test that tiles are not created with a side_length (height in this case)
    // that is too small to render while not being designated as a "small file"
    //
    // the only case in which this can happen if this is the last tile to be placed
    // this case might in the future be solved by artificially increasing its size
    // to the minimum with some sort of asterisk to explain

    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);
    let keyboard_events = sleep_and_quit_events(1);
    let temp_dir_path = create_root_temp_dir("minimum_tile_sides").expect("failed to create temp dir");

    for i in 0..7 {
        let mut file_path = PathBuf::from(&temp_dir_path);
        file_path.push(format!("big_file{}", i));
        create_temp_file(file_path, 134000).expect("failed to create temp file");
    }

    for i in 0..2 {
        let mut file_path = PathBuf::from(&temp_dir_path);
        file_path.push(format!("medium_file{}", i));
        create_temp_file(file_path, 8000).expect("failed to create temp file");
    }

    for i in 0..50 {
        let mut file_path = PathBuf::from(&temp_dir_path);
        file_path.push(format!("file{}", i));
        create_temp_file(file_path, 4000).expect("failed to create temp file");
    }

    start(backend, keyboard_events, temp_dir_path.clone());
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().unwrap();

    let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Clear, ShowCursor];
    assert_eq!(
        &terminal_events.lock().unwrap()[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 1);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
}

#[test]
fn move_down_and_enter_folder() {

    let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

    let mut events: Vec<Option<Event>> = iter::repeat(None).take(2).collect();
    events.push(Some(Event::Key(Key::Char('j')))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(Event::Key(Key::Char('j'))));
    events.push(None);
    events.push(Some(Event::Key(Key::Char('\n'))));
    events.push(None);
    events.push(Some(Event::Key(Key::Ctrl('c'))));
    let keyboard_events = Box::new(KeyboardEvents::new(events));

    let temp_dir_path = create_root_temp_dir("move_down_and_enter_folder").expect("failed to create temp dir");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("file1");
    create_temp_file(file_1_path, 4000).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 5000).expect("failed to create temp file");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(subfolder_1_path).expect("failed to create temporary directory");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("subfolder1");
    file_3_path.push("file3");
    create_temp_file(file_3_path, 5000).expect("failed to create temp file");

    start(backend, keyboard_events, temp_dir_path.clone());
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Clear, ShowCursor];

    assert_eq!(
        &terminal_events.lock().expect("could not acquire lock on terminal_events")[..],
        &expected_terminal_events[..]
    );

    assert_eq!(terminal_draw_events_mirror.len(), 4);
    assert_snapshot!(&terminal_draw_events_mirror[0]);
    assert_snapshot!(&terminal_draw_events_mirror[1]);
    assert_snapshot!(&terminal_draw_events_mirror[2]);
    assert_snapshot!(&terminal_draw_events_mirror[3]);
}

#[test]
fn noop_when_entering_file() {
   let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

   let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
   events.push(Some(Event::Key(Key::Char('j')))); // once to place selected marker on screen
   events.push(None);
   events.push(Some(Event::Key(Key::Char('j'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Char('\n'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('c'))));
   let keyboard_events = Box::new(KeyboardEvents::new(events));

   let temp_dir_path = create_root_temp_dir("noop_when_entering_file").expect("failed to create temp dir");

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
   let terminal_draw_events_mirror = terminal_draw_events.lock().expect("could not acquire lock on terminal events");

   let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Clear, ShowCursor];

   assert_eq!(
       &terminal_events.lock().expect("could not acquire lock on terminal_events")[..],
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
    events.push(Some(Event::Key(Key::Char('j')))); // once to place selected marker on screen
    events.push(None);
    events.push(Some(Event::Key(Key::Char('j'))));
    events.push(None);
    events.push(Some(Event::Key(Key::Char('k'))));
    events.push(None);
    events.push(Some(Event::Key(Key::Char('\n'))));
    events.push(None);
    events.push(Some(Event::Key(Key::Ctrl('c'))));
    let keyboard_events = Box::new(KeyboardEvents::new(events));

    let temp_dir_path = create_root_temp_dir("move_up_and_enter_folder").expect("failed to create temp dir");

    let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
    subfolder_1_path.push("subfolder1");
    create_dir(subfolder_1_path).expect("failed to create temporary directory");

    let mut file_1_path = PathBuf::from(&temp_dir_path);
    file_1_path.push("subfolder1");
    file_1_path.push("file1");
    create_temp_file(file_1_path, 10000).expect("failed to create temp file");

    let mut file_2_path = PathBuf::from(&temp_dir_path);
    file_2_path.push("file2");
    create_temp_file(file_2_path, 4000).expect("failed to create temp file");

    let mut file_3_path = PathBuf::from(&temp_dir_path);
    file_3_path.push("file3");
    create_temp_file(file_3_path, 5000).expect("failed to create temp file");

    start(backend, keyboard_events, temp_dir_path.clone());
    std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
    let terminal_draw_events_mirror = terminal_draw_events.lock().expect("could not acquire lock on terminal events");

    let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Clear, ShowCursor];
    assert_eq!(
        &terminal_events.lock().expect("could not acquire lock on terminal_events")[..],
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
fn move_right_and_enter_folder() {
   let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

   let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
   events.push(Some(Event::Key(Key::Char('l')))); // once to place selected marker on screen
   events.push(None);
   events.push(Some(Event::Key(Key::Char('l'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Char('\n'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('c'))));
   let keyboard_events = Box::new(KeyboardEvents::new(events));

   let temp_dir_path = create_root_temp_dir("move_right_and_enter_folder").expect("failed to create temp dir");

   let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
   subfolder_1_path.push("subfolder1");
   create_dir(subfolder_1_path).expect("failed to create temporary directory");

   let mut file_1_path = PathBuf::from(&temp_dir_path);
   file_1_path.push("subfolder1");
   file_1_path.push("file1");
   create_temp_file(file_1_path, 4000).expect("failed to create temp file");

   let mut file_2_path = PathBuf::from(&temp_dir_path);
   file_2_path.push("file2");
   create_temp_file(file_2_path, 4000).expect("failed to create temp file");

   let mut file_3_path = PathBuf::from(&temp_dir_path);
   file_3_path.push("file3");
   create_temp_file(file_3_path, 4000).expect("failed to create temp file");

   start(backend, keyboard_events, temp_dir_path.clone());
   std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
   let terminal_draw_events_mirror = terminal_draw_events.lock().expect("could not acquire lock on terminal events");

   let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Clear, ShowCursor];

   assert_eq!(
       &terminal_events.lock().expect("could not acquire lock on terminal_events")[..],
       &expected_terminal_events[..]
   );

   assert_eq!(terminal_draw_events_mirror.len(), 4);
   assert_snapshot!(&terminal_draw_events_mirror[0]);
   assert_snapshot!(&terminal_draw_events_mirror[1]);
   assert_snapshot!(&terminal_draw_events_mirror[2]);
   assert_snapshot!(&terminal_draw_events_mirror[3]);
}

#[test]
fn move_left_and_enter_folder() {
   let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

   let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
   events.push(Some(Event::Key(Key::Char('l')))); // once to place selected marker on screen
   events.push(None);
   events.push(Some(Event::Key(Key::Char('l'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Char('h'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Char('\n'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('c'))));
   let keyboard_events = Box::new(KeyboardEvents::new(events));

   let temp_dir_path = create_root_temp_dir("move_left_and_enter_folder").expect("failed to create temp dir");

   let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
   subfolder_1_path.push("subfolder1");
   create_dir(subfolder_1_path).expect("failed to create temporary directory");

   let mut file_1_path = PathBuf::from(&temp_dir_path);
   file_1_path.push("subfolder1");
   file_1_path.push("file1");
   create_temp_file(file_1_path, 8000).expect("failed to create temp file");

   let mut file_2_path = PathBuf::from(&temp_dir_path);
   file_2_path.push("file2");
   create_temp_file(file_2_path, 4000).expect("failed to create temp file");

   let mut file_3_path = PathBuf::from(&temp_dir_path);
   file_3_path.push("file3");
   create_temp_file(file_3_path, 4000).expect("failed to create temp file");

   start(backend, keyboard_events, temp_dir_path.clone());
   std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
   let terminal_draw_events_mirror = terminal_draw_events.lock().expect("could not acquire lock on terminal events");

   let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Clear, ShowCursor];
   assert_eq!(
       &terminal_events.lock().expect("could not acquire lock on terminal_events")[..],
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
fn noop_when_moving_off_screen_edges() {
   let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

   let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
   events.push(Some(Event::Key(Key::Char('l')))); // once to place selected marker on screen
   events.push(None);
   events.push(Some(Event::Key(Key::Char('l'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Char('l'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('c'))));
   let keyboard_events = Box::new(KeyboardEvents::new(events));

   let temp_dir_path = create_root_temp_dir("noop_when_moving_off_screen_edges").expect("failed to create temp dir");

   let mut file_1_path = PathBuf::from(&temp_dir_path);
   file_1_path.push("file1");
   create_temp_file(file_1_path, 4000).expect("failed to create temp file");

   let mut file_2_path = PathBuf::from(&temp_dir_path);
   file_2_path.push("file2");
   create_temp_file(file_2_path, 4000).expect("failed to create temp file");

   let mut file_3_path = PathBuf::from(&temp_dir_path);
   file_3_path.push("file3");
   create_temp_file(file_3_path, 4000).expect("failed to create temp file");

   start(backend, keyboard_events, temp_dir_path.clone());
   std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
   let terminal_draw_events_mirror = terminal_draw_events.lock().expect("could not acquire lock on terminal events");

   let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Clear, ShowCursor];
   assert_eq!(
       &terminal_events.lock().expect("could not acquire lock on terminal_events")[..],
       &expected_terminal_events[..]
   );

   assert_eq!(terminal_draw_events_mirror.len(), 4);
   assert_snapshot!(&terminal_draw_events_mirror[0]);
   assert_snapshot!(&terminal_draw_events_mirror[1]);
   assert_snapshot!(&terminal_draw_events_mirror[2]);
   assert_snapshot!(&terminal_draw_events_mirror[3]);
}

#[test]
fn esc_to_go_up() {
   let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

   let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
   events.push(Some(Event::Key(Key::Char('l')))); // once to place selected marker on screen
   events.push(None);
   events.push(Some(Event::Key(Key::Char('l'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Char('\n'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Esc)));
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('c'))));
   let keyboard_events = Box::new(KeyboardEvents::new(events));

   let temp_dir_path = create_root_temp_dir("esc_to_go_up").expect("failed to create temp dir");

   let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
   subfolder_1_path.push("subfolder1");
   create_dir(subfolder_1_path).expect("failed to create temporary directory");

   let mut file_1_path = PathBuf::from(&temp_dir_path);
   file_1_path.push("subfolder1");
   file_1_path.push("file1");
   create_temp_file(file_1_path, 4000).expect("failed to create temp file");

   let mut file_2_path = PathBuf::from(&temp_dir_path);
   file_2_path.push("file2");
   create_temp_file(file_2_path, 4000).expect("failed to create temp file");

   let mut file_3_path = PathBuf::from(&temp_dir_path);
   file_3_path.push("file3");
   create_temp_file(file_3_path, 4000).expect("failed to create temp file");

   start(backend, keyboard_events, temp_dir_path.clone());
   std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
   let terminal_draw_events_mirror = terminal_draw_events.lock().expect("could not acquire lock on terminal events");

   let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Clear, ShowCursor];
   assert_eq!(
       &terminal_events.lock().expect("could not acquire lock on terminal_events")[..],
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
fn noop_when_pressing_esc_at_base_folder() {
   let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

   let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
   events.push(Some(Event::Key(Key::Char('l')))); // once to place selected marker on screen
   events.push(None);
   events.push(Some(Event::Key(Key::Char('l'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Char('\n'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Esc)));
   events.push(None);
   events.push(Some(Event::Key(Key::Esc)));
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('c'))));
   let keyboard_events = Box::new(KeyboardEvents::new(events));

   let temp_dir_path = create_root_temp_dir("noop_when_pressing_esc_at_base_folder").expect("failed to create temp dir");

   let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
   subfolder_1_path.push("subfolder1");
   create_dir(subfolder_1_path).expect("failed to create temporary directory");

   let mut file_1_path = PathBuf::from(&temp_dir_path);
   file_1_path.push("subfolder1");
   file_1_path.push("file1");
   create_temp_file(file_1_path, 4000).expect("failed to create temp file");

   let mut file_2_path = PathBuf::from(&temp_dir_path);
   file_2_path.push("file2");
   create_temp_file(file_2_path, 4000).expect("failed to create temp file");

   let mut file_3_path = PathBuf::from(&temp_dir_path);
   file_3_path.push("file3");
   create_temp_file(file_3_path, 4000).expect("failed to create temp file");

   start(backend, keyboard_events, temp_dir_path.clone());
   std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");
   let terminal_draw_events_mirror = terminal_draw_events.lock().expect("could not acquire lock on terminal events");

   let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Clear, ShowCursor];
   assert_eq!(
       &terminal_events.lock().expect("could not acquire lock on terminal_events")[..],
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
fn delete_file() {
   let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

   let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
   events.push(Some(Event::Key(Key::Char('l')))); // once to place selected marker on screen
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('d'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Char('y'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('c'))));
   let keyboard_events = Box::new(KeyboardEvents::new(events));

   let temp_dir_path = create_root_temp_dir("delete_file").expect("failed to create temp dir");

   let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
   subfolder_1_path.push("subfolder1");
   create_dir(&subfolder_1_path).expect("failed to create temporary directory");

   let mut file_1_path = PathBuf::from(&temp_dir_path);
   file_1_path.push("subfolder1");
   file_1_path.push("file1");
   create_temp_file(&file_1_path, 4000).expect("failed to create temp file");

   let mut file_2_path = PathBuf::from(&temp_dir_path);
   file_2_path.push("file2");
   create_temp_file(&file_2_path, 4000).expect("failed to create temp file");

   let mut file_3_path = PathBuf::from(&temp_dir_path);
   file_3_path.push("file3");
   create_temp_file(&file_3_path, 4000).expect("failed to create temp file");

   start(backend, keyboard_events, temp_dir_path.clone());
   let terminal_draw_events_mirror = terminal_draw_events.lock().expect("could not acquire lock on terminal events");

   let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Clear, ShowCursor];
   assert_eq!(
       &terminal_events.lock().expect("could not acquire lock on terminal_events")[..],
       &expected_terminal_events[..]
   );
   assert_eq!(std::fs::metadata(&file_2_path).is_err(), true, "file successfully deleted");
   assert_eq!(std::fs::metadata(&subfolder_1_path).is_ok(), true, "different folder stayed the same");
   assert_eq!(std::fs::metadata(&file_1_path).is_ok(), true, "different file was untoucehd");
   assert_eq!(std::fs::metadata(&file_3_path).is_ok(), true, "second different file was untouched");
   std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

   assert_eq!(terminal_draw_events_mirror.len(), 4);
   assert_snapshot!(&terminal_draw_events_mirror[0]);
   assert_snapshot!(&terminal_draw_events_mirror[1]);
   assert_snapshot!(&terminal_draw_events_mirror[2]);
   assert_snapshot!(&terminal_draw_events_mirror[3]);

}

#[test]
fn delete_folder() {
   let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

   let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
   events.push(Some(Event::Key(Key::Char('l')))); // once to place selected marker on screen
   events.push(None);
   events.push(Some(Event::Key(Key::Char('l'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('d'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Char('y'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('c'))));
   let keyboard_events = Box::new(KeyboardEvents::new(events));

   let temp_dir_path = create_root_temp_dir("delete_folder").expect("failed to create temp dir");

   let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
   subfolder_1_path.push("subfolder1");
   create_dir(&subfolder_1_path).expect("failed to create temporary directory");

   let mut file_1_path = PathBuf::from(&temp_dir_path);
   file_1_path.push("subfolder1");
   file_1_path.push("file1");
   create_temp_file(&file_1_path, 4000).expect("failed to create temp file");

   let mut file_2_path = PathBuf::from(&temp_dir_path);
   file_2_path.push("file2");
   create_temp_file(&file_2_path, 4000).expect("failed to create temp file");

   let mut file_3_path = PathBuf::from(&temp_dir_path);
   file_3_path.push("file3");
   create_temp_file(&file_3_path, 4000).expect("failed to create temp file");

   start(backend, keyboard_events, temp_dir_path.clone());
   let terminal_draw_events_mirror = terminal_draw_events.lock().expect("could not acquire lock on terminal events");

   let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Clear, ShowCursor];
   assert_eq!(
       &terminal_events.lock().expect("could not acquire lock on terminal_events")[..],
       &expected_terminal_events[..]
   );
   assert_eq!(std::fs::metadata(&subfolder_1_path).is_err(), true, "folder successfully deleted");
   assert_eq!(std::fs::metadata(&file_1_path).is_err(), true, "internal file successfully deleted"); // can't really fail on its own, but left here for clarity
   assert_eq!(std::fs::metadata(&file_2_path).is_ok(), true, "different file was untouched");
   assert_eq!(std::fs::metadata(&file_3_path).is_ok(), true, "second different file was untouched");
   std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

   assert_eq!(terminal_draw_events_mirror.len(), 5);
   assert_snapshot!(&terminal_draw_events_mirror[0]);
   assert_snapshot!(&terminal_draw_events_mirror[1]);
   assert_snapshot!(&terminal_draw_events_mirror[2]);
   assert_snapshot!(&terminal_draw_events_mirror[3]);
   assert_snapshot!(&terminal_draw_events_mirror[4]);
}

#[test]
fn delete_folder_small_window () {
   // terminal window with a width of 60 (shorter message window layout)
   let (terminal_events, terminal_draw_events, backend) = test_backend_factory(60, 50);

   let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
   events.push(Some(Event::Key(Key::Char('j')))); // once to place selected marker on screen
   events.push(None);
   events.push(Some(Event::Key(Key::Char('j'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('d'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Char('y'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('c'))));
   let keyboard_events = Box::new(KeyboardEvents::new(events));

   let temp_dir_path = create_root_temp_dir("delete_folder_small_wikndow").expect("failed to create temp dir");

   let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
   subfolder_1_path.push("subfolder1");
   create_dir(&subfolder_1_path).expect("failed to create temporary directory");

   let mut file_1_path = PathBuf::from(&temp_dir_path);
   file_1_path.push("subfolder1");
   file_1_path.push("file1");
   create_temp_file(&file_1_path, 4000).expect("failed to create temp file");

   let mut file_2_path = PathBuf::from(&temp_dir_path);
   file_2_path.push("file2");
   create_temp_file(&file_2_path, 4000).expect("failed to create temp file");

   let mut file_3_path = PathBuf::from(&temp_dir_path);
   file_3_path.push("file3");
   create_temp_file(&file_3_path, 4000).expect("failed to create temp file");

   start(backend, keyboard_events, temp_dir_path.clone());
   let terminal_draw_events_mirror = terminal_draw_events.lock().expect("could not acquire lock on terminal events");

   let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Clear, ShowCursor];
   assert_eq!(
       &terminal_events.lock().expect("could not acquire lock on terminal_events")[..],
       &expected_terminal_events[..]
   );
   assert_eq!(std::fs::metadata(&file_2_path).is_ok(), true, "different file was untouched");
   assert_eq!(std::fs::metadata(&subfolder_1_path).is_err(), true, "file successfully deleted");
   assert_eq!(std::fs::metadata(&file_1_path).is_err(), true, "file in folder deleted");
   assert_eq!(std::fs::metadata(&file_3_path).is_ok(), true, "second different file was untouched");
   std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

   assert_eq!(terminal_draw_events_mirror.len(), 5);
   assert_snapshot!(&terminal_draw_events_mirror[0]);
   assert_snapshot!(&terminal_draw_events_mirror[1]);
   assert_snapshot!(&terminal_draw_events_mirror[2]);
   assert_snapshot!(&terminal_draw_events_mirror[3]);
   assert_snapshot!(&terminal_draw_events_mirror[4]);
}

#[test]
fn delete_folder_with_multiple_children() {
   let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

   let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
   events.push(Some(Event::Key(Key::Char('l')))); // once to place selected marker on screen
   events.push(None);
   events.push(Some(Event::Key(Key::Char('l'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('d'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Char('y'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('c'))));
   let keyboard_events = Box::new(KeyboardEvents::new(events));

   let temp_dir_path = create_root_temp_dir("delete_folder_with_multiple_children").expect("failed to create temp dir");

   let mut file_1_path = PathBuf::from(&temp_dir_path);
   file_1_path.push("file1");
   create_temp_file(&file_1_path, 16000).expect("failed to create temp file");

   let mut file_2_path = PathBuf::from(&temp_dir_path);
   file_2_path.push("file2");
   create_temp_file(&file_2_path, 16000).expect("failed to create temp file");

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
   create_temp_file(&file_3_path, 4000).expect("failed to create temp file");

   let mut file_4_path = PathBuf::from(&temp_dir_path);
   file_4_path.push("subfolder1");
   file_4_path.push("subfolder2");
   file_4_path.push("file4");
   create_temp_file(&file_4_path, 4000).expect("failed to create temp file");

   let mut file_5_path = PathBuf::from(&temp_dir_path);
   file_5_path.push("subfolder1");
   file_5_path.push("file5");
   create_temp_file(&file_5_path, 4000).expect("failed to create temp file");

   start(backend, keyboard_events, temp_dir_path.clone());
   let terminal_draw_events_mirror = terminal_draw_events.lock().expect("could not acquire lock on terminal events");

   let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Clear, ShowCursor];
   assert_eq!(
       &terminal_events.lock().expect("could not acquire lock on terminal_events")[..],
       &expected_terminal_events[..]
   );
   assert_eq!(std::fs::metadata(&subfolder_1_path).is_err(), true, "folder successfully deleted");
   assert_eq!(std::fs::metadata(&subfolder_2_path).is_err(), true, "folder inside deleted folder successfully deleted");
   assert_eq!(std::fs::metadata(&file_1_path).is_ok(), true, "different file was untouched");
   assert_eq!(std::fs::metadata(&file_2_path).is_ok(), true, "different file was untouched");
   assert_eq!(std::fs::metadata(&file_3_path).is_err(), true, "internal file in folder deleted");
   assert_eq!(std::fs::metadata(&file_4_path).is_err(), true, "internal file in folder deleted");
   assert_eq!(std::fs::metadata(&file_5_path).is_err(), true, "internal file in folder deleted");
   std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

   assert_eq!(terminal_draw_events_mirror.len(), 5);
   assert_snapshot!(&terminal_draw_events_mirror[0]);
   assert_snapshot!(&terminal_draw_events_mirror[1]);
   assert_snapshot!(&terminal_draw_events_mirror[2]);
   assert_snapshot!(&terminal_draw_events_mirror[3]);
   assert_snapshot!(&terminal_draw_events_mirror[4]);
}

#[test]
fn pressing_delete_with_no_selected_tile() {
   let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

   let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
   events.push(Some(Event::Key(Key::Ctrl('d'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('c'))));
   let keyboard_events = Box::new(KeyboardEvents::new(events));

   let temp_dir_path = create_root_temp_dir("pressing_delete_with_no_selected_tile").expect("failed to create temp dir");

   let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
   subfolder_1_path.push("subfolder1");
   create_dir(&subfolder_1_path).expect("failed to create temporary directory");

   let mut file_1_path = PathBuf::from(&temp_dir_path);
   file_1_path.push("subfolder1");
   file_1_path.push("file1");
   create_temp_file(&file_1_path, 4000).expect("failed to create temp file");

   let mut file_2_path = PathBuf::from(&temp_dir_path);
   file_2_path.push("file2");
   create_temp_file(&file_2_path, 4000).expect("failed to create temp file");

   let mut file_3_path = PathBuf::from(&temp_dir_path);
   file_3_path.push("file3");
   create_temp_file(&file_3_path, 4000).expect("failed to create temp file");

   start(backend, keyboard_events, temp_dir_path.clone());
   let terminal_draw_events_mirror = terminal_draw_events.lock().expect("could not acquire lock on terminal events");

   let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Clear, ShowCursor];
   assert_eq!(
       &terminal_events.lock().expect("could not acquire lock on terminal_events")[..],
       &expected_terminal_events[..]
   );
   assert_eq!(std::fs::metadata(&file_2_path).is_ok(), true, "file not deleted");
   assert_eq!(std::fs::metadata(&subfolder_1_path).is_ok(), true, "different folder stayed the same");
   assert_eq!(std::fs::metadata(&file_1_path).is_ok(), true, "different file was untoucehd");
   assert_eq!(std::fs::metadata(&file_3_path).is_ok(), true, "second different file was untouched");
   std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

   assert_eq!(terminal_draw_events_mirror.len(), 2);
   assert_snapshot!(&terminal_draw_events_mirror[0]);
   assert_snapshot!(&terminal_draw_events_mirror[1]);
}

#[test]
fn delete_file_press_n() {
   let (terminal_events, terminal_draw_events, backend) = test_backend_factory(190, 50);

   let mut events: Vec<Option<Event>> = iter::repeat(None).take(1).collect();
   events.push(Some(Event::Key(Key::Char('l')))); // once to place selected marker on screen
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('d'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Char('n'))));
   events.push(None);
   events.push(Some(Event::Key(Key::Ctrl('c'))));
   let keyboard_events = Box::new(KeyboardEvents::new(events));

   let temp_dir_path = create_root_temp_dir("delete_file_press_n").expect("failed to create temp dir");

   let mut subfolder_1_path = PathBuf::from(&temp_dir_path);
   subfolder_1_path.push("subfolder1");
   create_dir(&subfolder_1_path).expect("failed to create temporary directory");

   let mut file_1_path = PathBuf::from(&temp_dir_path);
   file_1_path.push("subfolder1");
   file_1_path.push("file1");
   create_temp_file(&file_1_path, 4000).expect("failed to create temp file");

   let mut file_2_path = PathBuf::from(&temp_dir_path);
   file_2_path.push("file2");
   create_temp_file(&file_2_path, 4000).expect("failed to create temp file");

   let mut file_3_path = PathBuf::from(&temp_dir_path);
   file_3_path.push("file3");
   create_temp_file(&file_3_path, 4000).expect("failed to create temp file");

   start(backend, keyboard_events, temp_dir_path.clone());
   let terminal_draw_events_mirror = terminal_draw_events.lock().expect("could not acquire lock on terminal events");

   let expected_terminal_events = vec![Clear, HideCursor, Draw, Flush, Draw, Flush, Draw, Flush, Draw, Flush, Clear, ShowCursor];
   assert_eq!(
       &terminal_events.lock().expect("could not acquire lock on terminal_events")[..],
       &expected_terminal_events[..]
   );
   assert_eq!(std::fs::metadata(&file_2_path).is_ok(), true, "file not deleted");
   assert_eq!(std::fs::metadata(&subfolder_1_path).is_ok(), true, "different folder stayed the same");
   assert_eq!(std::fs::metadata(&file_1_path).is_ok(), true, "different file was untoucehd");
   assert_eq!(std::fs::metadata(&file_3_path).is_ok(), true, "second different file was untouched");
   std::fs::remove_dir_all(temp_dir_path).expect("failed to remove temporary folder");

   assert_eq!(terminal_draw_events_mirror.len(), 4);
   assert_snapshot!(&terminal_draw_events_mirror[0]);
   assert_snapshot!(&terminal_draw_events_mirror[1]);
   assert_snapshot!(&terminal_draw_events_mirror[2]);
   assert_snapshot!(&terminal_draw_events_mirror[3]);
}
