use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::scan::walk_dir;

static NEXT_FIXTURE: AtomicUsize = AtomicUsize::new(0);

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn new(parent: &Path) -> Self {
        let root = parent.join(format!(
            "diskonaut-ignore-{}-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos(),
            NEXT_FIXTURE.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&root).expect("could not create scan fixture");
        Self { root }
    }

    fn file(&self, relative_path: &str) {
        let path = self.root.join(relative_path);
        fs::create_dir_all(path.parent().unwrap()).expect("could not create fixture directory");
        fs::write(path, b"fixture").expect("could not create fixture file");
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        fs::remove_dir_all(&self.root).expect("could not remove scan fixture");
    }
}

fn indexed_paths(root: &Path, ignored: &[&str]) -> BTreeSet<PathBuf> {
    walk_dir(root, ignored.iter().map(PathBuf::from).collect())
        .into_iter()
        .map(|entry| {
            entry
                .expect("unexpected scan error")
                .path()
                .strip_prefix(root)
                .expect("scan entry outside fixture")
                .to_path_buf()
        })
        .filter(|relative_path| !relative_path.as_os_str().is_empty())
        .collect()
}

fn paths(expected: &[&str]) -> BTreeSet<PathBuf> {
    expected.iter().map(PathBuf::from).collect()
}

#[test]
fn scan_without_ignores_includes_files_directories_and_hidden_entries() {
    let fixture = Fixture::new(&std::env::temp_dir());
    fixture.file("keep.txt");
    fixture.file("nested/file.txt");
    fixture.file(".hidden/file.txt");

    assert_eq!(
        indexed_paths(&fixture.root, &[]),
        paths(&[
            "keep.txt",
            "nested",
            "nested/file.txt",
            ".hidden",
            ".hidden/file.txt",
        ])
    );
}

#[test]
fn multiple_ignores_skip_files_and_entire_directory_subtrees() {
    let fixture = Fixture::new(&std::env::temp_dir());
    fixture.file("keep.txt");
    fixture.file("omit.txt");
    fixture.file("cache/deep/omit.txt");
    fixture.file("cache-backup/keep.txt");

    assert_eq!(
        indexed_paths(&fixture.root, &["omit.txt", "cache", "cache/deep"]),
        paths(&["keep.txt", "cache-backup", "cache-backup/keep.txt"])
    );
}

#[test]
fn nested_ignore_is_relative_to_scan_root_and_preserves_matching_names_elsewhere() {
    let fixture = Fixture::new(&std::env::temp_dir());
    fixture.file("project/cache/omit.txt");
    fixture.file("project/keep.txt");
    fixture.file("cache/keep.txt");
    fixture.file("other/cache/keep.txt");

    assert_eq!(
        indexed_paths(&fixture.root, &["project/cache"]),
        paths(&[
            "project",
            "project/keep.txt",
            "cache",
            "cache/keep.txt",
            "other",
            "other/cache",
            "other/cache/keep.txt",
        ])
    );
}

#[test]
fn nonexistent_ignore_does_not_change_indexed_paths() {
    let fixture = Fixture::new(&std::env::temp_dir());
    fixture.file("nested/keep.txt");

    assert_eq!(
        indexed_paths(&fixture.root, &["missing", "nested/missing"]),
        indexed_paths(&fixture.root, &[])
    );
}

#[test]
fn ignores_work_when_scan_root_is_relative_with_dot_prefix() {
    let fixture = Fixture::new(Path::new("."));
    fixture.file("keep.txt");
    fixture.file("cache/deep/omit.txt");

    assert_eq!(
        indexed_paths(&fixture.root, &["cache"]),
        paths(&["keep.txt"])
    );
}
