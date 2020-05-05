use crate::ui::rectangle_grid::{MINIMUM_HEIGHT, MINIMUM_WIDTH};
use tui::layout::Rect;

#[derive(Clone, Debug)]
pub struct RectFloat {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

// TODO: CONTINUE HERE (03/05) - just finished moving all the methods to RectFloat
// now we need to move RectFloat to a separate file
//
// might be best to create a "rectangles" or some such folder inside state and put
// tiles plus this new file in there... maybe also want to think of a new name for RectFloat?
//
// then keep looking here and see what we'd like to refactor... maybe keep breaking things away
// from RectangleGrid could be related?
impl RectFloat {
    pub fn is_right_of(&self, other: &RectFloat) -> bool {
        self.x >= other.x + other.width
    }

    pub fn is_left_of(&self, other: &RectFloat) -> bool {
        self.x + self.width <= other.x
    }

    pub fn is_below(&self, other: &RectFloat) -> bool {
       self.y >= other.y + other.height
    }

    pub fn is_above(&self, other: &RectFloat) -> bool {
       self.y + self.height <= other.y
    }

    pub fn horizontally_overlaps_with(&self, other: &RectFloat) -> bool {
        ( self.y >= other.y && self.y <= (other.y + other.height) ) ||
        ( (self.y + self.height) <= (other.y + other.height) && (self.y + self.height) > other.y) ||
        (self.y <= other.y && (self.y + self.height >= (other.y + other.height)) ) ||
        ( other.y <= self.y && (other.y + other.height >= (self.y + self.height)) )
    }

    pub fn vertically_overlaps_with(&self, other: &RectFloat) -> bool {
        ( self.x >= other.x && self.x <= (other.x + other.width) ) ||
        ( (self.x + self.width) <= (other.x + other.width) && (self.x + self.width) > other.x) ||
        ( self.x <= other.x && (self.x + self.width >= (other.x + other.width)) ) ||
        ( other.x <= self.x && (other.x + other.width >= (self.x + self.width)) )
    } 

    pub fn get_vertical_overlap_with(&self, other: &RectFloat) -> f64 {
        if self.x < other.x {
            if self.x + self.width >= other.x + other.width {
                other.width
            } else {
                self.x + self.width - other.x
            } 
        } else {
            if other.x + other.width >= self.x + self.width {
                self.width
            } else {
                other.x + other.width - self.x
            } 
        }
    }

    pub fn get_horizontal_overlap_with(&self, other: &RectFloat) -> f64 {
        if self.y < other.y {
            if self.y + self.height >= other.y + other.height {
                other.height
            } else {
                self.y + self.height - other.y
            } 
        } else {
            if other.y + other.height >= self.y + self.height {
                self.height
            } else {
                other.y + other.height - self.y
            } 
        }
    }

    pub fn is_atleast_minimum_size(&self) -> bool {
        self.height > MINIMUM_HEIGHT as f64 && self.width > MINIMUM_WIDTH as f64
    }

    pub fn is_aligned_left_with(&self, other: &RectFloat) -> bool {
        self.x.round() == other.x.round()
    }
    pub fn is_aligned_right_with(&self, other: &RectFloat) -> bool {
        (self.x + self.width).round() == (other.x + other.width).round()
    }

    pub fn is_aligned_top_with(&self, other: &RectFloat) -> bool {
        self.y.round() == other.y.round()
    }

    pub fn is_aligned_bottom_with(&self, other: &RectFloat) -> bool {
        (self.y + self.height).round() == (other.y + other.height).round()
    }
    
    pub fn round(&self) -> Rect {
        let rounded_x = self.x.round();
        let rounded_y = self.y.round();
        let mut rect = Rect {
            x: rounded_x as u16,
            y: rounded_y  as u16,
            width: ((self.x - rounded_x) + self.width).round() as u16,
            height: ((self.y - rounded_y) + self.height).round() as u16,
        };

        // fix rounding errors
        if (self.x + self.width).round() as u16 > rect.x + rect.width {
            rect.width += 1;
        }
        if (self.y + self.height).round() as u16 > rect.y + rect.height {
            rect.height += 1;
        }
        rect
    }
}
