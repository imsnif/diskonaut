use std::ffi::OsString;

use crate::state::files::{FileOrFolder, Folder};

#[derive(Copy, Clone, Debug)]
pub enum FileType {
    File,
    Folder,
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub name: OsString,
    pub size: u64,
    pub descendants: Option<u64>,
    pub percentage: f64, // 1.0 is 100% (0.5 is 50%, etc.)
    pub file_type: FileType,
}

pub fn files_in_folder(folder: &Folder) -> Vec<FileMetadata> {
    let mut files = Vec::new();
    let total_size = folder.size;
    for (name, file_or_folder) in &folder.contents {
        files.push({
            let size = file_or_folder.size();
            let name = name.clone();
            let (descendants, file_type) = match file_or_folder {
                FileOrFolder::Folder(folder) => (Some(folder.num_descendants), FileType::Folder),
                FileOrFolder::File(_file) => (None, FileType::File),
            };
            let percentage = if size == 0 && total_size == 0 {
                // if all files in the folder are of size 0, we'll want to display them all as
                // the same size
                1.0 / folder.contents.len() as f64
            } else {
                size as f64 / total_size as f64
            };
            FileMetadata {
                size,
                name,
                descendants,
                percentage,
                file_type,
            }
        });
    }
    files.sort_by(|a, b| {
        if a.percentage == b.percentage {
            a.name.partial_cmp(&b.name).expect("could not compare name")
        } else {
            b.percentage
                .partial_cmp(&a.percentage)
                .expect("could not compare percentage")
        }
    });
    files
}
