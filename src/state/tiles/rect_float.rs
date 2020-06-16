use ::tui::layout::Rect;

#[derive(Clone, Debug)]
pub struct RectFloat {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl RectFloat {
    pub fn new(rect: &Rect) -> Self {
        RectFloat {
            x: rect.x as f64,
            y: rect.y as f64,
            height: rect.height as f64,
            width: rect.width as f64,
        }
    }
    pub fn round(&self) -> Rect {
        let rounded_x = self.x.round();
        let rounded_y = self.y.round();
        let mut rect = Rect {
            x: rounded_x as u16,
            y: rounded_y as u16,
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
