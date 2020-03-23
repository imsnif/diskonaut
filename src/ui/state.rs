use tui::layout::Rect;

use crate::ui::rectangle_grid::{RectWithText, RectFloat};
use crate::input::{FileOrFolder, Folder};
use ::std::fmt;
use std::path::PathBuf;

pub struct DisplaySize(pub f64);

impl fmt::Display for DisplaySize{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 > 999_999_999.0 {
            write!(f, "{:.1}G", self.0 / 1_000_000_000.0)
        } else if self.0 > 999_999.0 {
            write!(f, "{:.1}M", self.0 / 1_000_000.0)
        } else if self.0 > 999.0 {
            write!(f, "{:.1}K", self.0 / 1000.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

#[derive(Debug, Clone)]
pub struct FilePercentage {
    pub file_name: String,
    pub percentage: f64,
    pub actual_file_name: String,
}

#[derive(Debug, Clone)]
pub struct RectSize {
    height: u16,
    width: u16,
    file_name: String,
    actual_file_name: String,
}

const HEIGHT_WIDTH_RATIO: f64 = 2.5;

pub struct State {
    pub tiles: Option<Tiles>,
    pub base_folder: Option<Folder>,
    pub path_in_filesystem: Option<String>,
    pub current_folder_names: Vec<String>,
    pub current_selected: String,
}

struct TreeMap {
    pub rectangles: Vec<RectWithText>,
    empty_space: RectFloat,
    total_size: f64,
    currently_selected_text: Option<String>, // TODO: not here
}
impl TreeMap {

    pub fn new (empty_space: RectFloat, currently_selected_text: Option<String>) -> Self {
        TreeMap {
            rectangles: vec![],
            total_size: (empty_space.height * empty_space.width) as f64,
            empty_space,
            currently_selected_text,
        }
    }
    fn layoutrow(&mut self, row: Vec<FilePercentage>) {
        let row_total = row.iter().fold(0.0, |acc, file_percentage| {
            let size = file_percentage.percentage * self.total_size;
            acc + size
        });
        if self.empty_space.width <= self.empty_space.height * HEIGHT_WIDTH_RATIO {
            let mut x = self.empty_space.x;
            let mut row_height = 0.0;
            for file_percentage in row {
                let size = file_percentage.percentage * self.total_size;
                let width = (size / row_total) * self.empty_space.width as f64;
                let height = size / width;
                let selected = if let Some(currently_selected_text) = &self.currently_selected_text {
                    currently_selected_text == &file_percentage.file_name
                } else {
                    self.currently_selected_text = Some(file_percentage.file_name.clone());
                    true
                };
                let rect_with_text = RectWithText {
                    rect: RectFloat {x, y: self.empty_space.y, width: width , height: height },
                    text: file_percentage.file_name.clone(),
                    file_name: file_percentage.actual_file_name.clone(), // TODO: better
                    selected,
                };
                x += rect_with_text.rect.width;
                self.rectangles.push(rect_with_text);
                if height > row_height {
                    row_height = height; // TODO: check if this changes in iterations
                }
            }
            self.empty_space.height -= row_height;
            self.empty_space.y += row_height;
        } else {
          let mut y = self.empty_space.y;
          let mut row_width = 0.0;
          for file_percentage in row {
            let size = file_percentage.percentage * self.total_size;
            let height = (size / row_total) * self.empty_space.height as f64;
            let width = size / height;

            let selected = if let Some(currently_selected_text) = &self.currently_selected_text {
                currently_selected_text == &file_percentage.file_name
            } else {
                self.currently_selected_text = Some(file_percentage.file_name.clone());
                true
            };
            let mut rect_with_text = RectWithText {
                rect: RectFloat { x: self.empty_space.x, y, width: width, height: height },
                text: file_percentage.file_name.clone(),
                file_name: file_percentage.actual_file_name.clone(), // TODO: better
                selected,
            };
            y += rect_with_text.rect.height;
            if row_width > width {
                rect_with_text.rect.width = row_width // TODO: better
            }
            self.rectangles.push(rect_with_text);
            if width > row_width {
                row_width = width; // TODO: check if this changes in iterations
            }
          }
          self.empty_space.width -= row_width; // TODO: check if this changes in iterations
          self.empty_space.x += row_width;
        }
    }
    
    fn worst (&self, row: Vec<FilePercentage>, length: f64) -> f64 {
        let sum = row.iter().fold(0.0, |accum, file_percentage| {
            let size = file_percentage.percentage * self.total_size;
            accum + size
        });
        let mut worst_aspect_ratio = 0.0;
        for val in row.iter() {
            let size = val.percentage * self.total_size;
            let first_side = (size / sum) * length;
            let second_side = size / first_side;
            if first_side >= 5.0 && second_side >= 5.0 {
                let val_aspect_ratio = if first_side < second_side {
                    first_side / second_side
                } else {
                    second_side / first_side
                };
                if worst_aspect_ratio == 0.0 {
                    worst_aspect_ratio = val_aspect_ratio;
                } else if val_aspect_ratio < worst_aspect_ratio {
                    worst_aspect_ratio = val_aspect_ratio;
                }
            } else {
                return 0.0
            }
        }
        worst_aspect_ratio
    }
    
    fn squarify (&mut self, mut children: Vec<FilePercentage>, row: Vec<FilePercentage>) {
        let length = if self.empty_space.height * HEIGHT_WIDTH_RATIO < self.empty_space.width {
            self.empty_space.height * 2.5
        } else {
            self.empty_space.width * 0.6
        };

        if children.len() == 0 && !row.is_empty() { // TODO: better
            self.layoutrow(row);
        } else if children.len() == 0 {
            return;
        } else {
            let mut row_with_first_child = row.clone();
            row_with_first_child.push(children[0].clone());
            let mut nums = vec![];
            for per in row.iter() {
                nums.push(per.percentage * self.total_size);
            }

            let current_row_worst_ratio = self.worst(row.clone(), length);
            let row_with_child_worst_ratio = self.worst(row_with_first_child.clone(), length);

            if current_row_worst_ratio != 0.0 && row_with_child_worst_ratio == 0.0 {
                self.layoutrow(row);
                self.squarify(children, vec![]);
            } else if row.len() == 1 || current_row_worst_ratio <= row_with_child_worst_ratio || current_row_worst_ratio == 0.0 {
                children.remove(0);
                self.squarify(children, row_with_first_child.clone());
            } else {
                self.layoutrow(row);
                self.squarify(children, vec![]);
            }
        }
    }
}

pub struct Tiles {
    pub rectangles: Vec<RectWithText>,
    selected_index: usize,
    area: Rect,
}

impl Tiles {
    pub fn new (area: &Rect, file_percentages: Vec<FilePercentage>) -> Self {

        let empty_space = RectFloat { x: area.x as f64, y: area.y as f64, height: area.height as f64, width: area.width as f64 };
        let mut tree_map = TreeMap::new(empty_space, None);
        
        tree_map.squarify(file_percentages, vec![]);
        let mut rectangles = tree_map.rectangles;
        let selected_index = 0;
        if let Some(rect) = rectangles.get_mut(selected_index) {
            rect.selected = true;
        }
        Tiles {
            rectangles,
            selected_index,
            area: area.clone(),
        }
    }
    pub fn set_selected_index (&mut self, next_index: &usize) {
        {
            let mut existing_selected = self.rectangles.get_mut(self.selected_index).expect(&format!("could not find selected rect at index {}", self.selected_index));
            existing_selected.selected = false;
        }
        {
            let mut next_selected = self.rectangles.get_mut(*next_index).expect(&format!("could not find selected rect at index {}", next_index));
            next_selected.selected = true;
        }
        self.selected_index = *next_index;
    }
    pub fn currently_selected (&self) -> Option<&RectWithText> {
        self.rectangles.get(self.selected_index)
    }
    pub fn move_selected_right (&mut self) {
        let currently_selected = self.rectangles.get(self.selected_index).expect(&format!("could not find selected rectangle at index {}", self.selected_index));
        
        let mut next_rectangle_index = None;
        for (candidate_index, candidate) in self.rectangles.iter().enumerate() {
            if candidate.rect.x >= currently_selected.rect.x + currently_selected.rect.width && (
                ( candidate.rect.y >= currently_selected.rect.y && candidate.rect.y <= (currently_selected.rect.y + currently_selected.rect.height) ) ||
                ( (candidate.rect.y + candidate.rect.height) <= (currently_selected.rect.y + currently_selected.rect.height) && (candidate.rect.y + candidate.rect.height) > currently_selected.rect.y) ||
                (candidate.rect.y <= currently_selected.rect.y && (candidate.rect.y + candidate.rect.height >= (currently_selected.rect.y + currently_selected.rect.height)) ) ||
                ( currently_selected.rect.y <= candidate.rect.y && (currently_selected.rect.y + currently_selected.rect.height >= (candidate.rect.y + candidate.rect.height)) )
            ) {

                match next_rectangle_index {
                    Some(existing_candidate_index) => {
                        
                        let existing_candidate: &RectWithText = self.rectangles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                        // let existing_candidate: &RectWithText = self.rectangles.get(existing_candidate_index).unwrap();
                        
                        if existing_candidate.rect.x.round() == candidate.rect.x.round() {
                            let existing_candidate_overlap = if existing_candidate.rect.y < currently_selected.rect.y {
                                if existing_candidate.rect.y + existing_candidate.rect.height >= currently_selected.rect.y + currently_selected.rect.height {
                                    currently_selected.rect.height
                                } else {
                                    existing_candidate.rect.y + existing_candidate.rect.height - currently_selected.rect.y
                                } 
                            } else {
                                if currently_selected.rect.y + currently_selected.rect.height >= existing_candidate.rect.y + existing_candidate.rect.height {
                                    existing_candidate.rect.height
                                } else {
                                    currently_selected.rect.y + currently_selected.rect.height - existing_candidate.rect.y
                                } 
                            };
                            let candidate_overlap = if candidate.rect.y < currently_selected.rect.y {
                                if candidate.rect.y + candidate.rect.height >= currently_selected.rect.y + currently_selected.rect.height {
                                    currently_selected.rect.height
                                } else {
                                    candidate.rect.y + candidate.rect.height - currently_selected.rect.y
                                } 
                            } else {
                                if currently_selected.rect.y + currently_selected.rect.height >= candidate.rect.y + candidate.rect.height {
                                    candidate.rect.height
                                } else {
                                    currently_selected.rect.y + currently_selected.rect.height - candidate.rect.y
                                } 
                            };
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
            {
                let mut existing_selected = self.rectangles.get_mut(self.selected_index).expect(&format!("could not find selected rect at index {}", self.selected_index));
                existing_selected.selected = false;
            }
            {
                let mut next_selected = self.rectangles.get_mut(next_index).expect(&format!("could not find selected rect at index {}", self.selected_index));
                next_selected.selected = true;
            }
            self.selected_index = next_index;
        }

    }
    pub fn move_selected_left (&mut self) {
        let currently_selected = self.rectangles.get(self.selected_index).expect(&format!("could not find selected rectangle at index {}", self.selected_index));
        
        let mut next_rectangle_index = None;
        for (candidate_index, candidate) in self.rectangles.iter().enumerate() {

            if candidate.rect.x + candidate.rect.width <= currently_selected.rect.x && (
                ( candidate.rect.y >= currently_selected.rect.y && candidate.rect.y <= (currently_selected.rect.y + currently_selected.rect.height) ) ||
                ( (candidate.rect.y + candidate.rect.height) <= (currently_selected.rect.y + currently_selected.rect.height) && (candidate.rect.y + candidate.rect.height) > currently_selected.rect.y) ||
                ( candidate.rect.y <= currently_selected.rect.y && (candidate.rect.y + candidate.rect.height >= (currently_selected.rect.y + currently_selected.rect.height)) ) ||
                ( currently_selected.rect.y <= candidate.rect.y && (currently_selected.rect.y + currently_selected.rect.height >= (candidate.rect.y + candidate.rect.height)) )
            ) {

                match next_rectangle_index {
                    Some(existing_candidate_index) => {
                        
                        let existing_candidate: &RectWithText = self.rectangles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                        
                        if (existing_candidate.rect.x + existing_candidate.rect.width).round() == (candidate.rect.x + candidate.rect.width).round() {
                            let existing_candidate_overlap = if existing_candidate.rect.y < currently_selected.rect.y {
                                if existing_candidate.rect.y + existing_candidate.rect.height >= currently_selected.rect.y + currently_selected.rect.height {
                                    currently_selected.rect.height
                                } else {
                                    existing_candidate.rect.y + existing_candidate.rect.height - currently_selected.rect.y
                                } 
                            } else {
                                if currently_selected.rect.y + currently_selected.rect.height >= existing_candidate.rect.y + existing_candidate.rect.height {
                                    existing_candidate.rect.height
                                } else {
                                    currently_selected.rect.y + currently_selected.rect.height - existing_candidate.rect.y
                                } 
                            };
                            let candidate_overlap = if candidate.rect.y < currently_selected.rect.y {
                                if candidate.rect.y + candidate.rect.height >= currently_selected.rect.y + currently_selected.rect.height {
                                    currently_selected.rect.height
                                } else {
                                    candidate.rect.y + candidate.rect.height - currently_selected.rect.y
                                } 
                            } else {
                                if currently_selected.rect.y + currently_selected.rect.height >= candidate.rect.y + candidate.rect.height {
                                    candidate.rect.height
                                } else {
                                    currently_selected.rect.y + currently_selected.rect.height - candidate.rect.y
                                } 
                            };
                            if existing_candidate_overlap < candidate_overlap {
                                next_rectangle_index = Some(candidate_index);
                            }
                        } else {
                            if candidate.rect.x > existing_candidate.rect.x {
                                next_rectangle_index = Some(candidate_index);
                            }
                        }
                    },
                    None => next_rectangle_index = Some(candidate_index),
                };
            }
        }
        if let Some(next_index) = next_rectangle_index {
            {
                let mut existing_selected = self.rectangles.get_mut(self.selected_index).expect(&format!("could not find selected rect at index {}", self.selected_index));
                existing_selected.selected = false;
            }
            {
                let mut next_selected = self.rectangles.get_mut(next_index).expect(&format!("could not find selected rect at index {}", self.selected_index));
                next_selected.selected = true;
            }
            self.selected_index = next_index;
        }
    }
    pub fn move_selected_down (&mut self) {

        let currently_selected = self.rectangles.get(self.selected_index).expect(&format!("could not find selected rectangle at index {}", self.selected_index));
        
        let mut next_rectangle_index = None;
        for (candidate_index, candidate) in self.rectangles.iter().enumerate() {

            if candidate.rect.y >= currently_selected.rect.y + currently_selected.rect.height && (
                ( candidate.rect.x >= currently_selected.rect.x && candidate.rect.x <= (currently_selected.rect.x + currently_selected.rect.width) ) ||
                ( (candidate.rect.x + candidate.rect.width) <= (currently_selected.rect.x + currently_selected.rect.width) && (candidate.rect.x + candidate.rect.width) > currently_selected.rect.x) ||
                ( candidate.rect.x <= currently_selected.rect.x && (candidate.rect.x + candidate.rect.width >= (currently_selected.rect.x + currently_selected.rect.width)) ) ||
                ( currently_selected.rect.x <= candidate.rect.x && (currently_selected.rect.x + currently_selected.rect.width >= (candidate.rect.x + candidate.rect.width)) )
            ) {

                match next_rectangle_index {
                    Some(existing_candidate_index) => {
                        
                        let existing_candidate: &RectWithText = self.rectangles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                        
                        if existing_candidate.rect.y.round() == candidate.rect.y.round() {
                            let existing_candidate_overlap = if existing_candidate.rect.x < currently_selected.rect.x {
                                if existing_candidate.rect.x + existing_candidate.rect.width >= currently_selected.rect.x + currently_selected.rect.width {
                                    currently_selected.rect.width
                                } else {
                                    existing_candidate.rect.x + existing_candidate.rect.width - currently_selected.rect.x
                                } 
                            } else {
                                if currently_selected.rect.x + currently_selected.rect.width >= existing_candidate.rect.x + existing_candidate.rect.width {
                                    existing_candidate.rect.width
                                } else {
                                    currently_selected.rect.x + currently_selected.rect.width - existing_candidate.rect.x
                                } 
                            };
                            let candidate_overlap = if candidate.rect.x < currently_selected.rect.x {
                                if candidate.rect.x + candidate.rect.width >= currently_selected.rect.x + currently_selected.rect.width {
                                    currently_selected.rect.width
                                } else {
                                    candidate.rect.x + candidate.rect.width - currently_selected.rect.x
                                } 
                            } else {
                                if currently_selected.rect.x + currently_selected.rect.width >= candidate.rect.x + candidate.rect.width {
                                    candidate.rect.width
                                } else {
                                    currently_selected.rect.x + currently_selected.rect.width - candidate.rect.x
                                } 
                            };
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
            {
                let mut existing_selected = self.rectangles.get_mut(self.selected_index).expect(&format!("could not find selected rect at index {}", self.selected_index));
                existing_selected.selected = false;
            }
            {
                let mut next_selected = self.rectangles.get_mut(next_index).expect(&format!("could not find selected rect at index {}", self.selected_index));
                next_selected.selected = true;
            }
            self.selected_index = next_index;
        }
    }
    pub fn move_selected_up (&mut self) {

        let currently_selected = self.rectangles.get(self.selected_index).expect(&format!("could not find selected rectangle at index {}", self.selected_index));
        
        let mut next_rectangle_index = None;
        for (candidate_index, candidate) in self.rectangles.iter().enumerate() {

            if candidate.rect.y + candidate.rect.height <= currently_selected.rect.y && (
                ( candidate.rect.x >= currently_selected.rect.x && candidate.rect.x <= (currently_selected.rect.x + currently_selected.rect.width) ) ||
                ( (candidate.rect.x + candidate.rect.width) <= (currently_selected.rect.x + currently_selected.rect.width) && (candidate.rect.x + candidate.rect.width) > currently_selected.rect.x) ||
                ( candidate.rect.x <= currently_selected.rect.x && (candidate.rect.x + candidate.rect.width >= (currently_selected.rect.x + currently_selected.rect.width)) ) ||
                ( currently_selected.rect.x <= candidate.rect.x && (currently_selected.rect.x + currently_selected.rect.width >= (candidate.rect.x + candidate.rect.width)) )
                
            ) {

                match next_rectangle_index {
                    Some(existing_candidate_index) => {
                        
                        let existing_candidate: &RectWithText = self.rectangles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                        
                        if (existing_candidate.rect.y + existing_candidate.rect.height).round() == (candidate.rect.y + candidate.rect.height).round() {

                        let existing_candidate_overlap = if existing_candidate.rect.x < currently_selected.rect.x {
                            if existing_candidate.rect.x + existing_candidate.rect.width >= currently_selected.rect.x + currently_selected.rect.width {
                                currently_selected.rect.width
                            } else {
                                existing_candidate.rect.x + existing_candidate.rect.width - currently_selected.rect.x
                            } 
                        } else {
                            if currently_selected.rect.x + currently_selected.rect.width >= existing_candidate.rect.x + existing_candidate.rect.width {
                                existing_candidate.rect.width
                            } else {
                                currently_selected.rect.x + currently_selected.rect.width - existing_candidate.rect.x
                            } 
                        };
                        let candidate_overlap = if candidate.rect.x < currently_selected.rect.x {
                            if candidate.rect.x + candidate.rect.width >= currently_selected.rect.x + currently_selected.rect.width {
                                currently_selected.rect.width
                            } else {
                                candidate.rect.x + candidate.rect.width - currently_selected.rect.x
                            } 
                        } else {
                            if currently_selected.rect.x + currently_selected.rect.width >= candidate.rect.x + candidate.rect.width {
                                candidate.rect.width
                            } else {
                                currently_selected.rect.x + currently_selected.rect.width - candidate.rect.x
                            } 
                        };

                            if existing_candidate_overlap < candidate_overlap {
                                next_rectangle_index = Some(candidate_index);
                            }
                        } else {
                            if candidate.rect.y > existing_candidate.rect.y {
                                next_rectangle_index = Some(candidate_index);
                            }
                        }
                    },
                    None => next_rectangle_index = Some(candidate_index),
                };
            }
        }
        if let Some(next_index) = next_rectangle_index {
            {
                let mut existing_selected = self.rectangles.get_mut(self.selected_index).expect(&format!("could not find selected rect at index {}", self.selected_index));
                existing_selected.selected = false;
            }
            {
                let mut next_selected = self.rectangles.get_mut(next_index).expect(&format!("could not find selected rect at index {}", self.selected_index));
                next_selected.selected = true;
            }
            self.selected_index = next_index;
        }

    }
}

impl State {
    pub fn new() -> Self {
        Self {
            tiles: None,
            base_folder: None,
            path_in_filesystem: None,
            current_folder_names: Vec::new(),
            current_selected: String::new(), // TODO: better
        }
    }
    pub fn set_base_folder(&mut self, base_folder: Folder, path_in_filesystem: String) {
        self.current_selected = String::from(&base_folder.name);
        self.base_folder = Some(base_folder);
        self.path_in_filesystem = Some(path_in_filesystem);
    }
    pub fn get_current_path(&self) -> Option<PathBuf> {
        if let Some(path_in_filesystem) = &self.path_in_filesystem {
            let mut full_path = PathBuf::from(&path_in_filesystem);
            for folder in &self.current_folder_names {
                full_path.push(&folder)
            }
            return Some(full_path);
        }
        None
    }
    pub fn set_tiles(&mut self, full_screen: Rect) {
        if let Some(base_folder) = &self.base_folder {
            if let Some(current_tiles) = &self.tiles {
                if current_tiles.area != full_screen {
                    let current_folder = base_folder.path(&self.current_folder_names);
                    let file_percentages = calculate_percentages(current_folder.expect("could not find have current folder"));
                    self.tiles = Some(Tiles::new(&full_screen, file_percentages))
                } else {
                    let current_selected_index = &self.tiles.as_ref().expect("could not find tiles").selected_index;
                    let current_folder = base_folder.path(&self.current_folder_names);
                    let file_percentages = calculate_percentages(current_folder.expect("could not find have current folder"));
                    let mut new_tiles = Tiles::new(&full_screen, file_percentages);
                    new_tiles.set_selected_index(current_selected_index);
                    self.tiles = Some(new_tiles);
                }
            } else {
                let current_folder = base_folder.path(&self.current_folder_names);
                let file_percentages = calculate_percentages(current_folder.expect("could not find have current folder"));
                self.tiles = Some(Tiles::new(&full_screen, file_percentages))
            }
        }
    }
    pub fn move_selected_right (&mut self) {
        if let Some(tiles) = &mut self.tiles {
            tiles.move_selected_right();
        }
    }
    pub fn move_selected_left(&mut self) {
        if let Some(tiles) = &mut self.tiles {
            tiles.move_selected_left();
        }
    }
    pub fn move_selected_down(&mut self) {
        if let Some(tiles) = &mut self.tiles {
            tiles.move_selected_down();
        }
    }
    pub fn move_selected_up(&mut self) {
        if let Some(tiles) = &mut self.tiles {
            tiles.move_selected_up();
        }
    }
    pub fn enter_selected(&mut self) {
        if let Some(base_folder) = &self.base_folder {
            if let Some(file_percentages) = &self.tiles.as_ref().expect("could not find tiles").currently_selected() {
                let path_to_selected = &mut self.current_folder_names.clone();
                path_to_selected.push(String::from(&file_percentages.file_name));
                if &self.current_selected == "Small files" {
                    return;
                }
                if let Some(_) = base_folder.path(&path_to_selected) {
                    // there is a folder at this path!
                    self.current_folder_names.push(String::from(&file_percentages.file_name));
                    if let Some(tiles) = &mut self.tiles {
                        tiles.set_selected_index(&0);
                    }
                }
            }
        }
    }
    pub fn go_up(&mut self) {
        self.current_folder_names.pop();
        if let Some(tiles) = &mut self.tiles {
            tiles.set_selected_index(&0);
        }
    }
}

pub fn calculate_percentages (folder: &Folder) -> Vec<FilePercentage> {
    let mut file_percentages = Vec::new();
    let total_size = folder.size();
    for (name, file_or_folder) in &folder.contents {
        match file_or_folder {
            FileOrFolder::Folder(folder) => {
                let size = folder.size();
                let percentage = size as f64 / total_size as f64;
                let file_percentage = FilePercentage {
                    file_name: format!("{}/ {} ({:.0}%)", name, DisplaySize(size as f64),percentage * 100.0),
                    actual_file_name: String::from(name), // TODO: better
                    percentage,
                };
                file_percentages.push(file_percentage);
            },
            FileOrFolder::File(file) => {
                let size = file.size;
                let percentage = size as f64 / total_size as f64;
                let file_percentage = FilePercentage {
                    file_name: format!("{} {} ({:.0}%)", name, DisplaySize(size as f64),percentage * 100.0),
                    actual_file_name: String::from(name),
                    percentage,
                };
                file_percentages.push(file_percentage);
            }
        }
    }

    file_percentages.sort_by(|a, b| {
        if a.percentage == b.percentage {
            a.file_name.partial_cmp(&b.file_name).unwrap()
        } else {
            b.percentage.partial_cmp(&a.percentage).unwrap()
        }
    });
    file_percentages
}
