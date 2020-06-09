use tui::layout::Rect;
use std::ffi::OsString;

use crate::state::files::{FileOrFolder, Folder};
use crate::state::tiles::{TreeMap, RectFloat};

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct FileRect {
    pub rect: RectFloat,
    pub file_metadata: FileMetadata,
    pub selected: bool,
}

pub struct Board {
    pub rectangles: Vec<FileRect>,
    selected_index: Option<usize>, // None means nothing is selected
    area: Option<Rect>,
    files: Vec<FileMetadata>,
}

impl Board {
    pub fn new (folder: &Folder) -> Self {
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
                b.percentage.partial_cmp(&a.percentage).expect("could not compare percentage")
            }
        });

        Board {
            rectangles: vec![],
            files,
            selected_index: None,
            area: None,
        }
    }
    pub fn change_files(&mut self, folder: &Folder) {
        // TODO: better - this is basically a copy of the new function above
        // maybe just hold a reference to this folder and calculate on fill?
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
                b.percentage.partial_cmp(&a.percentage).expect("could not compare percentage")
            }
        });
        self.files = files;
        self.fill();
    }
    pub fn change_area(&mut self, area: &Rect) {
        match self.area {
            Some(current_area) => {
                if current_area != *area {
                    self.area = Some(area.clone());
                    self.selected_index = None;
                    self.fill();
                }
            },
            None => {
                self.area = Some(area.clone());
                self.selected_index = None;
                self.fill();
            }
        }
    }
    fn fill(&mut self) {

        if let Some(area) = self.area {

            let empty_space = RectFloat { x: area.x as f64, y: area.y as f64, height: area.height as f64, width: area.width as f64 };
            let mut tree_map = TreeMap::new(empty_space);
            
            tree_map.squarify(self.files.clone(), vec![]); // TODO: do not clone
            let mut rectangles = tree_map.rectangles;
            if let Some(selected_index) = self.selected_index {
                let mut selected_rect = rectangles.get_mut(selected_index).expect(&format!("could not find selected rect at index {}", selected_index));
                selected_rect.selected = true;
            }
            self.rectangles = rectangles;
        }
    }
    pub fn set_selected_index (&mut self, next_index: &usize) {
        if let Some(selected_index) = self.selected_index {
            let mut existing_selected = self.rectangles.get_mut(selected_index).expect(&format!("could not find selected rect at index {}", selected_index));
            existing_selected.selected = false;
        }
        let mut next_selected = self.rectangles.get_mut(*next_index).expect(&format!("could not find selected rect at index {}", next_index));
        next_selected.selected = true;
        self.selected_index = Some(*next_index);
    }
    pub fn has_selected_index (&self) -> bool {
        match self.selected_index {
            Some(_) => true,
            None => false
        }
    }
    pub fn reset_selected_index (&mut self) {
        if let Some(selected_index) = self.selected_index {
            let mut existing_selected = self.rectangles.get_mut(selected_index).expect(&format!("could not find selected rect at index {}", selected_index));
            existing_selected.selected = false;
        }
        self.selected_index = None;
    }
    pub fn currently_selected (&self) -> Option<&FileRect> {
        match &self.selected_index {
            Some(selected_index) => self.rectangles.get(*selected_index),
            None => None,
        }
    }
    pub fn move_selected_right (&mut self) {
        if let Some(selected_index) = self.selected_index {

            let currently_selected = self.rectangles.get(selected_index).expect(&format!("could not find selected rectangle at index {}", selected_index));
            
            let mut next_rectangle_index = None;

            for (candidate_index, candidate) in self.rectangles.iter().enumerate().filter(|(_, c)| {
                c.rect.is_atleast_minimum_size() &&
                c.rect.is_right_of(&currently_selected.rect) &&
                c.rect.horizontally_overlaps_with(&currently_selected.rect)
            }) {
                match next_rectangle_index {
                    Some(existing_candidate_index) => {
                        
                        let existing_candidate: &FileRect = self.rectangles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                        
                        if existing_candidate.rect.is_aligned_left_with(&candidate.rect) {
                            let existing_candidate_overlap = existing_candidate.rect.get_horizontal_overlap_with(&currently_selected.rect);
                            let candidate_overlap = candidate.rect.get_horizontal_overlap_with(&currently_selected.rect);
                            if existing_candidate_overlap < candidate_overlap {
                                next_rectangle_index = Some(candidate_index);
                            }
                        } else {
                            if candidate.rect.x < existing_candidate.rect.x {
                                next_rectangle_index = Some(candidate_index);
                            }
                        }
                    },
                    None => next_rectangle_index = Some(candidate_index),
                };
            }
            if let Some(next_index) = next_rectangle_index {
                self.set_selected_index(&next_index);
            }
        } else if self.rectangles.len() > 0 {
            self.set_selected_index(&0);
        }

    }
    pub fn move_selected_left (&mut self) {
        if let Some(selected_index) = self.selected_index {

            let currently_selected = self.rectangles.get(selected_index).expect(&format!("could not find selected rectangle at index {}", selected_index));
        
            let mut next_rectangle_index = None;
            for (candidate_index, candidate) in self.rectangles.iter().enumerate().filter(|(_, c)| {
                c.rect.is_atleast_minimum_size() &&
                c.rect.is_left_of(&currently_selected.rect) &&
                c.rect.horizontally_overlaps_with(&currently_selected.rect)
            }) {
                match next_rectangle_index {
                    Some(existing_candidate_index) => {
                        
                        let existing_candidate: &FileRect = self.rectangles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                        
                        if existing_candidate.rect.is_aligned_right_with(&candidate.rect) {
                            let existing_candidate_overlap = existing_candidate.rect.get_horizontal_overlap_with(&currently_selected.rect);
                            let candidate_overlap = candidate.rect.get_horizontal_overlap_with(&currently_selected.rect);
                            if existing_candidate_overlap < candidate_overlap {
                                next_rectangle_index = Some(candidate_index);
                            }
                        } else {
                            if candidate.rect.x + candidate.rect.width > existing_candidate.rect.x + existing_candidate.rect.width {
                                next_rectangle_index = Some(candidate_index);
                            }
                        }
                    },
                    None => next_rectangle_index = Some(candidate_index),
                };
            }
            if let Some(next_index) = next_rectangle_index {
                self.set_selected_index(&next_index);
            }
        } else if self.rectangles.len() > 0 {
            self.set_selected_index(&0);
        }
    }
    pub fn move_selected_down (&mut self) {
        if let Some(selected_index) = self.selected_index {
            let currently_selected = self.rectangles.get(selected_index).expect(&format!("could not find selected rectangle at index {}", selected_index));
            let mut next_rectangle_index = None;
            for (candidate_index, candidate) in self.rectangles.iter().enumerate().filter(|(_, c)| {
                c.rect.is_atleast_minimum_size() &&
                c.rect.is_below(&currently_selected.rect) &&
                c.rect.vertically_overlaps_with(&currently_selected.rect)
            }) {
                match next_rectangle_index {
                    Some(existing_candidate_index) => {
                        let existing_candidate: &FileRect = self.rectangles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                        if existing_candidate.rect.is_aligned_top_with(&candidate.rect) {
                            let existing_candidate_overlap = existing_candidate.rect.get_vertical_overlap_with(&currently_selected.rect);
                            let candidate_overlap = candidate.rect.get_vertical_overlap_with(&currently_selected.rect);
                            if existing_candidate_overlap < candidate_overlap {
                                next_rectangle_index = Some(candidate_index);
                            }
                        } else {
                            if candidate.rect.y < existing_candidate.rect.y {
                                next_rectangle_index = Some(candidate_index);
                            }
                        }
                    },
                    None => next_rectangle_index = Some(candidate_index),
                };
            }
            if let Some(next_index) = next_rectangle_index {
                self.set_selected_index(&next_index);
            }
        } else if self.rectangles.len() > 0 {
            self.set_selected_index(&0);
        }
    }
    pub fn move_selected_up (&mut self) {
        if let Some(selected_index) = self.selected_index {
            let currently_selected = self.rectangles.get(selected_index).expect(&format!("could not find selected rectangle at index {}", selected_index));
            let mut next_rectangle_index = None;
            for (candidate_index, candidate) in self.rectangles.iter().enumerate().filter(|(_, c)| {
                c.rect.is_atleast_minimum_size() &&
                c.rect.is_above(&currently_selected.rect) &&
                c.rect.vertically_overlaps_with(&currently_selected.rect)
            }) {
                match next_rectangle_index {
                    Some(existing_candidate_index) => {
                        let existing_candidate: &FileRect = self.rectangles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                        if existing_candidate.rect.is_aligned_bottom_with(&candidate.rect) {
                            let existing_candidate_overlap = existing_candidate.rect.get_vertical_overlap_with(&currently_selected.rect);
                            let candidate_overlap = candidate.rect.get_vertical_overlap_with(&currently_selected.rect);
                            if existing_candidate_overlap < candidate_overlap {
                                next_rectangle_index = Some(candidate_index);
                            }
                        } else {
                            if candidate.rect.y + candidate.rect.height > existing_candidate.rect.y + existing_candidate.rect.height {
                                next_rectangle_index = Some(candidate_index);
                            }
                        }
                    },
                    None => next_rectangle_index = Some(candidate_index),
                };
            }
            if let Some(next_index) = next_rectangle_index {
                self.set_selected_index(&next_index);
            }
        } else if self.rectangles.len() > 0 {
            self.set_selected_index(&0);
        }
    }
}
