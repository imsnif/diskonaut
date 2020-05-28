use crate::state::files::{FileOrFolder, Folder};
use std::path::{Path, PathBuf};
use std::ffi::{OsString, OsStr};
use std::fs::Metadata;

pub struct FileTree {
    base_folder: Folder,
    current_folder_names: Vec<OsString>,
    pub space_freed: u64, // TODO: move elsewhere
    pub failed_to_read: u64,
    pub path_in_filesystem: PathBuf,
}

impl FileTree {
    pub fn new(base_folder: Folder, path_in_filesystem: PathBuf) -> Self {
        FileTree {
            base_folder,
            current_folder_names: Vec::new(),
            path_in_filesystem,
            space_freed: 0,
            failed_to_read: 0,
        }

    }
    pub fn get_total_size (&self) -> u64 {
        self.base_folder.size
    }
    pub fn get_total_descendants (&self) -> u64 {
        self.base_folder.num_descendants
    }
    pub fn get_current_folder (&self) -> &Folder {
        if self.current_folder_names.is_empty() {
            &self.base_folder
        } else if let Some(FileOrFolder::Folder(current_folder)) = self.base_folder.path(&self.current_folder_names) {
            &current_folder
        } else {
            // here we have something in current_folder_names but the last
            // one is somehow not a folder... this is a corrupted state
            unreachable!("couldn't find current folder size")
        }
    }
    pub fn get_current_folder_size (&self) -> u64 {
        self.get_current_folder().size
    }
    pub fn get_current_path (&self) -> PathBuf {
        let mut full_path = PathBuf::from(&self.path_in_filesystem);
        for folder in &self.current_folder_names {
            full_path.push(&folder)
        }
        return full_path;
    }
    pub fn item_in_current_folder(&self, item_name: &OsStr) -> Option<&FileOrFolder> {
        let current_folder = &self.get_current_folder();
        current_folder.path(&vec![item_name.to_os_string()])
    }
    pub fn enter_folder(&mut self, folder_name: &OsStr) {
        self.current_folder_names.push(folder_name.to_os_string());
    }
    pub fn leave_folder(&mut self) -> bool { // true => succeeded, false => at base folder
        match self.current_folder_names.pop() {
            Some(_) => true,
            None => false
        }
    }
    pub fn delete_file(&mut self, file_name: &OsStr) {
        let path_to_delete = &mut self.current_folder_names.clone();
        path_to_delete.push(file_name.to_os_string());
        self.base_folder.delete_path(&path_to_delete);
    }
    pub fn add_entry(&mut self, entry_metadata: &Metadata, entry_full_path: &Path, base_path_length: &usize) {
        self.base_folder.add_entry(entry_metadata, entry_full_path, base_path_length);
    }
}
