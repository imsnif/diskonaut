use tui::layout::Rect;

use crate::display::rectangle_grid::{RectWithText, RectFloat};
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
    pub tiles: Vec<RectWithText>,
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
            if first_side >= 2.0 && second_side >= 2.0 {
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
            if row.len() == 1 || current_row_worst_ratio <= row_with_child_worst_ratio || current_row_worst_ratio == 0.0 {
                children.remove(0);
                self.squarify(children, row_with_first_child.clone());
            } else {
                self.layoutrow(row);
                self.squarify(children, vec![]);
            }
        }
    }
}

impl State {
    pub fn new() -> Self {
        Self {
            tiles: Vec::new(),
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

            let current_folder = base_folder.path(&self.current_folder_names);
            let file_percentages = calculate_percentages(current_folder.expect("could not find have current folder"));
            let total_space_area = full_screen.width as f64 * full_screen.height as f64;

            let currently_selected = self.tiles.iter().find(|&t| t.selected == true);
            if let Some(currently_selected) = currently_selected {
                self.current_selected = String::from(&currently_selected.file_name);
            }

            let empty_space = RectFloat { x: full_screen.x as f64, y: full_screen.y as f64, height: full_screen.height as f64, width: full_screen.width as f64 };
            
            let mut tree_map = if let Some(currently_selected) = currently_selected {
                TreeMap::new(empty_space, Some(String::from(&currently_selected.text))) // TODO: better
            } else {
                TreeMap::new(empty_space, None)
            };
            let mut files_to_render = vec![];
            let mut too_small = vec![];
            for file in file_percentages {
                let size = file.percentage * total_space_area;
                if size >= 30.0 { // TODO: this number should be reconsidered
                    files_to_render.push(file);
                } else {
                    too_small.push(file);
                }
            }
            let mut small_files = FilePercentage {
                file_name: String::from("Small files"),
                actual_file_name: String::from("Small files"),
                percentage: 0.0,
            };
            for file in too_small {
                small_files.percentage += file.percentage;
            }
            small_files.file_name = format!("Small files ({:.0?}%)", small_files.percentage * 100.0);
            files_to_render.push(small_files);
            files_to_render.sort_by(|a, b| {
                if a.percentage == b.percentage {
                    a.file_name.partial_cmp(&b.file_name).unwrap()
                } else {
                    b.percentage.partial_cmp(&a.percentage).unwrap()
                }
            });

            tree_map.squarify(files_to_render, vec![]);
            self.current_selected = String::from(&tree_map.rectangles.iter().find(|&t| t.selected == true).expect("no selected tile").file_name);

            self.tiles = tree_map.rectangles;
        }
    }
    pub fn clone_currently_selected(&self) -> RectWithText {
        self.tiles.iter().find(|t| t.selected == true).expect("could not find selected rect").clone()
    }
    pub fn move_selected_right (&mut self) {
        let currently_selected = self.clone_currently_selected();
        let found_next = {
            let mut candidates: Vec<&RectWithText> = self.tiles.iter()
                .filter(|t|
                    t.rect.x >= currently_selected.rect.x + currently_selected.rect.width &&
                    (
                        ( t.rect.y >= currently_selected.rect.y && t.rect.y <= (currently_selected.rect.y + currently_selected.rect.height) ) ||
                        ( (t.rect.y + t.rect.height) <= (currently_selected.rect.y + currently_selected.rect.height) && (t.rect.y + t.rect.height) > currently_selected.rect.y) ||
                        ( t.rect.y <= currently_selected.rect.y && (t.rect.y + t.rect.height >= (currently_selected.rect.y + currently_selected.rect.height)) ) ||
                        ( currently_selected.rect.y <= t.rect.y && (currently_selected.rect.y + currently_selected.rect.height >= (t.rect.y + t.rect.height)) )
                    )
                ).collect();

            candidates.sort_by(|a, b| {
                if a.rect.x.round() == b.rect.x.round() {
                    let a_overlap = if a.rect.y < currently_selected.rect.y {
                        if a.rect.y + a.rect.height >= currently_selected.rect.y + currently_selected.rect.height {
                            currently_selected.rect.height
                        } else {
                            a.rect.y + a.rect.height - currently_selected.rect.y
                        } 
                    } else {
                        if currently_selected.rect.y + currently_selected.rect.height >= a.rect.y + a.rect.height {
                            a.rect.height
                        } else {
                            currently_selected.rect.y + currently_selected.rect.height - a.rect.y
                        } 
                    };
                    let b_overlap = if b.rect.y < currently_selected.rect.y {
                        if b.rect.y + b.rect.height >= currently_selected.rect.y + currently_selected.rect.height {
                            currently_selected.rect.height
                        } else {
                            b.rect.y + b.rect.height - currently_selected.rect.y
                        } 
                    } else {
                        if currently_selected.rect.y + currently_selected.rect.height >= b.rect.y + b.rect.height {
                            b.rect.height
                        } else {
                            currently_selected.rect.y + currently_selected.rect.height - b.rect.y
                        } 
                    };
                    b_overlap.partial_cmp(&a_overlap).expect("could not compare rects")
                } else {
                    a.rect.x.partial_cmp(&b.rect.x).expect("could not compare rects")
                }
            });
            match candidates.get(0) {
                Some(next_rect) => Some(String::from(&next_rect.text)),
                None => None,
            }
        };

        if let Some(next_rect_text) = found_next {
            for tile in self.tiles.iter_mut() {
                if tile.text == next_rect_text {
                    tile.selected = true;
                    self.current_selected = String::from(tile.file_name.clone());
                } else if tile.text == currently_selected.text {
                    tile.selected = false;
                }
            }
        }
    }
    pub fn move_selected_left(&mut self) {
        let currently_selected = self.clone_currently_selected();
        let found_next = {
            let mut candidates: Vec<&RectWithText> = self.tiles.iter()
                .filter(|t|
                    t.rect.x + t.rect.width <= currently_selected.rect.x &&
                    (
                        ( t.rect.y >= currently_selected.rect.y && t.rect.y <= (currently_selected.rect.y + currently_selected.rect.height) ) ||
                        ( (t.rect.y + t.rect.height) <= (currently_selected.rect.y + currently_selected.rect.height) && (t.rect.y + t.rect.height) > currently_selected.rect.y) ||
                        ( t.rect.y <= currently_selected.rect.y && (t.rect.y + t.rect.height >= (currently_selected.rect.y + currently_selected.rect.height)) ) ||
                        ( currently_selected.rect.y <= t.rect.y && (currently_selected.rect.y + currently_selected.rect.height >= (t.rect.y + t.rect.height)) )
                    )
                ).collect();

            candidates.sort_by(|a, b| {
                if a.rect.x.round() == b.rect.x.round() {
                    let a_overlap = if a.rect.y < currently_selected.rect.y {
                        if a.rect.y + a.rect.height >= currently_selected.rect.y + currently_selected.rect.height {
                            currently_selected.rect.height
                        } else {
                            a.rect.y + a.rect.height - currently_selected.rect.y
                        } 
                    } else {
                        if currently_selected.rect.y + currently_selected.rect.height >= a.rect.y + a.rect.height {
                            a.rect.height
                        } else {
                            currently_selected.rect.y + currently_selected.rect.height - a.rect.y
                        } 
                    };
                    let b_overlap = if b.rect.y < currently_selected.rect.y {
                        if b.rect.y + b.rect.height >= currently_selected.rect.y + currently_selected.rect.height {
                            currently_selected.rect.height
                        } else {
                            b.rect.y + b.rect.height - currently_selected.rect.y
                        } 
                    } else {
                        if currently_selected.rect.y + currently_selected.rect.height >= b.rect.y + b.rect.height {
                            b.rect.height
                        } else {
                            currently_selected.rect.y + currently_selected.rect.height - b.rect.y
                        } 
                    };
                    b_overlap.partial_cmp(&a_overlap).expect("could not compare rects")
                } else {
                    (b.rect.x + b.rect.width).partial_cmp(&(a.rect.x + a.rect.width)).expect("could not compare rects")
                }
            });
            match candidates.get(0) {
                Some(next_rect) => Some(String::from(&next_rect.text)),
                None => None,
            }
        };

        if let Some(next_rect_text) = found_next {
            for tile in self.tiles.iter_mut() {
                if tile.text == next_rect_text {
                    tile.selected = true;
                    self.current_selected = String::from(tile.file_name.clone());
                } else if tile.text == currently_selected.text {
                    tile.selected = false;
                }
            }
        }
    }
    pub fn move_selected_down(&mut self) {
        let currently_selected = self.clone_currently_selected();
        let found_next = {
            let mut candidates: Vec<&RectWithText> = self.tiles.iter()
                .filter(|t|
                    t.rect.y >= currently_selected.rect.y + currently_selected.rect.height &&
                    (
                        ( t.rect.x >= currently_selected.rect.x && t.rect.x <= (currently_selected.rect.x + currently_selected.rect.width) ) ||
                        ( (t.rect.x + t.rect.width) <= (currently_selected.rect.x + currently_selected.rect.width) && (t.rect.x + t.rect.width) > currently_selected.rect.x) ||
                        ( t.rect.x <= currently_selected.rect.x && (t.rect.x + t.rect.width >= (currently_selected.rect.x + currently_selected.rect.width)) ) ||
                        ( currently_selected.rect.x <= t.rect.x && (currently_selected.rect.x + currently_selected.rect.width >= (t.rect.x + t.rect.width)) )
                    )
                ).collect();

            candidates.sort_by(|a, b| {
                if a.rect.y.round() == b.rect.y.round() {
                    let a_overlap = if a.rect.x < currently_selected.rect.x {
                        if a.rect.x + a.rect.width >= currently_selected.rect.x + currently_selected.rect.width {
                            currently_selected.rect.width
                        } else {
                            a.rect.x + a.rect.width - currently_selected.rect.x
                        } 
                    } else {
                        if currently_selected.rect.x + currently_selected.rect.width >= a.rect.x + a.rect.width {
                            a.rect.width
                        } else {
                            currently_selected.rect.x + currently_selected.rect.width - a.rect.x
                        } 
                    };
                    let b_overlap = if b.rect.x < currently_selected.rect.x {
                        if b.rect.x + b.rect.width >= currently_selected.rect.x + currently_selected.rect.width {
                            currently_selected.rect.width
                        } else {
                            b.rect.x + b.rect.width - currently_selected.rect.x
                        } 
                    } else {
                        if currently_selected.rect.x + currently_selected.rect.width >= b.rect.x + b.rect.width {
                            b.rect.width
                        } else {
                            currently_selected.rect.x + currently_selected.rect.width - b.rect.x
                        } 
                    };
                    b_overlap.partial_cmp(&a_overlap).expect("could not compare rects")
                } else {
                    a.rect.y.partial_cmp(&b.rect.y).expect("could not compare rects")
                }
            });
            match candidates.get(0) {
                Some(next_rect) => Some(String::from(&next_rect.text)),
                None => None,
            }
        };

        if let Some(next_rect_text) = found_next {
            for tile in self.tiles.iter_mut() {
                if tile.text == next_rect_text {
                    tile.selected = true;
                    self.current_selected = String::from(tile.file_name.clone());
                } else if tile.text == currently_selected.text {
                    tile.selected = false;
                }
            }
        }
    }
    pub fn move_selected_up(&mut self) {
        let currently_selected = self.clone_currently_selected();
        let found_next = {
            let mut candidates: Vec<&RectWithText> = self.tiles.iter()
                .filter(|t|
                    t.rect.y + t.rect.height <= currently_selected.rect.y &&
                    (
                        ( t.rect.x >= currently_selected.rect.x && t.rect.x <= (currently_selected.rect.x + currently_selected.rect.width) ) ||
                        ( (t.rect.x + t.rect.width) <= (currently_selected.rect.x + currently_selected.rect.width) && (t.rect.x + t.rect.width) > currently_selected.rect.x) ||
                        ( t.rect.x <= currently_selected.rect.x && (t.rect.x + t.rect.width >= (currently_selected.rect.x + currently_selected.rect.width)) ) ||
                        ( currently_selected.rect.x <= t.rect.x && (currently_selected.rect.x + currently_selected.rect.width >= (t.rect.x + t.rect.width)) )
                    )
                ).collect();
            candidates.sort_by(|a, b| {
                if (a.rect.y + a.rect.height).round() == (b.rect.y + b.rect.height).round() {
                    let a_overlap = if a.rect.x < currently_selected.rect.x {
                        if a.rect.x + a.rect.width >= currently_selected.rect.x + currently_selected.rect.width {
                            currently_selected.rect.width
                        } else {
                            a.rect.x + a.rect.width - currently_selected.rect.x
                        } 
                    } else {
                        if currently_selected.rect.x + currently_selected.rect.width >= a.rect.x + a.rect.width {
                            a.rect.width
                        } else {
                            currently_selected.rect.x + currently_selected.rect.width - a.rect.x
                        } 
                    };
                    let b_overlap = if b.rect.x < currently_selected.rect.x {
                        if b.rect.x + b.rect.width >= currently_selected.rect.x + currently_selected.rect.width {
                            currently_selected.rect.width
                        } else {
                            b.rect.x + b.rect.width - currently_selected.rect.x
                        } 
                    } else {
                        if currently_selected.rect.x + currently_selected.rect.width >= b.rect.x + b.rect.width {
                            b.rect.width
                        } else {
                            currently_selected.rect.x + currently_selected.rect.width - b.rect.x
                        } 
                    };
                    b_overlap.partial_cmp(&a_overlap).expect("could not compare rects")
                } else {
                    (b.rect.y + b.rect.height).partial_cmp(&(a.rect.y + a.rect.height)).expect("could not compare rects")
                }
            });
            match candidates.get(0) {
                Some(next_rect) => Some(String::from(&next_rect.text)),
                None => None,
            }
        };

        if let Some(next_rect_text) = found_next {
            for tile in self.tiles.iter_mut() {
                if tile.text == next_rect_text {
                    tile.selected = true;
                    self.current_selected = String::from(tile.file_name.clone());
                } else if tile.text == currently_selected.text {
                    tile.selected = false;
                }
            }
        }
    }
    pub fn enter_selected(&mut self) {
        if let Some(base_folder) = &self.base_folder {
            let path_to_selected = &mut self.current_folder_names.clone();
            path_to_selected.push(String::from(&self.current_selected));
            if &self.current_selected == "Small files" {
                return;
            }
            if let Some(_) = base_folder.path(&path_to_selected) {
                // there is a folder at this path!
                self.current_folder_names.push(String::from(&self.current_selected));
                self.current_selected = String::from("I am a bug waiting to happen fix me");
                if let Some(selected_tile) = self.tiles.iter_mut().find(|t| t.selected == true) {
                    selected_tile.selected = false;
                }
            }
        }
    }
    pub fn go_up(&mut self) {
        self.current_folder_names.pop();
        if let Some(selected_tile) = self.tiles.iter_mut().find(|t| t.selected == true) {
            selected_tile.selected = false;
        }
        // self.current_selected = String::from("I am a bug waiting to happen fix me");
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
