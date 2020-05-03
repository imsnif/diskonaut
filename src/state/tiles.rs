use tui::layout::Rect;

use crate::state::files::{FileOrFolder, Folder};
use crate::ui::TreeMap;
use crate::ui::rectangle_grid::{MINIMUM_HEIGHT, MINIMUM_WIDTH};

fn first_is_right_of_second(first: &RectFloat, second: &RectFloat) -> bool {
    first.x >= second.x + second.width
}

fn first_is_left_of_second (first: &RectFloat, second: &RectFloat) -> bool {
    first.x + first.width <= second.x
}

fn first_is_below_second(first: &RectFloat, second: &RectFloat) -> bool {
   first.y >= second.y + second.height
}

fn first_is_above_second(first: &RectFloat, second: &RectFloat) -> bool {
   first.y + first.height <= second.y
}

fn horizontally_overlap(first: &RectFloat, second: &RectFloat) -> bool {
    ( first.y >= second.y && first.y <= (second.y + second.height) ) ||
    ( (first.y + first.height) <= (second.y + second.height) && (first.y + first.height) > second.y) ||
    (first.y <= second.y && (first.y + first.height >= (second.y + second.height)) ) ||
    ( second.y <= first.y && (second.y + second.height >= (first.y + first.height)) )
}

fn vertically_overlap(first: &RectFloat, second: &RectFloat) -> bool {
    ( first.x >= second.x && first.x <= (second.x + second.width) ) ||
    ( (first.x + first.width) <= (second.x + second.width) && (first.x + first.width) > second.x) ||
    ( first.x <= second.x && (first.x + first.width >= (second.x + second.width)) ) ||
    ( second.x <= first.x && (second.x + second.width >= (first.x + first.width)) )
} 

fn get_vertical_overlap (first: &RectFloat, second: &RectFloat) -> f64 {
    if first.x < second.x {
        if first.x + first.width >= second.x + second.width {
            second.width
        } else {
            first.x + first.width - second.x
        } 
    } else {
        if second.x + second.width >= first.x + first.width {
            first.width
        } else {
            second.x + second.width - first.x
        } 
    }
}

fn get_horizontal_overlap (first: &RectFloat, second: &RectFloat) -> f64 {
    if first.y < second.y {
        if first.y + first.height >= second.y + second.height {
            second.height
        } else {
            first.y + first.height - second.y
        } 
    } else {
        if second.y + second.height >= first.y + first.height {
            first.height
        } else {
            second.y + second.height - first.y
        } 
    }
}

fn is_atleast_minimum_size(rect: &RectFloat) -> bool {
    rect.height > MINIMUM_HEIGHT as f64 && rect.width > MINIMUM_WIDTH as f64
}

fn rects_are_aligned_left (first: &RectFloat, second: &RectFloat) -> bool {
    first.x.round() == second.x.round()
}
fn rects_are_aligned_right (first: &RectFloat, second: &RectFloat) -> bool {
    (first.x + first.width).round() == (second.x + second.width).round()
}

fn rects_are_aligned_top (first: &RectFloat, second: &RectFloat) -> bool {
    first.y.round() == second.y.round()
}

fn rects_are_aligned_bottom(first: &RectFloat, second: &RectFloat) -> bool {
    (first.y + first.height).round() == (second.y + second.height).round()
}

#[derive(Clone, Debug)]
pub enum FileType {
    File,
    Folder,
}

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub name: String,
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

#[derive(Clone, Debug)]
pub struct RectFloat {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

pub struct Tiles {
    pub rectangles: Vec<FileRect>,
    selected_index: Option<usize>, // None means nothing is selected
    area: Option<Rect>,
    files: Vec<FileMetadata>,
}

impl Tiles {
    pub fn new (folder: &Folder) -> Self {
        let mut files = Vec::new();
        let total_size = folder.size;
        for (name, file_or_folder) in &folder.contents {
            files.push({
                let size = file_or_folder.size();
                let name = String::from(name);
                let (descendants, file_type) = match file_or_folder {
                    FileOrFolder::Folder(folder) => (Some(folder.num_descendants), FileType::Folder),
                    FileOrFolder::File(_file) => (None, FileType::File),
                };
                let percentage = size as f64 / total_size as f64;
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




        Tiles {
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
                let name = String::from(name);
                let (descendants, file_type) = match file_or_folder {
                    FileOrFolder::Folder(folder) => (Some(folder.num_descendants), FileType::Folder),
                    FileOrFolder::File(_file) => (None, FileType::File),
                };
                let percentage = size as f64 / total_size as f64;
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

            for (candidate_index, candidate) in self.rectangles.iter().enumerate() {
                if is_atleast_minimum_size(&candidate.rect) &&
                   first_is_right_of_second(&candidate.rect, &currently_selected.rect) &&
                   horizontally_overlap(&candidate.rect, &currently_selected.rect)
                {

                    match next_rectangle_index {
                        Some(existing_candidate_index) => {
                            
                            let existing_candidate: &FileRect = self.rectangles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                            
                            if rects_are_aligned_left(&existing_candidate.rect, &candidate.rect) {
                                let existing_candidate_overlap = get_horizontal_overlap(&existing_candidate.rect, &currently_selected.rect);
                                let candidate_overlap = get_horizontal_overlap(&candidate.rect, &currently_selected.rect);
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
            for (candidate_index, candidate) in self.rectangles.iter().enumerate() {

                if is_atleast_minimum_size(&candidate.rect) &&
                    first_is_left_of_second(&candidate.rect, &currently_selected.rect) &&
                    horizontally_overlap(&candidate.rect, &currently_selected.rect)
                {

                    match next_rectangle_index {
                        Some(existing_candidate_index) => {
                            
                            let existing_candidate: &FileRect = self.rectangles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                            
                            if rects_are_aligned_right(&existing_candidate.rect, &candidate.rect) {
                                let existing_candidate_overlap = get_horizontal_overlap(&existing_candidate.rect, &currently_selected.rect);
                                let candidate_overlap = get_horizontal_overlap(&candidate.rect, &currently_selected.rect);
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
            for (candidate_index, candidate) in self.rectangles.iter().enumerate() {

                if is_atleast_minimum_size(&candidate.rect) &&
                    first_is_below_second(&candidate.rect, &currently_selected.rect) &&
                    vertically_overlap(&candidate.rect, &currently_selected.rect)
                {

                    match next_rectangle_index {
                        Some(existing_candidate_index) => {
                            
                            let existing_candidate: &FileRect = self.rectangles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                            
                            if rects_are_aligned_top(&existing_candidate.rect, &candidate.rect) {

                                let existing_candidate_overlap = get_vertical_overlap(&existing_candidate.rect, &currently_selected.rect);
                                let candidate_overlap = get_vertical_overlap(&candidate.rect, &currently_selected.rect);

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
            for (candidate_index, candidate) in self.rectangles.iter().enumerate() {

                if is_atleast_minimum_size(&candidate.rect) &&
                    first_is_above_second(&candidate.rect, &currently_selected.rect) &&
                    vertically_overlap(&candidate.rect, &currently_selected.rect)
                {

                    match next_rectangle_index {
                        Some(existing_candidate_index) => {
                            
                            let existing_candidate: &FileRect = self.rectangles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                            
                            if rects_are_aligned_bottom(&existing_candidate.rect, &candidate.rect) {

                                let existing_candidate_overlap = get_vertical_overlap(&existing_candidate.rect, &currently_selected.rect);
                                let candidate_overlap = get_vertical_overlap(&candidate.rect, &currently_selected.rect);

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
            }
            if let Some(next_index) = next_rectangle_index {
                self.set_selected_index(&next_index);
            }
        } else if self.rectangles.len() > 0 {
            self.set_selected_index(&0);
        }
    }
}
