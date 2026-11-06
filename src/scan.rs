use std::path::{Path, PathBuf};

use jwalk::Parallelism::{RayonDefaultPool, Serial};
use jwalk::WalkDir;

pub(crate) fn walk_dir(path: &Path, ignored_paths: Vec<PathBuf>) -> WalkDir {
    let walk_dir = WalkDir::new(path)
        .parallelism(if crate::SHOULD_SCAN_HD_FILES_IN_MULTIPLE_THREADS {
            RayonDefaultPool
        } else {
            Serial
        })
        .skip_hidden(false)
        .follow_links(false);
    if ignored_paths.is_empty() {
        return walk_dir;
    }

    let ignored_paths: Vec<PathBuf> = ignored_paths
        .into_iter()
        .map(|ignored_path| path.join(ignored_path))
        .collect();
    walk_dir.process_read_dir(move |_, children| {
        // Remove entries before jwalk schedules their directories for traversal.
        children.retain(|entry| {
            entry
                .as_ref()
                .map(|entry| !ignored_paths.contains(&entry.path()))
                .unwrap_or(true)
        });
    })
}
