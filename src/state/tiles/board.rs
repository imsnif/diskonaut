use ::tui::layout::Rect;

use crate::state::files::Folder;
use crate::state::tiles::{files_in_folder, FileMetadata, Tile, TreeMap};

pub struct Board {
    pub tiles: Vec<Tile>,
    pub unrenderable_tile_coordinates: Option<(u16, u16)>,
    pub selected_index: Option<usize>, // None means nothing is selected
    area: Rect,
    files: Vec<FileMetadata>,
}

impl Board {
    pub fn new(folder: &Folder) -> Self {
        Board {
            tiles: vec![],
            unrenderable_tile_coordinates: None,
            files: files_in_folder(folder),
            selected_index: None,
            area: Rect {
                x: 0,
                y: 0,
                width: 0,
                height: 0,
            },
        }
    }
    pub fn change_files(&mut self, folder: &Folder) {
        self.files = files_in_folder(folder);
        self.fill();
    }
    pub fn change_area(&mut self, area: &Rect) {
        if self.area != *area {
            self.area = area.clone();
            self.fill();
        }
    }
    fn fill(&mut self) {
        let mut tree_map = TreeMap::new(&self.area);
        tree_map.populate_tiles(self.files.iter().collect());
        self.tiles = tree_map.tiles;
        self.unrenderable_tile_coordinates = tree_map.unrenderable_tile_coordinates;
    }
    pub fn set_selected_index(&mut self, next_index: &usize) {
        self.selected_index = Some(*next_index);
    }
    pub fn has_selected_index(&self) -> bool {
        match self.selected_index {
            Some(_) => true,
            None => false,
        }
    }
    pub fn reset_selected_index(&mut self) {
        self.selected_index = None;
    }
    pub fn currently_selected(&self) -> Option<&Tile> {
        match &self.selected_index {
            Some(selected_index) => self.tiles.get(*selected_index),
            None => None,
        }
    }
    pub fn move_selected_right(&mut self) {
        match self.currently_selected() {
            Some(currently_selected) => {
                let next_index = {
                    let mut candidates_to_the_right: Vec<(usize, &Tile)> = self
                        .tiles
                        .iter()
                        .enumerate()
                        .filter(|(_, c)| {
                            c.is_directly_right_of(&currently_selected)
                                && c.horizontally_overlaps_with(&currently_selected)
                        })
                        .collect();
                    candidates_to_the_right.sort_by(|(_, a), (_, b)| {
                        let a_overlap = a.get_horizontal_overlap_with(&currently_selected);
                        let b_overlap = b.get_horizontal_overlap_with(&currently_selected);
                        b_overlap.cmp(&a_overlap)
                    });
                    // get the index of the tile with the most overlap with currently selected
                    candidates_to_the_right
                        .iter()
                        .map(|(index, _)| *index)
                        .nth(0)
                };
                match next_index {
                    Some(i) => self.set_selected_index(&i),
                    None => self.reset_selected_index(), // move off the edge of the screen resets selection
                }
            }
            None => self.set_selected_index(&0),
        }
    }
    pub fn move_selected_left(&mut self) {
        match self.currently_selected() {
            Some(currently_selected) => {
                let next_index = {
                    let mut candidates_to_the_left: Vec<(usize, &Tile)> = self
                        .tiles
                        .iter()
                        .enumerate()
                        .filter(|(_, c)| {
                            c.is_directly_left_of(&currently_selected)
                                && c.horizontally_overlaps_with(&currently_selected)
                        })
                        .collect();
                    candidates_to_the_left.sort_by(|(_, a), (_, b)| {
                        let a_overlap = a.get_horizontal_overlap_with(&currently_selected);
                        let b_overlap = b.get_horizontal_overlap_with(&currently_selected);
                        b_overlap.cmp(&a_overlap)
                    });
                    // get the index of the tile with the most overlap with currently selected
                    candidates_to_the_left
                        .iter()
                        .map(|(index, _)| *index)
                        .nth(0)
                };
                match next_index {
                    Some(i) => self.set_selected_index(&i),
                    None => self.reset_selected_index(), // move off the edge of the screen resets selection
                }
            }
            None => self.set_selected_index(&0),
        }
    }
    pub fn move_selected_down(&mut self) {
        match self.currently_selected() {
            Some(currently_selected) => {
                let next_index = {
                    let mut candidates_below: Vec<(usize, &Tile)> = self
                        .tiles
                        .iter()
                        .enumerate()
                        .filter(|(_, c)| {
                            c.is_directly_below(&currently_selected)
                                && c.vertically_overlaps_with(&currently_selected)
                        })
                        .collect();
                    candidates_below.sort_by(|(_, a), (_, b)| {
                        let a_overlap = a.get_vertical_overlap_with(&currently_selected);
                        let b_overlap = b.get_vertical_overlap_with(&currently_selected);
                        b_overlap.cmp(&a_overlap)
                    });
                    // get the index of the tile with the most overlap with currently selected
                    candidates_below.iter().map(|(index, _)| *index).nth(0)
                };
                match next_index {
                    Some(i) => self.set_selected_index(&i),
                    None => self.reset_selected_index(), // move off the edge of the screen resets selection
                }
            }
            None => self.set_selected_index(&0),
        }
    }
    pub fn move_selected_up(&mut self) {
        match self.currently_selected() {
            Some(currently_selected) => {
                let next_index = {
                    let mut candidates_below: Vec<(usize, &Tile)> = self
                        .tiles
                        .iter()
                        .enumerate()
                        .filter(|(_, c)| {
                            c.is_directly_above(&currently_selected)
                                && c.vertically_overlaps_with(&currently_selected)
                        })
                        .collect();
                    candidates_below.sort_by(|(_, a), (_, b)| {
                        let a_overlap = a.get_vertical_overlap_with(&currently_selected);
                        let b_overlap = b.get_vertical_overlap_with(&currently_selected);
                        b_overlap.cmp(&a_overlap)
                    });
                    // get the index of the tile with the most overlap with currently selected
                    candidates_below.iter().map(|(index, _)| *index).nth(0)
                };
                match next_index {
                    Some(i) => self.set_selected_index(&i),
                    None => self.reset_selected_index(), // move off the edge of the screen resets selection
                }
            }
            None => self.set_selected_index(&0),
        }
    }
}
