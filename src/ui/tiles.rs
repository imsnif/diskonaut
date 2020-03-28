use tui::layout::Rect;

use crate::ui::TreeMap;
use crate::ui::FilePercentage;
use crate::ui::rectangle_grid::{RectWithText, RectFloat, MINIMUM_HEIGHT, MINIMUM_WIDTH};

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

pub struct Tiles {
    pub rectangles: Vec<RectWithText>,
    selected_index: Option<usize>,
    area: Option<Rect>,
    files: Vec<FilePercentage>,
}

impl Tiles {
    pub fn new () -> Self {
        Tiles {
            rectangles: vec![],
            files: vec![],
            selected_index: None,
            area: None,
        }
    }
    pub fn change_files(&mut self, file_percentages: Vec<FilePercentage>) {
        self.files = file_percentages;
        self.selected_index = Some(0);
        self.fill();
    }
    pub fn change_area(&mut self, area: &Rect) {
        match self.area {
            Some(current_area) => {
                if current_area != *area {
                    self.area = Some(area.clone());
                    self.selected_index = Some(0);
                    self.fill();
                }
            },
            None => {
                self.area = Some(area.clone());
                self.selected_index = Some(0);
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
    pub fn currently_selected (&self) -> Option<&RectWithText> {
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
                            
                            let existing_candidate: &RectWithText = self.rectangles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                            
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
                            
                            let existing_candidate: &RectWithText = self.rectangles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                            
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
                            
                            let existing_candidate: &RectWithText = self.rectangles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                            
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
                            
                            let existing_candidate: &RectWithText = self.rectangles.get(existing_candidate_index).expect(&format!("could not find existing candidate at index {}", existing_candidate_index));
                            
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
        }

    }
}
