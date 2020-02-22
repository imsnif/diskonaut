#[allow(dead_code)]

use std::collections::HashMap;
use std::env;
use std::io;
use std::fs;
use std::os::unix::fs::MetadataExt; // TODO: support other OSs
use ::std::{thread, time};

use ::std::sync::atomic::{AtomicBool, Ordering};

use failure;

use termion::event::Key;
use termion::raw::IntoRawMode;
use tui::backend::TermionBackend;
use tui::layout::Rect;
use tui::widgets::Widget;
use tui::Terminal;

use std::process;

use walkdir::WalkDir;

use ::std::fmt;

use std::path::PathBuf;

use ::tui::backend::Backend;
use ::std::sync::Arc;

use ::std::io::stdin;
use ::termion::input::TermRead;
use ::termion::event::Event;

pub fn scan_folder (path: PathBuf) -> HashMap<String, u64> {
    let mut file_sizes: HashMap<String, u64> = HashMap::new();
    let path_length = path.components().count() + 1;
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path().clone();
        // all_files.push(entry_path.clone());
        match fs::metadata(entry_path.display().to_string()) {
            Ok(file_metadata) => {
                let file_size_entry_name = entry_path.iter().take(path_length).last().expect("failed to get path name"); // TODO: less hacky (also, 3 should be the length of the base dir)
                let file_size_entry = file_sizes.entry(String::from(file_size_entry_name.to_string_lossy())).or_insert(0);
                *file_size_entry += file_metadata.blocks() * 512;
            },
            Err(_e) => {
                // println!("\rerror opening {:?} {:?}", entry, e); // TODO: look into these
            }
        }
    }
    file_sizes
}
