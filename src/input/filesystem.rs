#[allow(dead_code)]

use std::collections::{HashMap, VecDeque};
use std::fs;
use std::os::unix::fs::MetadataExt; // TODO: support other OSs

use walkdir::WalkDir;
use std::path::PathBuf;

#[derive(Debug)]
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
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    pub size: u64,
}

#[derive(Debug)]
pub struct Folder {
    pub name: String,
    pub contents: HashMap<String, FileOrFolder>
}

impl Folder {
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
                    }
                )
            );
            match path_entry { // TODO: less ugly
                FileOrFolder::Folder(folder) => folder.add_folder(path.iter().skip(1).collect()),
                _ => {}
            };
        } else {
            let name = String::from(path.iter().next().expect("could not get next path element for file").to_string_lossy());
            self.contents.insert(name.clone(),
                FileOrFolder::Folder(
                    Folder {
                        name,
                        contents: HashMap::new(),
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
                    }
                )
            );
            match path_entry { // TODO: less ugly
                FileOrFolder::Folder(folder) => folder.add_file(path.iter().skip(1).collect(), size),
                _ => {}
            };
        } else {
            let name = String::from(path.iter().next().expect("could not get next path element for file").to_string_lossy());
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
    pub fn size (&self) -> u64 {
        let mut total_size = 0;
        for (_, descendant) in &self.contents {
            match descendant {
                FileOrFolder::Folder(folder) => {
                    total_size += folder.size();
                },
                FileOrFolder::File(file) => {
                    total_size += file.size;
                }
            };
        }
        total_size
    }
    pub fn num_descendants (&self) -> u64 {
        let mut total_descendants = 0;
        for (_, descendant) in &self.contents {
            total_descendants += 1;
            if let FileOrFolder::Folder(folder) = descendant {
                total_descendants += folder.num_descendants();
            }
        }
        total_descendants
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
            &self.contents.remove(folder_names.last().expect("could not find last item in path"));
        } else {
            let next_name = folders_to_traverse.pop_front().expect("could not find next path folder");
            let next_item = &mut self.contents.get_mut(&next_name).expect("could not find folder in path"); // TODO: better
            match next_item {
                FileOrFolder::Folder(folder) => {
                    folder.delete_path(&Vec::from(folders_to_traverse)); // TODO: better
                },
                FileOrFolder::File(_) => {
                    panic!("got a file in the middle of a path");
                }
            }
        }
    }
}

pub fn scan_folder (path: PathBuf) -> Folder {
    
    let base_folder_name = path.iter().last().expect("could not get path base name").to_string_lossy();
    let mut base_folder = Folder {
        name: String::from(base_folder_name),
        contents: HashMap::new(),
    };

    let path_length = path.components().count();
    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        let entry_path = entry.path().clone();
        match fs::metadata(entry_path.display().to_string()) {
            Ok(file_metadata) => {
                let mut relative_path = PathBuf::new();
                for dir in entry_path.components().skip(path_length) {
                    relative_path.push(dir);
                }
                if file_metadata.is_dir() {
                    base_folder.add_folder(relative_path);
                } else {
                    let size = file_metadata.blocks() * 512;
                    base_folder.add_file(relative_path, size);
                }
            },
            Err(_e) => {
                ();
                // println!("\rerror opening {:?} {:?}", entry, e); // TODO: look into these
            }
        }
    }
    base_folder
}
