use ::std::ffi::OsString;
use ::std::path::PathBuf;

use crate::state::FileType;

#[derive(Clone)]
pub struct FileToDelete {
    pub path_in_filesystem: PathBuf,
    pub path_to_file: Vec<OsString>,
    pub file_type: FileType,
    pub num_descendants: Option<u64>,
    pub size: u64,
}

impl FileToDelete {
    pub fn full_path(&self) -> PathBuf {
        let mut full_path = self.path_in_filesystem.clone();
        for component in &self.path_to_file {
            full_path.push(component);
        }
        full_path
    }
}
