use ::std::fmt;

pub struct DisplaySize(pub f64);

impl fmt::Display for DisplaySize{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 > 999_999_999.0 {
            write!(f, "{:.1}G", self.0 / 1073741824.0) // 1024 * 1024 * 1024
        } else if self.0 > 999_999.0 {
            write!(f, "{:.1}M", self.0 / 1048576.0) //  1024 * 1024
        } else if self.0 > 999.0 {
            write!(f, "{:.1}K", self.0 / 1024.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}

pub struct DisplaySizeRounded(pub f64);

impl fmt::Display for DisplaySizeRounded {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0 > 999_999_999.0 {
            write!(f, "{:.0}G", self.0 / 1073741824.0) // 1024 * 1024 * 1024
        } else if self.0 > 999_999.0 {
            write!(f, "{:.0}M", self.0 / 1048576.0) //  1024 * 1024
        } else if self.0 > 999.0 {
            write!(f, "{:.0}K", self.0 / 1024.0)
        } else {
            write!(f, "{}", self.0)
        }
    }
}
