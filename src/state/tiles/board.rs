use tui::layout::Rect;
use std::ffi::OsString;

use crate::state::files::{FileOrFolder, Folder};
use crate::state::tiles::{TreeMap, RectFloat};
use crate::ui::rectangle_grid::{MINIMUM_HEIGHT, MINIMUM_WIDTH};

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

#[derive(Clone, Debug)]
pub struct Tile {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub name: OsString,
    pub size: u64,
    pub descendants: Option<u64>,
    pub percentage: f64,
    pub file_type: FileType,
}

impl Tile {
    pub fn new (rect: &RectFloat, file_metadata: &FileMetadata) -> Self {
        let rounded = rect.round(); // TODO: do not allocate
        Tile {
            x: rounded.x,
            y: rounded.y,
            width: rounded.width,
            height: rounded.height,
            name: file_metadata.name.clone(),
            size: file_metadata.size,
            descendants: file_metadata.descendants,
            percentage: file_metadata.percentage,
            file_type: file_metadata.file_type.clone(),
        }
    }
    pub fn is_right_of(&self, other: &Tile) -> bool {
        self.x >= other.x + other.width
    }

    pub fn is_left_of(&self, other: &Tile) -> bool {
        self.x + self.width <= other.x
    }

    pub fn is_below(&self, other: &Tile) -> bool {
       self.y >= other.y + other.height
    }

    pub fn is_above(&self, other: &Tile) -> bool {
       self.y + self.height <= other.y
    }

    pub fn horizontally_overlaps_with(&self, other: &Tile) -> bool {
        ( self.y >= other.y && self.y <= (other.y + other.height) ) ||
        ( (self.y + self.height) <= (other.y + other.height) && (self.y + self.height) > other.y) ||
        (self.y <= other.y && (self.y + self.height >= (other.y + other.height)) ) ||
        ( other.y <= self.y && (other.y + other.height >= (self.y + self.height)) )
    }

    pub fn vertically_overlaps_with(&self, other: &Tile) -> bool {
        ( self.x >= other.x && self.x <= (other.x + other.width) ) ||
        ( (self.x + self.width) <= (other.x + other.width) && (self.x + self.width) > other.x) ||
        ( self.x <= other.x && (self.x + self.width >= (other.x + other.width)) ) ||
        ( other.x <= self.x && (other.x + other.width >= (self.x + self.width)) )
    }

    pub fn get_vertical_overlap_with(&self, other: &Tile) -> u16 {
        if self.x < other.x {
            if self.x + self.width >= other.x + other.width {
                other.width
            } else {
                self.x + self.width - other.x
            }
        } else {
            if other.x + other.width >= self.x + self.width {
                self.width
            } else {
                other.x + other.width - self.x
            }
        }
    }

    pub fn get_horizontal_overlap_with(&self, other: &Tile) -> u16 {
        if self.y < other.y {
            if self.y + self.height >= other.y + other.height {
                other.height
            } else {
                self.y + self.height - other.y
            }
        } else {
            if other.y + other.height >= self.y + self.height {
                self.height
            } else {
                other.y + other.height - self.y
            }
        }
    }

    pub fn is_atleast_minimum_size(&self) -> bool {
        self.height >= MINIMUM_HEIGHT && self.width >= MINIMUM_WIDTH
    }

    pub fn is_aligned_left_with(&self, other: &Tile) -> bool {
        self.x == other.x
    }
    pub fn is_aligned_right_with(&self, other: &Tile) -> bool {
        self.x + self.width == other.x + other.width
    }

    pub fn is_aligned_top_with(&self, other: &Tile) -> bool {
        self.y == other.y
    }

    pub fn is_aligned_bottom_with(&self, other: &Tile) -> bool {
        self.y + self.height == other.y + other.height
    }
}

pub struct Board {
    pub tiles: Vec<Tile>,
    pub selected_index: Option<usize>, // None means nothing is selected
    area: Option<Rect>,
    files: Vec<FileMetadata>,
}

fn files_in_folder(folder: &Folder) -> Vec<FileMetadata> {
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
    files
}

impl Board {
    pub fn new (folder: &Folder) -> Self {
        Board {
            tiles: vec![],
            files: files_in_folder(folder),
            selected_index: None,
            area: None,
        }
    }
    pub fn change_files(&mut self, folder: &Folder) {
        self.files = files_in_folder(folder);
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

            tree_map.squarify(self.files.iter().collect(), vec![]); // TODO: do not clone
            self.tiles = tree_map.tiles;
        }
    }
    pub fn set_selected_index (&mut self, next_index: &usize) {
        self.selected_index = Some(*next_index);
    }
    pub fn has_selected_index (&self) -> bool {
        match self.selected_index {
            Some(_) => true,
            None => false
        }
    }
    pub fn reset_selected_index (&mut self) {
        self.selected_index = None;
    }
    pub fn currently_selected (&self) -> Option<&Tile> {
        match &self.selected_index {
            Some(selected_index) => self.tiles.get(*selected_index),
            None => None,
        }
    }
    pub fn move_selected_right (&mut self) {
        if let Some(selected_index) = self.selected_index {

            let currently_selected = self.tiles.get(selected_index).expect(&format!("could not find selected rectangle at index {}", selected_index));

            let mut next_rectangle_index = None;

            for (candidate_index, candidate) in self.tiles.iter().enumerate().filter(|(_, c)| {
                c.is_atleast_minimum_size() &&
                c.is_right_of(&currently_selected) &&
                c.horizontally_overlaps_with(&currently_selected)
            }) {
                match next_rectangle_index {
                    Some(existing_candidate_index) => {

                        let existing_candidate: &Tile = self.tiles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));

                        if existing_candidate.is_aligned_left_with(&candidate) {
                            let existing_candidate_overlap = existing_candidate.get_horizontal_overlap_with(&currently_selected);
                            let candidate_overlap = candidate.get_horizontal_overlap_with(&currently_selected);
                            if existing_candidate_overlap < candidate_overlap {
                                next_rectangle_index = Some(candidate_index);
                            }
                        } else {
                            if candidate.x < existing_candidate.x {
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
        } else if self.tiles.len() > 0 {
            self.set_selected_index(&0);
        }

    }
    pub fn move_selected_left (&mut self) {
        if let Some(selected_index) = self.selected_index {

            let currently_selected = self.tiles.get(selected_index).expect(&format!("could not find selected rectangle at index {}", selected_index));

            let mut next_rectangle_index = None;
            for (candidate_index, candidate) in self.tiles.iter().enumerate().filter(|(_, c)| {
                c.is_atleast_minimum_size() &&
                c.is_left_of(&currently_selected) &&
                c.horizontally_overlaps_with(&currently_selected)
            }) {
                match next_rectangle_index {
                    Some(existing_candidate_index) => {

                        let existing_candidate: &Tile = self.tiles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));

                        if existing_candidate.is_aligned_right_with(&candidate) {
                            let existing_candidate_overlap = existing_candidate.get_horizontal_overlap_with(&currently_selected);
                            let candidate_overlap = candidate.get_horizontal_overlap_with(&currently_selected);
                            if existing_candidate_overlap < candidate_overlap {
                                next_rectangle_index = Some(candidate_index);
                            }
                        } else {
                            if candidate.x + candidate.width > existing_candidate.x + existing_candidate.width {
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
        } else if self.tiles.len() > 0 {
            self.set_selected_index(&0);
        }
    }
    pub fn move_selected_down (&mut self) {
        if let Some(selected_index) = self.selected_index {
            let currently_selected = self.tiles.get(selected_index).expect(&format!("could not find selected rectangle at index {}", selected_index));
            let mut next_rectangle_index = None;
            for (candidate_index, candidate) in self.tiles.iter().enumerate().filter(|(_, c)| {
                c.is_atleast_minimum_size() &&
                c.is_below(&currently_selected) &&
                c.vertically_overlaps_with(&currently_selected)
            }) {
                match next_rectangle_index {
                    Some(existing_candidate_index) => {
                        let existing_candidate: &Tile = self.tiles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                        if existing_candidate.is_aligned_top_with(&candidate) {
                            let existing_candidate_overlap = existing_candidate.get_vertical_overlap_with(&currently_selected);
                            let candidate_overlap = candidate.get_vertical_overlap_with(&currently_selected);
                            if existing_candidate_overlap < candidate_overlap {
                                next_rectangle_index = Some(candidate_index);
                            }
                        } else {
                            if candidate.y < existing_candidate.y {
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
        } else if self.tiles.len() > 0 {
            self.set_selected_index(&0);
        }
    }
    pub fn move_selected_up (&mut self) {
        if let Some(selected_index) = self.selected_index {
            let currently_selected = self.tiles.get(selected_index).expect(&format!("could not find selected rectangle at index {}", selected_index));
            let mut next_rectangle_index = None;
            for (candidate_index, candidate) in self.tiles.iter().enumerate().filter(|(_, c)| {
                c.is_atleast_minimum_size() &&
                c.is_above(&currently_selected) &&
                c.vertically_overlaps_with(&currently_selected)
            }) {
                match next_rectangle_index {
                    Some(existing_candidate_index) => {
                        let existing_candidate: &Tile = self.tiles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                        if existing_candidate.is_aligned_bottom_with(&candidate) {
                            let existing_candidate_overlap = existing_candidate.get_vertical_overlap_with(&currently_selected);
                            let candidate_overlap = candidate.get_vertical_overlap_with(&currently_selected);
                            if existing_candidate_overlap < candidate_overlap {
                                next_rectangle_index = Some(candidate_index);
                            }
                        } else {
                            if candidate.y + candidate.height > existing_candidate.y + existing_candidate.height {
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
        } else if self.tiles.len() > 0 {
            self.set_selected_index(&0);
        }
    }
}
