use std::collections::{HashMap, VecDeque};
use std::fs::Metadata;
use std::os::unix::fs::MetadataExt; // TODO: support other OSs

use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub enum FileOrFolder {
    Folder(Folder),
    File(File),
}

impl FileOrFolder {
    pub fn name (&self) -> &str {
        match self {
            FileOrFolder::Folder(folder) => &folder.name,
            FileOrFolder::File(file) => &file.name,
        }
    }
    pub fn size (&self) -> u64 {
        match self {
            FileOrFolder::Folder(folder) => folder.size,
            FileOrFolder::File(file) => file.size,
        }
    }
}

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub size: u64,
}

#[derive(Debug, Clone)]
pub struct Folder {
    pub name: String,
    pub contents: HashMap<String, FileOrFolder>,
    pub size: u64,
    pub num_descendants: u64,
}

impl Folder {
    pub fn new (path: &PathBuf) -> Self {
        let base_folder_name = path.iter().last().expect("could not get path base name").to_string_lossy();
        Self {
            name: String::from(base_folder_name),
            contents: HashMap::new(),
            size: 0,
            num_descendants: 0,
        }
    }

    pub fn add_entry(&mut self, entry_metadata: &Metadata, entry_full_path: &Path, base_path_length: &usize) {
        let mut relative_path = PathBuf::new();
        for dir in entry_full_path.components().skip(*base_path_length) {
            relative_path.push(dir);
        }
        if entry_metadata.is_dir() {
            self.add_folder(relative_path);
        } else {
            let size = entry_metadata.blocks() * 512; // TODO: support other OSs that do not have blocks
            self.add_file(relative_path, size);
        }
    }


    pub fn add_folder (&mut self, path: PathBuf) {
        let path_length = path.components().count();
        if path_length == 0 {
            return
        }
        if path_length > 1 {
            let name = String::from(path.iter().next().expect("could not get next path element for folder").to_string_lossy());
            let path_entry = self.contents.entry(name.clone()).or_insert( 
                FileOrFolder::Folder(
                    Folder {
                        name,
                        contents: HashMap::new(),
                        size: 0,
                        num_descendants: 0,
                    }
                )
            );
            self.num_descendants += 1;
            match path_entry { // TODO: less ugly
                FileOrFolder::Folder(folder) => folder.add_folder(path.iter().skip(1).collect()),
                _ => {}
            };
        } else {
            let name = String::from(path.iter().next().expect("could not get next path element for file").to_string_lossy());
            self.num_descendants += 1;
            self.contents.insert(name.clone(),
                FileOrFolder::Folder(
                    Folder {
                        name,
                        contents: HashMap::new(),
                        size: 0,
                        num_descendants: 0,
                    }
                )
            );
        }
    }
    pub fn add_file (&mut self, path: PathBuf, size: u64) {
        let path_length = path.components().count();
        if path_length == 0 {
            return
        }
        if path_length > 1 {
            let name = String::from(path.iter().next().expect("could not get next path element for folder").to_string_lossy());
            let path_entry = self.contents.entry(name.clone()).or_insert( 
                FileOrFolder::Folder(
                    Folder {
                        name,
                        contents: HashMap::new(),
                        size: 0,
                        num_descendants: 0,
                    }
                )
            );
            self.size += size;
            self.num_descendants += 1;
            match path_entry { // TODO: less ugly
                FileOrFolder::Folder(folder) => {
                    folder.add_file(path.iter().skip(1).collect(), size);
                },
                _ => {}
            };
        } else {
            let name = String::from(path.iter().next().expect("could not get next path element for file").to_string_lossy());
            self.size += size;
            self.num_descendants += 1;
            self.contents.insert(name.clone(),
                FileOrFolder::File(
                    File {
                        name,
                        size,
                    }
                )
            );
        }
    }
    pub fn path(&self, folder_names: &Vec<String>) -> Option<&FileOrFolder> {
        let mut folders_to_traverse: VecDeque<String> = VecDeque::from(folder_names.to_owned());
        let next_name = folders_to_traverse.pop_front().expect("could not find next path folder1");
        let next_in_path = &self.contents.get(&next_name)?;
        if folders_to_traverse.is_empty() {
            Some(next_in_path)
        } else if let FileOrFolder::Folder(next_folder) = next_in_path {
            next_folder.path(&Vec::from(folders_to_traverse)) // TODO: less allocations
        } else {
            Some(next_in_path)
        }
    }
    pub fn delete_path(&mut self, folder_names: &Vec<String>) {
        let mut folders_to_traverse: VecDeque<String> = VecDeque::from(folder_names.to_owned()); // TODO: better
        if folder_names.len() == 1 {
            let name = folder_names.last().expect("could not find last item in path");
            let removed_size = &self.contents.get(name).expect("could not find folder").size();
            let removed_descendents = match &self.contents.get(name).expect("could not find folder") {
                FileOrFolder::Folder(folder) => folder.num_descendants,
                FileOrFolder::File(_file) => 1,
            };
            self.size -= removed_size;
            self.num_descendants -= removed_descendents;
            &self.contents.remove(name);
        } else {
            let (removed_size, removed_descendents) = {
                let item_to_remove = self.path(&Vec::from(folders_to_traverse.clone())).expect("could not find item to delete");
                let removed_size = item_to_remove.size();
                let removed_descendents = match item_to_remove {
                    FileOrFolder::Folder(folder) => folder.num_descendants,
                    FileOrFolder::File(_file) => 1,
                };
                (removed_size, removed_descendents)
            };
            let next_name = folders_to_traverse.pop_front().expect("could not find next path folder");
            let next_item = &mut self.contents.get_mut(&next_name).expect("could not find folder in path"); // TODO: better
            match next_item {
                FileOrFolder::Folder(folder) => {
                    // TODO: look into a move performance way of doing this
                    self.size -= removed_size;
                    self.num_descendants -= removed_descendents;

                    folder.delete_path(&Vec::from(folders_to_traverse)); // TODO: better
                },
                FileOrFolder::File(_) => {
                    panic!("got a file in the middle of a path");
                }
            }
        }
    }
}
