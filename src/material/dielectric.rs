use crate::material::ScatteredRay;
use crate::{Color, HitRecord, Material, Random, Ray, Vec3};

pub struct Dielectric {
    /// The index of refraction.
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Random) -> Option<ScatteredRay> {
        let attenuation: Color = Vec3::new(1., 1., 1.);
        let refraction_ratio = if hit.is_front_facing {
            1. / self.ir
        } else {
            self.ir
        };

        let unit_direction = ray.direction.as_unit_vector();
        let cos_theta = (-unit_direction).dot(&hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract {
            unit_direction.reflect(&hit.normal)
        } else {
            unit_direction.refract(&hit.normal, refraction_ratio)
        };

        Some(ScatteredRay {
            ray: Ray::new(hit.point, direction),
            attenuation,
        })
    }
}
