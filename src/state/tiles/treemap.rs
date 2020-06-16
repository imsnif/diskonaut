use ::tui::layout::Rect;

use crate::state::tiles::{FileMetadata, RectFloat, Tile};

const HEIGHT_WIDTH_RATIO: f64 = 2.5;
const MINIMUM_HEIGHT: u16 = 3;
const MINIMUM_WIDTH: u16 = 8;

pub struct TreeMap {
    pub tiles: Vec<Tile>,
    pub unrenderable_tile_coordinates: Option<(u16, u16)>,
    empty_space: RectFloat,
    total_size: f64,
}
impl TreeMap {
    pub fn new(empty_space: &Rect) -> Self {
        let empty_space = RectFloat::new(empty_space);
        TreeMap {
            tiles: vec![],
            unrenderable_tile_coordinates: None,
            total_size: (empty_space.height * empty_space.width) as f64,
            empty_space,
        }
    }
    pub fn populate_tiles<'a>(&'a mut self, children: Vec<&'a FileMetadata>) {
        self.squarify(children, vec![]);
        if let Some((x, y)) = self.unrenderable_tile_coordinates {
            // the unrenderable files area should always be a rectangle
            // so if due to rounding errors some renderable tile is in
            // this area, we'd better remove it
            self.tiles.retain(|tile| tile.x < x || tile.y < y);
        }
    }
    fn layoutrow(&mut self, row: Vec<&FileMetadata>) {
        let row_total = row.iter().fold(0.0, |acc, file_metadata| {
            let size = file_metadata.percentage * self.total_size;
            acc + size
        });
        let should_render_horizontally =
            self.empty_space.width <= self.empty_space.height * HEIGHT_WIDTH_RATIO;
        let mut progress_in_row = if should_render_horizontally {
            self.empty_space.x
        } else {
            self.empty_space.y
        };
        let mut length_of_row_second_side = 0.0;
        for file_metadata in row {
            let size = file_metadata.percentage * self.total_size;
            let tile_length_first_side = if should_render_horizontally {
                (size / row_total) * self.empty_space.width as f64
            } else {
                (size / row_total) * self.empty_space.height as f64
            };

            // we take the highest of length_of_row_second_side and length_candidate so the row will always
            // have the same width, even if it means fudging the calculation a little
            let length_candidate = size / tile_length_first_side;
            let tile_length_second_side = if length_of_row_second_side > length_candidate {
                length_of_row_second_side
            } else {
                length_candidate
            };

            let rect = if should_render_horizontally {
                RectFloat {
                    x: progress_in_row,
                    y: self.empty_space.y,
                    width: tile_length_first_side,
                    height: tile_length_second_side,
                }
            } else {
                RectFloat {
                    x: self.empty_space.x,
                    y: progress_in_row,
                    width: tile_length_second_side,
                    height: tile_length_first_side,
                }
            };
            progress_in_row += tile_length_first_side;

            let tile = Tile::new(&rect, &file_metadata);
            if tile.height < MINIMUM_HEIGHT || tile.width < MINIMUM_WIDTH {
                self.add_unrenderable_tile(&tile);
            } else {
                self.tiles.push(tile)
            }

            if tile_length_second_side > length_of_row_second_side {
                length_of_row_second_side = tile_length_second_side;
            }
        }

        if should_render_horizontally {
            self.empty_space.height -= length_of_row_second_side;
            self.empty_space.y += length_of_row_second_side;
        } else {
            self.empty_space.width -= length_of_row_second_side;
            self.empty_space.x += length_of_row_second_side;
        }
    }
    fn add_unrenderable_tile(&mut self, tile: &Tile) {
        match self.unrenderable_tile_coordinates {
            Some((x, y)) => {
                let x = if tile.x < x { tile.x } else { x };
                let y = if tile.y < y { tile.y } else { y };
                self.unrenderable_tile_coordinates = Some((x, y));
            }
            None => {
                self.unrenderable_tile_coordinates = Some((tile.x, tile.y));
            }
        }
    }

    fn worst_in_renderable_row(
        &self,
        row: &[&FileMetadata],
        length_of_row: f64,
        min_first_side: f64,
        min_second_side: f64,
    ) -> Option<f64> {
        // None means that at least one item in the row is not renderable, so it should not be
        // considered
        let sum = row.iter().fold(0.0, |accum, file_metadata| {
            let size = file_metadata.percentage * self.total_size;
            accum + size
        });
        let mut worst_aspect_ratio = None;
        for val in row.iter() {
            let size = val.percentage * self.total_size;
            let first_side = (size / sum) * length_of_row;
            let second_side = size / first_side;
            if first_side >= min_first_side && second_side >= min_second_side {
                let val_aspect_ratio = if first_side < second_side {
                    first_side / second_side
                } else {
                    second_side / first_side
                };
                match worst_aspect_ratio {
                    Some(current_worst) => {
                        if val_aspect_ratio < current_worst {
                            worst_aspect_ratio = Some(val_aspect_ratio);
                        }
                    }
                    None => {
                        worst_aspect_ratio = Some(val_aspect_ratio);
                    }
                }
            } else {
                return None;
            }
        }
        worst_aspect_ratio
    }

    fn has_renderable_items(
        &self,
        row: &Vec<&FileMetadata>,
        min_first_side: f64,
        min_second_side: f64,
    ) -> bool {
        for val in row.iter() {
            let size = val.percentage * self.total_size;
            if min_first_side * min_second_side <= size {
                return true;
            }
        }
        return false;
    }

    fn squarify<'a>(
        &'a mut self,
        mut children: Vec<&'a FileMetadata>,
        mut row: Vec<&'a FileMetadata>,
    ) {
        let (length_of_row, min_first_side, min_second_side) =
            if self.empty_space.height * HEIGHT_WIDTH_RATIO < self.empty_space.width {
                (
                    self.empty_space.height * HEIGHT_WIDTH_RATIO,
                    MINIMUM_HEIGHT as f64 * HEIGHT_WIDTH_RATIO,
                    MINIMUM_WIDTH as f64 / HEIGHT_WIDTH_RATIO,
                )
            } else {
                (
                    self.empty_space.width / HEIGHT_WIDTH_RATIO,
                    MINIMUM_WIDTH as f64 / HEIGHT_WIDTH_RATIO,
                    MINIMUM_HEIGHT as f64 * HEIGHT_WIDTH_RATIO,
                )
            };

        if children.len() == 0 {
            self.layoutrow(row);
        } else if !self.has_renderable_items(&children, min_first_side, min_second_side) {
            self.layoutrow(row);
            self.layoutrow(children);
        } else {
            let current_row_worst_ratio =
                self.worst_in_renderable_row(&row, length_of_row, min_first_side, min_second_side);
            let row_with_first_child: Vec<&FileMetadata> = row
                .iter()
                .chain(children.iter().take(1))
                .map(|f| *f)
                .collect();

            let row_with_child_worst_ratio = self.worst_in_renderable_row(
                &row_with_first_child,
                length_of_row,
                min_first_side,
                min_second_side,
            );

            match (current_row_worst_ratio, row_with_child_worst_ratio) {
                (None, None) => {
                    // we have renderable children somewhere, but not the way
                    // the row is now and not even if we add the next child
                    // let's add the child and keep looking
                    //
                    // worst case we'll run out of renderable children and layout a row
                    // of all of them together (above)
                    let child0 = children.remove(0);
                    row.push(child0);
                    self.squarify(children, row);
                }
                (None, Some(_next_ratio)) => {
                    // the row with the first child is renderable, as opposed to the current row
                    // let's add the child to it and keep looking for the best ratio
                    let child0 = children.remove(0);
                    row.push(child0);
                    self.squarify(children, row);
                }
                (Some(_current_ratio), None) => {
                    // current row is renderable as is and next row will
                    // just make things worse for us, let's render this
                    // row and keep going
                    self.layoutrow(row);
                    self.squarify(children, vec![]);
                }
                (Some(current_ratio), Some(next_ratio)) => {
                    if current_ratio < next_ratio {
                        // adding the next child will all-in-all be an improvement
                        // let's add it to the row and keep looking to see if we
                        // can add more children to it before laying it out
                        let child0 = children.remove(0);
                        row.push(child0);
                        self.squarify(children, row);
                    } else {
                        // this is the best aspect ratio we'll get, adding the next
                        // child will not be an improvement, let's layout this row
                        // and keep going
                        self.layoutrow(row);
                        self.squarify(children, vec![]);
                    }
                }
            };
        }
    }
}
