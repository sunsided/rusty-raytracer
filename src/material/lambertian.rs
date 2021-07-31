use crate::material::ScatteredRay;
use crate::{Color, HitRecord, Material, Random, Ray, Vec3};

pub struct Lambertian {
    albedo: Color,
    scatter_probability: f64,
}

impl Lambertian {
    pub fn new(albedo: Color, scatter_probability: f64) -> Self {
        Self {
            albedo,
            scatter_probability: scatter_probability.clamp(0., 1.),
        }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, hit: &HitRecord, rng: &Random) -> Option<ScatteredRay> {
        let p = rng.sample();
        if p > self.scatter_probability {
            return None;
        }

        let mut scatter_direction = hit.normal + Vec3::random_unit(rng);

        // Fix degenerate scatter directions.
        if scatter_direction.near_zero() {
            scatter_direction = hit.normal;
        }

        let scattered = Ray::new(hit.point, scatter_direction);
        Some(ScatteredRay {
            ray: scattered,
            attenuation: self.albedo / self.scatter_probability,
        })
    }
}
