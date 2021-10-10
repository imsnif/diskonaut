use ::std::ffi::OsString;

use crate::state::files::{FileOrFolder, Folder};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FileType {
    File,
    Folder,
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub name: OsString,
    pub size: u128,
    pub descendants: Option<u64>,
    pub percentage: f64, // 1.0 is 100% (0.5 is 50%, etc.)
    pub file_type: FileType,
}

fn calculate_percentage(size: u128, total_size: u128, total_files_in_parent: usize) -> f64 {
    if size == 0 && total_size == 0 {
        // if all files in the folder are of size 0, we'll want to display them all as
        // the same size
        1.0 / total_files_in_parent as f64
    } else {
        size as f64 / total_size as f64
    }
}

pub fn files_in_folder(folder: &Folder, offset: usize) -> Vec<FileMetadata> {
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
            let percentage = calculate_percentage(size, total_size, folder.contents.len());
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
        if (a.percentage - b.percentage).abs() < f64::EPSILON {
            a.name.partial_cmp(&b.name).expect("could not compare name")
        } else {
            b.percentage
                .partial_cmp(&a.percentage)
                .expect("could not compare percentage")
        }
    });
    if offset > 0 {
        let removed_items = files.drain(..offset);
        let number_of_files_without_removed_contents = folder.contents.len() - removed_items.len();
        let removed_size = removed_items.fold(0, |acc, file| acc + file.size);
        let size_without_removed_items = total_size - removed_size;
        for file in &mut files {
            file.percentage = calculate_percentage(
                file.size,
                size_without_removed_items,
                number_of_files_without_removed_contents,
            );
        }
    }
    files
}
