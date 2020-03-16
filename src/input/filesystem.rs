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
    pub fn path(&self, folder_names: &Vec<String>) -> Option<&Folder> {
        let mut folders_to_traverse: VecDeque<String> = VecDeque::from(folder_names.to_owned());
        // folders_to_traverse.reverse();
        // let next_name = folders_to_traverse.pop().expect("got empty path");
        // let self_name = folders_to_traverse.pop_front().expect("got empty path");
        if folders_to_traverse.is_empty() {
            Some(&self)
            // self.contents.get(&next_name).expect("could not find path")
        } else {
            let next_name = folders_to_traverse.pop_front().expect("could not find next path folder");
            // &self.contents.get(next_name).expect("could not find path").path(folders_to_traverse)
//            println!("\r*****");
//            println!("\rfolders_to_traverse {:?}, next_name {:?}", folders_to_traverse, next_name);
//            println!("\rname {:?}", &self.name);
            match &self.contents.get(&next_name) {
                Some(_) => {},
                None => println!("\rcould not find next_name {:?}", next_name),
            };
            match &self.contents.get(&next_name).expect("could not find folder in path") {
                FileOrFolder::Folder(folder) => {
                    // folders_to_traverse.reverse(); // TODO: get rid of this
                    folder.path(&Vec::from(folders_to_traverse)) // TODO: less allocations
                },
                // FileOrFolder::File(file) => panic!("got a file in the middle of a path")
                FileOrFolder::File(_) => None
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
