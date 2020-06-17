use ::std::ffi::OsString;

use crate::state::tiles::{FileMetadata, FileType, RectFloat};

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
    pub fn new(rect: &RectFloat, file_metadata: &FileMetadata) -> Self {
        let rounded = rect.round();
        Tile {
            x: rounded.x,
            y: rounded.y,
            width: rounded.width,
            height: rounded.height,
            name: file_metadata.name.clone(),
            size: file_metadata.size,
            descendants: file_metadata.descendants,
            percentage: file_metadata.percentage,
            file_type: file_metadata.file_type,
        }
    }
    pub fn is_directly_right_of(&self, other: &Tile) -> bool {
        self.x == other.x + other.width
    }

    pub fn is_directly_left_of(&self, other: &Tile) -> bool {
        self.x + self.width == other.x
    }

    pub fn is_directly_below(&self, other: &Tile) -> bool {
        self.y == other.y + other.height
    }

    pub fn is_directly_above(&self, other: &Tile) -> bool {
        self.y + self.height == other.y
    }

    pub fn horizontally_overlaps_with(&self, other: &Tile) -> bool {
        (self.y >= other.y && self.y <= (other.y + other.height))
            || ((self.y + self.height) <= (other.y + other.height)
                && (self.y + self.height) > other.y)
            || (self.y <= other.y && (self.y + self.height >= (other.y + other.height)))
            || (other.y <= self.y && (other.y + other.height >= (self.y + self.height)))
    }

    pub fn vertically_overlaps_with(&self, other: &Tile) -> bool {
        (self.x >= other.x && self.x <= (other.x + other.width))
            || ((self.x + self.width) <= (other.x + other.width) && (self.x + self.width) > other.x)
            || (self.x <= other.x && (self.x + self.width >= (other.x + other.width)))
            || (other.x <= self.x && (other.x + other.width >= (self.x + self.width)))
    }

    pub fn get_vertical_overlap_with(&self, other: &Tile) -> u16 {
        if self.x < other.x {
            if self.x + self.width >= other.x + other.width {
                other.width
            } else {
                self.x + self.width - other.x
            }
        } else if other.x + other.width >= self.x + self.width {
            self.width
        } else {
            other.x + other.width - self.x
        }
    }

    pub fn get_horizontal_overlap_with(&self, other: &Tile) -> u16 {
        if self.y < other.y {
            if self.y + self.height >= other.y + other.height {
                other.height
            } else {
                self.y + self.height - other.y
            }
        } else if other.y + other.height >= self.y + self.height {
            self.height
        } else {
            other.y + other.height - self.y
        }
    }
}
