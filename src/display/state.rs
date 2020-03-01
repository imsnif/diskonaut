use tui::layout::Rect;

use std::collections::HashMap;
use crate::display::rectangle_grid::RectWithText;
use crate::filesystem::{FileOrFolder, Folder};
use ::std::fmt;

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

const minimum_height: u16 = 2; // TODO: consts
const minimum_width: u16 = 2;
const height_width_ratio: f64 = 2.5;

pub struct State {
    pub tiles: Vec<RectWithText>,
    pub base_folder: Option<Folder>,
    pub current_folder_names: Vec<String>,
    pub current_selected: String,
}

impl State {
    pub fn new() -> Self {
        Self {
            tiles: Vec::new(),
            base_folder: None,
            current_folder_names: Vec::new(),
            current_selected: String::new(), // TODO: better
        }
    }
    pub fn set_base_folder(&mut self, base_folder: Folder) {
        // self.current_folder_names.push(String::from(&base_folder.name));
        self.current_selected = String::from(&base_folder.name);
        self.base_folder = Some(base_folder);
    }
    pub fn set_tiles(&mut self, full_screen: Rect) {
        if let Some(base_folder) = &self.base_folder {

            let current_folder = base_folder.path(&self.current_folder_names);
            let file_percentages = calculate_percentages(current_folder.expect("could not find have current folder"));
            let total_space_area = full_screen.width as f64 * full_screen.height as f64;
            let mut free_spaces = vec![full_screen];
            let mut rectangles_to_render = vec![];

            let mut has_selected = false;
            let currently_selected = self.tiles.iter().find(|&t| t.selected == true);
            for file_percentage in &file_percentages {

                let total_file_area = total_space_area * file_percentage.percentage;
                let file_square_side_width = full_screen.width as f64 * (file_percentage.percentage * 2.0);
                let file_square_side_height = total_file_area / file_square_side_width;
                
                let mut candidate_index = 0;
                loop {
                    if candidate_index >= free_spaces.len() {
                        break;
                    }
                    if let Some(candidate) = free_spaces.get(candidate_index) {
                        let mut file_rect = None;
                        if candidate.width >= file_square_side_width as u16 &&
                            candidate.height >= file_square_side_height as u16
                            {
                            file_rect = Some(RectWithText {
                                rect: Rect {
                                    x: candidate.x,
                                    y: candidate.y,
                                    width: file_square_side_width as u16,
                                    height: file_square_side_height as u16,
                                },
                                text: file_percentage.file_name.clone(),
                                file_name: file_percentage.actual_file_name.clone(), // TODO: better
                                selected: false,
                            });
                        } else if candidate.height >= file_square_side_height as u16 &&
                            candidate.width >= (total_file_area / candidate.height as f64) as u16
                            {
                            let height = candidate.height;
                            let width = (total_file_area / height as f64) as u16;
                            file_rect = Some(RectWithText {
                                rect: Rect {
                                    x: candidate.x,
                                    y: candidate.y,
                                    width,
                                    height,
                                },
                                text: file_percentage.file_name.clone(),
                                file_name: file_percentage.actual_file_name.clone(), // TODO: better
                                selected: false,
                            });
                        } else if candidate.width >= file_square_side_width as u16 &&
                            candidate.height >= (total_file_area / candidate.width as f64) as u16
                            {
                            let width = candidate.width;
                            let height = (total_file_area / width as f64) as u16;
                            file_rect = Some(RectWithText {
                                rect: Rect {
                                    x: candidate.x,
                                    y: candidate.y,
                                    width,
                                    height,
                                },
                                text: file_percentage.file_name.clone(),
                                file_name: file_percentage.actual_file_name.clone(), // TODO: better
                                selected: false,
                            });
                        } else if candidate.height >= minimum_height &&
                            candidate.width >= ((total_file_area as u16 / minimum_height) as f64 * height_width_ratio) as u16
                        {
                            file_rect = Some(RectWithText {
                                rect: Rect {
                                    x: candidate.x,
                                    y: candidate.y,
                                    width: ((total_file_area as u16 / minimum_height) as f64 * height_width_ratio) as u16,
                                    height: minimum_height,
                                },
                                text: file_percentage.file_name.clone(),
                                file_name: file_percentage.actual_file_name.clone(), // TODO: better
                                selected: false,
                            });
                        } else if candidate.width >= minimum_width &&
                            candidate.height >= ((total_file_area as u16 / minimum_width) as f64 / height_width_ratio) as u16
                        {
                            file_rect = Some(RectWithText {
                                rect: Rect {
                                    x: candidate.x,
                                    y: candidate.y,
                                    width: minimum_width,
                                    height: ((total_file_area as u16 / minimum_width) as f64 / height_width_ratio) as u16,
                                },
                                text: file_percentage.file_name.clone(),
                                file_name: file_percentage.actual_file_name.clone(), // TODO: better
                                selected: false,
                            });
                        } else {
                        }
                        if let Some(mut rect_with_text) = file_rect {
                            // TODO: insert these at index?
                            if rect_with_text.rect.width > 1 && rect_with_text.rect.height > 1 {
                                let new_rectangle_right = Rect {
                                    x: candidate.x + rect_with_text.rect.width,
                                    width: (candidate.width as i16 - rect_with_text.rect.width as i16).abs() as u16,
                                    y: candidate.y,
                                    height: rect_with_text.rect.height
                                };
                                let mut new_rectangle_bottom = Rect {
                                    x: candidate.x,
                                    width: candidate.width,
                                    y: candidate.y + rect_with_text.rect.height,
                                    height: (candidate.height as i16 - rect_with_text.rect.height as i16).abs() as u16,
                                };
                                free_spaces.remove(candidate_index); // TODO: better - read api
                                if new_rectangle_right.width > 1 && new_rectangle_right.height > 1 && new_rectangle_right.x + new_rectangle_right.width <= full_screen.width {
                                    let free_rect_above_new = free_spaces.iter_mut()
                                        .find(|rect| rect.y + rect.height == new_rectangle_right.y && rect.x == new_rectangle_right.x && rect.x + rect.width == new_rectangle_right.x + new_rectangle_right.width);

                                    match free_rect_above_new {
                                        Some(free_rect_above_new) => {
                                            free_rect_above_new.height += new_rectangle_right.height;
                                        },
                                        None => {
                                            free_spaces.push(new_rectangle_right);
                                        }
                                    }
                                } else if new_rectangle_right.width == 1 {
                                    rect_with_text.rect.width += 1;
                                }
                                if new_rectangle_bottom.width > 1 && new_rectangle_bottom.height > 1 && new_rectangle_bottom.y + new_rectangle_bottom.height <= full_screen.height {
                                    free_spaces.push(new_rectangle_bottom);
                                } else if new_rectangle_bottom.height == 1 {
                                    rect_with_text.rect.height += 1;
                                    new_rectangle_bottom.x = rect_with_text.rect.x + rect_with_text.rect.width;
                                    new_rectangle_bottom.width -= rect_with_text.rect.width;

                                    let free_rect_above_new = free_spaces.iter_mut()
                                        .find(|rect| rect.y + rect.height == new_rectangle_bottom.y && rect.x == new_rectangle_bottom.x && rect.x + rect.width == new_rectangle_bottom.x + new_rectangle_bottom.width);

                                    match free_rect_above_new {
                                        Some(free_rect_above_new) => {
                                            free_rect_above_new.height += new_rectangle_bottom.height;
                                        },
                                        None => {
                                            if new_rectangle_bottom.width > 1 && new_rectangle_bottom.height > 1 && new_rectangle_bottom.y + new_rectangle_bottom.height <= full_screen.height {
                                                free_spaces.push(new_rectangle_bottom);
                                            } else {
                                                // TODO: ???
                                            }
                                        }
                                    }

                                }
                                match currently_selected {
                                    Some(currently_selected_rect) => {
                                        if currently_selected_rect.text == rect_with_text.text { 
                                            rect_with_text.selected = true;
                                            self.current_selected = String::from(&rect_with_text.file_name);
                                            rectangles_to_render.push(rect_with_text);
                                        } else {
                                            rect_with_text.selected = false;
                                            rectangles_to_render.push(rect_with_text);
                                        }
                                    },
                                    None => {
                                        if has_selected {
                                            rect_with_text.selected = false;
                                            rectangles_to_render.push(rect_with_text);
                                        } else {
                                            has_selected = true;
                                            rect_with_text.selected = true;
                                            self.current_selected = String::from(&rect_with_text.file_name);
                                            rectangles_to_render.push(rect_with_text);
                                        }
                                    }
                                };
                                break;
                            } else {
                                candidate_index += 1;
                                // println!("\rnot bigger than 1!!!111!!111 {:?}", rect_with_text);
                            }
                        } else {
                            candidate_index += 1;
                        }
                    }
                }
            }
            for free_rect in free_spaces {
                // rounding errors - TODO: find a better way to do this
                // TODO: throw if larger than 2
                let occupied_rect_left = rectangles_to_render.iter_mut()
                    .find(|rect_to_render| {
                        let occupied_rect = rect_to_render.rect;
                        occupied_rect.y == free_rect.y && occupied_rect.x + occupied_rect.width == free_rect.x
                    });
                match occupied_rect_left {
                    Some(occupied_rect_left) => {
                        occupied_rect_left.rect.width += free_rect.width
                    },
                    None => {
                        // TODO: ?? throw?
                    }
                }

            }
            self.tiles = rectangles_to_render
        }
    }
    pub fn clone_currently_selected(&self) -> RectWithText {
        self.tiles.iter().find(|t| t.selected == true).expect("could not find selected rect").clone()
    }
    pub fn move_selected_right (&mut self) {
        let currently_selected = self.clone_currently_selected();
        let next_to_the_right = {
            let found_next = self.tiles.iter().find(|t| { // TODO: find the rectangle with the most overlap, not just the first
                t.rect.x == currently_selected.rect.x + currently_selected.rect.width &&
                (
                    t.rect.y >= currently_selected.rect.y && t.rect.y < currently_selected.rect.y + currently_selected.rect.height ||
                    t.rect.y + t.rect.height >= currently_selected.rect.y && t.rect.y + t.rect.height < currently_selected.rect.y + currently_selected.rect.height ||
                    t.rect.y <= currently_selected.rect.y && t.rect.y + t.rect.height >= currently_selected.rect.y
                )
            });
            match found_next {
                Some(rect) => Some(rect.clone()),
                None => None
            }
        };
        match next_to_the_right {
            Some(rect) => {
                for tile in self.tiles.iter_mut() {
                    if tile.text == rect.text {
                        tile.selected = true;
                        self.current_selected = String::from(tile.file_name.clone());
                    } else if tile.text == currently_selected.text {
                        tile.selected = false;
                    }
                }
            },
            None => {}
        }
    }
    pub fn move_selected_left(&mut self) {
        let currently_selected = self.clone_currently_selected();
        let next_to_the_left = {
            let found_next = self.tiles.iter().find(|t| { // TODO: find the rectangle with the most overlap, not just the first
                t.rect.x + t.rect.width == currently_selected.rect.x &&
                (
                    t.rect.y >= currently_selected.rect.y && t.rect.y < currently_selected.rect.y + currently_selected.rect.height ||
                    t.rect.y + t.rect.height >= currently_selected.rect.y && t.rect.y + t.rect.height < currently_selected.rect.y + currently_selected.rect.height ||
                    t.rect.y <= currently_selected.rect.y && t.rect.y + t.rect.height >= currently_selected.rect.y
                )
            });
            match found_next {
                Some(rect) => Some(rect.clone()),
                None => None
            }
        };
        match next_to_the_left {
            Some(rect) => {
                for tile in self.tiles.iter_mut() {
                    if tile.text == rect.text {
                        tile.selected = true;
                        self.current_selected = String::from(tile.file_name.clone());
                    } else if tile.text == currently_selected.text {
                        tile.selected = false;
                    }
                }
            },
            None => {}
        }
    }
    pub fn move_selected_down(&mut self) {
        let currently_selected = self.clone_currently_selected();
        let next_down = {
            let found_next = self.tiles.iter().find(|t| { // TODO: find the rectangle with the most overlap, not just the first
                t.rect.y == currently_selected.rect.y + currently_selected.rect.height &&
                (
                    t.rect.x >= currently_selected.rect.x && t.rect.x < currently_selected.rect.x + currently_selected.rect.width ||
                    t.rect.x + t.rect.width >= currently_selected.rect.x && t.rect.x + t.rect.width < currently_selected.rect.x + currently_selected.rect.width ||
                    t.rect.x <= currently_selected.rect.x && t.rect.x + t.rect.width >= currently_selected.rect.x
                )
            });
            match found_next {
                Some(rect) => Some(rect.clone()),
                None => None
            }
        };
        match next_down {
            Some(rect) => {
                for tile in self.tiles.iter_mut() {
                    if tile.text == rect.text {
                        tile.selected = true;
                        self.current_selected = String::from(tile.file_name.clone());
                    } else if tile.text == currently_selected.text {
                        tile.selected = false;
                    }
                }
            },
            None => {}
        }
    }
    pub fn move_selected_up(&mut self) {
        let currently_selected = self.clone_currently_selected();
        let next_up = {
            let found_next = self.tiles.iter().find(|t| { // TODO: find the rectangle with the most overlap, not just the first
                t.rect.y + t.rect.height == currently_selected.rect.y &&
                (
                    t.rect.x >= currently_selected.rect.x && t.rect.x < currently_selected.rect.x + currently_selected.rect.width ||
                    t.rect.x + t.rect.width >= currently_selected.rect.x && t.rect.x + t.rect.width < currently_selected.rect.x + currently_selected.rect.width ||
                    t.rect.x <= currently_selected.rect.x && t.rect.x + t.rect.width >= currently_selected.rect.x
                )
            });
            match found_next {
                Some(rect) => Some(rect.clone()),
                None => None
            }
        };
        match next_up {
            Some(rect) => {
                for tile in self.tiles.iter_mut() {
                    if tile.text == rect.text {
                        tile.selected = true;
                        self.current_selected = String::from(tile.file_name.clone());
                    } else if tile.text == currently_selected.text {
                        tile.selected = false;
                    }
                }
            },
            None => {}
        }
    }
    pub fn enter_selected(&mut self) {
        if let Some(base_folder) = &self.base_folder {
            let mut path_to_selected = &mut self.current_folder_names.clone();
            path_to_selected.push(String::from(&self.current_selected));
            if let Some(potential_current_folder) = base_folder.path(&path_to_selected) {
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

// pub fn calculate_percentages (file_sizes: HashMap<String, u64>) -> Vec<FilePercentage> {
// pub fn calculate_percentages (folder: Folder) -> Vec<FilePercentage> {
pub fn calculate_percentages (folder: &Folder) -> Vec<FilePercentage> {
    let mut file_percentages = Vec::new();
    // let total_size = file_sizes.values().fold(0, |acc, size| acc + size);
    let total_size = folder.size();
    let mut small_files = FilePercentage {
        file_name: String::from("Small files"),
        actual_file_name: String::from("Small files"),
        percentage: 0.0,
    };
    for (name, file_or_folder) in &folder.contents {
        match file_or_folder {
            FileOrFolder::Folder(folder) => {
                let size = folder.size();
                let percentage = size as f64 / total_size as f64;
                // TODO: CONTINUE HERE (26/02)
                // record actual file name here as well, pass it on to rectwithtext so that we can
                // properly extract it and deal with it in the move/enter methods (right now they
                // take the whole string with the percent when trying to enter directories)
                let file_percentage = FilePercentage {
                    file_name: format!("{}/ {} ({:.0}%)", name, DisplaySize(size as f64),percentage * 100.0),
                    actual_file_name: String::from(name), // TODO: better
                    percentage,
                };
                if file_percentage.percentage <= 0.01 { // TODO: calculate this and not hard-coded
                    small_files.percentage += file_percentage.percentage;
                } else {
                    file_percentages.push(file_percentage);
                }
            },
            FileOrFolder::File(file) => {
                let size = file.size;
                let percentage = size as f64 / total_size as f64;
                let file_percentage = FilePercentage {
                    file_name: format!("{} {} ({:.0}%)", name, DisplaySize(size as f64),percentage * 100.0),
                    actual_file_name: String::from(name),
                    percentage,
                };
                if file_percentage.percentage <= 0.01 { // TODO: calculate this and not hard-coded
                    small_files.percentage += file_percentage.percentage;
                } else {
                    file_percentages.push(file_percentage);
                }
            }
        }
    }

    small_files.file_name = format!("<Small files> ({:.0}%)", small_files.percentage * 100.0);
    file_percentages.push(small_files);
    file_percentages.sort_by(|a, b| {
        if a.percentage == b.percentage {
            a.file_name.partial_cmp(&b.file_name).unwrap()
        } else {
            b.percentage.partial_cmp(&a.percentage).unwrap()
        }
    });
    file_percentages
}
