mod lambertian;
mod metal;

use crate::{Color, HitRecord, Random, Ray};
pub use lambertian::Lambertian;
pub use metal::Metal;
use std::sync::Arc;

pub type MaterialPtr = Arc<Box<dyn Material>>;

pub struct ScatteredRay {
    pub ray: Ray,
    pub attenuation: Color,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Random) -> Option<ScatteredRay>;
}
