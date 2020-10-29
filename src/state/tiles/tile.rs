use ::std::ffi::OsString;

use crate::state::tiles::{FileMetadata, FileType, RectFloat};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HV {
    Horizontal,
    Vertical,
}

#[derive(Clone, Debug)]
pub struct Tile {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub name: OsString,
    pub size: u128,
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
    pub fn position(&self, hv: HV) -> u16 {
        if hv == HV::Horizontal {
            self.x
        } else {
            self.y
        }
    }
    pub fn extent(&self, hv: HV) -> u16 {
        if hv == HV::Horizontal {
            self.width
        } else {
            self.height
        }
    }
    pub fn is_directly_after(&self, other: &Tile, hv: HV) -> bool {
        self.position(hv) == other.position(hv) + other.extent(hv)
    }

    pub fn is_directly_before(&self, other: &Tile, hv: HV) -> bool {
        self.position(hv) + self.extent(hv) == other.position(hv)
    }

    pub fn overlaps_with(&self, other: &Tile, hv: HV) -> bool {
        let hv = if HV::Horizontal == hv {
            HV::Vertical
        } else {
            HV::Horizontal
        };
        (self.position(hv) >= other.position(hv)
            && self.position(hv) <= (other.position(hv) + other.extent(hv)))
            || ((self.position(hv) + self.extent(hv)) <= (other.position(hv) + other.extent(hv))
                && (self.position(hv) + self.extent(hv)) > other.position(hv))
            || (self.position(hv) <= other.position(hv)
                && (self.position(hv) + self.extent(hv) >= (other.position(hv) + other.extent(hv))))
            || (other.position(hv) <= self.position(hv)
                && (other.position(hv) + other.extent(hv) >= (self.position(hv) + self.extent(hv))))
    }

    pub fn get_overlap_with(&self, other: &Tile, hv: HV) -> u16 {
        let hv = if HV::Horizontal == hv {
            HV::Vertical
        } else {
            HV::Horizontal
        };
        std::cmp::min(
            self.position(hv) + self.extent(hv),
            other.position(hv) + other.extent(hv),
        ) - std::cmp::max(self.position(hv), other.position(hv))
    }
}
