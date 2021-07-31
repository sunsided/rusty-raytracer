use crate::vec3::Vec3;
use std::fmt::{Display, Formatter};

/// An RGB color.
pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, samples_per_pixel: usize) -> ColorFormatter {
        ColorFormatter {
            color: &self,
            samples_per_pixel,
        }
    }
}

/// Helper class for formatting colors.
pub struct ColorFormatter<'a> {
    color: &'a Color,
    samples_per_pixel: usize,
}

impl<'a> Display for ColorFormatter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let scale = 1. / self.samples_per_pixel as f64;
        let r = self.color.e[0] * scale;
        let g = self.color.e[1] * scale;
        let b = self.color.e[2] * scale;

        let ir = (256. * r.clamp(0., 0.999)) as u8;
        let ig = (256. * g.clamp(0., 0.999)) as u8;
        let ib = (256. * b.clamp(0., 0.999)) as u8;

        write!(f, "{} {} {}\n", ir, ig, ib)
    }
}
