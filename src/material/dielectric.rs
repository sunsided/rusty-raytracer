use crate::material::ScatteredRay;
use crate::{Color, HitRecord, Material, Random, Ray, Vec3};
use num_traits::Pow;

pub struct Dielectric {
    /// The index of refraction.
    ir: f32,
}

impl Dielectric {
    pub fn new(ir: f32) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        // Use Schlick's approximation for reflectance.
        let mut r0 = (1. - ref_idx) / (1. + ref_idx);
        r0 *= r0;
        r0 + (1. - r0) * (1. - cosine).pow(5.)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &Random) -> Option<ScatteredRay> {
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
        let direction =
            if cannot_refract || Self::reflectance(cos_theta, refraction_ratio) > rng.sample() {
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
