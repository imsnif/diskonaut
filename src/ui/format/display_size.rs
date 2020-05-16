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

pub struct DisplaySizeRounded(pub f64);

impl fmt::Display for DisplaySizeRounded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 > 999_999_999.0 {
            write!(f, "{:.0}G", self.0 / 1_000_000_000.0)
        } else if self.0 > 999_999.0 {
            write!(f, "{:.0}M", self.0 / 1_000_000.0)
        } else if self.0 > 999.0 {
            write!(f, "{:.0}K", self.0 / 1000.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
