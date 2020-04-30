use crate::ui::FileMetadata;
use crate::ui::rectangle_grid::{FileSizeRect, RectFloat, MINIMUM_HEIGHT, MINIMUM_WIDTH} ;

const HEIGHT_WIDTH_RATIO: f64 = 2.5;

pub struct TreeMap {
    pub rectangles: Vec<FileSizeRect>,
    empty_space: RectFloat,
    total_size: f64,
}
impl TreeMap {

    pub fn new (empty_space: RectFloat) -> Self {
        TreeMap {
            rectangles: vec![],
            total_size: (empty_space.height * empty_space.width) as f64,
            empty_space,
        }
    }
    fn layoutrow(&mut self, row: Vec<FileMetadata>) {
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
                let height = size / width;
                let rect_with_text = FileSizeRect {
                    rect: RectFloat {x, y: self.empty_space.y, width: width , height: height },
                    file_metadata,
                    selected: false,
                };
                x += rect_with_text.rect.width;
                self.rectangles.push(rect_with_text);
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
            let width = size / height;

            let mut rect_with_text = FileSizeRect {
                rect: RectFloat { x: self.empty_space.x, y, width: width, height: height },
                file_metadata,
                selected: false,
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

    fn worst (&self, row: &Vec<&FileMetadata>, length_of_row: f64, min_first_side: f64, min_second_side: f64) -> f64 {
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

    pub fn squarify (&mut self, mut children: Vec<FileMetadata>, mut row: Vec<FileMetadata>) {
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

            let mut row_refs: Vec<&FileMetadata> = row.iter().collect();

            let current_row_worst_ratio = self.worst(&row_refs, length_of_row, min_first_side, min_second_side);

            let children_refs: Vec<&FileMetadata> = children.iter().collect();
            if !self.has_renderable_items(&children_refs, min_first_side, min_second_side) {
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

            row_refs.push(&children[0]);

            let row_with_child_worst_ratio = self.worst(&row_refs, length_of_row, min_first_side, min_second_side);

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
