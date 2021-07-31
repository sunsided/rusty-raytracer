use crate::material::ScatteredRay;
use crate::{Color, HitRecord, Material, Random, Ray, Vec3};

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Random) -> Option<ScatteredRay> {
        let reflected = ray.direction.as_unit_vector().reflect(&hit.normal);
        let scattered = Ray::new(hit.point, reflected);

        if scattered.direction.dot(&hit.normal) <= 0. {
            return None;
        }

        Some(ScatteredRay {
            ray: scattered,
            attenuation: self.albedo,
        })
    }
}
