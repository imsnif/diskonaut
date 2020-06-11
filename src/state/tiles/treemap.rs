use crate::state::FileMetadata;
use crate::ui::rectangle_grid::{MINIMUM_HEIGHT, MINIMUM_WIDTH} ;
use crate::state::{RectFloat, Tile};

const HEIGHT_WIDTH_RATIO: f64 = 2.5;

pub struct TreeMap {
    pub tiles: Vec<Tile>,
    empty_space: RectFloat,
    total_size: f64,
}
impl TreeMap {

    pub fn new (empty_space: RectFloat) -> Self {
        TreeMap {
            tiles: vec![],
            total_size: (empty_space.height * empty_space.width) as f64,
            empty_space,
        }
    }
    fn layoutrow(&mut self, row: Vec<&FileMetadata>) {
        let row_total = row.iter().fold(0.0, |acc, file_metadata| {
            let size = file_metadata.percentage * self.total_size;
            acc + size
        });
        if self.empty_space.width <= self.empty_space.height * HEIGHT_WIDTH_RATIO {
            let mut x = self.empty_space.x;
            let mut row_height = 0.0;
            for file_metadata in row {
                let size = file_metadata.percentage * self.total_size;
                let width = (size / row_total) * self.empty_space.width as f64;
                let relative_height = size / width;
                // we take the highest of row_height and relative_height so the row will always
                // have the same height, even if it means fudging the calculation a little
                let height = if row_height > relative_height { row_height } else { relative_height };

                let rect = RectFloat {x, y: self.empty_space.y, width, height };
                x += width;
                self.tiles.push(Tile::new(&rect, &file_metadata));
                if height > row_height {
                    row_height = height;
                }
            }
            self.empty_space.height -= row_height;
            self.empty_space.y += row_height;
        } else {
            let mut y = self.empty_space.y;
            let mut row_width = 0.0;
            for file_metadata in row {
                let size = file_metadata.percentage * self.total_size;
                let height = (size / row_total) * self.empty_space.height as f64;
                let relative_width = size / height;
                // we take the highest of row_width and relative_width so the row will always
                // have the same width, even if it means fudging the calculation a little
                let width = if row_width > relative_width { row_width } else { relative_width };
    
                let rect = RectFloat { x: self.empty_space.x, y, width, height };
                y += height;
                self.tiles.push(Tile::new(&rect, &file_metadata));
                if width > row_width {
                    row_width = width; // TODO: check if this changes in iterations
                }
            }
            self.empty_space.width -= row_width; // TODO: check if this changes in iterations
            self.empty_space.x += row_width;
        }
    }

    fn worst (&self, row: &[&FileMetadata], length_of_row: f64, min_first_side: f64, min_second_side: f64) -> f64 {
        let sum = row.iter().fold(0.0, |accum, file_metadata| {
            let size = file_metadata.percentage * self.total_size;
            accum + size
        });
        let mut worst_aspect_ratio = 0.0;
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

    fn has_renderable_items (&self, row: &Vec<&FileMetadata>, min_first_side: f64, min_second_side: f64) -> bool {
        for val in row.iter() {
            let size = val.percentage * self.total_size;
            if min_first_side * min_second_side <= size {
                return true
            }
        }
        return false;
    }

    pub fn squarify <'a>(&'a mut self, mut children: Vec<&'a FileMetadata>, mut row: Vec<&'a FileMetadata>) {
        let (length_of_row, min_first_side, min_second_side) = if self.empty_space.height * HEIGHT_WIDTH_RATIO < self.empty_space.width {
            (self.empty_space.height * HEIGHT_WIDTH_RATIO, MINIMUM_HEIGHT as f64 * HEIGHT_WIDTH_RATIO, MINIMUM_WIDTH as f64 / HEIGHT_WIDTH_RATIO)
        } else {
            (self.empty_space.width / HEIGHT_WIDTH_RATIO, MINIMUM_WIDTH as f64 / HEIGHT_WIDTH_RATIO, MINIMUM_HEIGHT as f64 * HEIGHT_WIDTH_RATIO)
        };

        if children.len() == 0 && !row.is_empty() { // TODO: better
            self.layoutrow(row);
        } else if children.len() == 0 {
            return;
        } else {

            if !self.has_renderable_items(&children, min_first_side, min_second_side) {
                if row.len() > 0 {
                    self.layoutrow(row);
                    self.squarify(children, vec![]);
                } else {
                    for child in children.drain(..) {
                        row.push(child);
                    }
                    self.layoutrow(row);
                    self.squarify(children, vec![]);
                }
                return;
            }

            let current_row_worst_ratio = self.worst(&row, length_of_row, min_first_side, min_second_side);
            let row_with_first_child: Vec<&FileMetadata> = row.iter()
                .chain(children.iter().take(1))
                .map(|f| *f)
                .collect();

            let row_with_child_worst_ratio = self.worst(&row_with_first_child, length_of_row, min_first_side, min_second_side);

            if current_row_worst_ratio != 0.0 && row_with_child_worst_ratio == 0.0 {
                self.layoutrow(row);
                self.squarify(children, vec![]);
            } else if row.len() == 1 || current_row_worst_ratio <= row_with_child_worst_ratio || current_row_worst_ratio == 0.0 {
                let child0 = children.remove(0);
                row.push(child0);
                self.squarify(children, row);
            } else {
                self.layoutrow(row);
                self.squarify(children, vec![]);
            }
        }
    }
}
