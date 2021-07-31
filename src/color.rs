use crate::vec3::Vec3;
use std::fmt::{Display, Formatter};

/// An RGB color.
pub type Color = Vec3;

impl Color {
    pub fn write_color(&self) -> ColorFormatter {
        ColorFormatter(&self)
    }
}

/// Helper class for formatting colors.
pub struct ColorFormatter<'a>(&'a Color);

impl<'a> Display for ColorFormatter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ir = (255.299 * self.0.e[0]) as u8;
        let ig = (255.299 * self.0.e[1]) as u8;
        let ib = (255.299 * self.0.e[2]) as u8;

        write!(f, "{} {} {}\n", ir, ig, ib)
    }
}
