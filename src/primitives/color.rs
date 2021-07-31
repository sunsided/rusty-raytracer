use crate::Vec3;
use num_traits::Pow;
use std::fmt::{Display, Formatter};

/// An RGB color.
pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, samples_per_pixel: usize, gamma: f32) -> ColorFormatter {
        ColorFormatter {
            color: &self,
            samples_per_pixel,
            gamma,
        }
    }
}

/// Helper class for formatting colors.
pub struct ColorFormatter<'a> {
    color: &'a Color,
    samples_per_pixel: usize,
    gamma: f32,
}

impl<'a> Display for ColorFormatter<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let scale = 1. / self.samples_per_pixel as f32;
        let r = self.color.e[0] * scale;
        let g = self.color.e[1] * scale;
        let b = self.color.e[2] * scale;

        // Gamma-correct the colors.
        let gamma_factor = 1. / self.gamma;
        let r = r.pow(gamma_factor);
        let g = g.pow(gamma_factor);
        let b = b.pow(gamma_factor);

        let ir = (256. * r.clamp(0., 0.999)) as u8;
        let ig = (256. * g.clamp(0., 0.999)) as u8;
        let ib = (256. * b.clamp(0., 0.999)) as u8;

        write!(f, "{} {} {}\n", ir, ig, ib)
    }
}
