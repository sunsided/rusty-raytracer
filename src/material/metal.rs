use crate::material::ScatteredRay;
use crate::{Color, HitRecord, Material, Random, Ray, Vec3};

pub struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        Self {
            albedo,
            fuzziness: fuzziness.clamp(0., 1.),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit: &HitRecord, rng: &mut Random) -> Option<ScatteredRay> {
        let reflected = ray.direction.as_unit_vector().reflect(&hit.normal);
        let scattered = Ray::new(
            hit.point,
            reflected + self.fuzziness * Vec3::random_in_unit_sphere(rng),
        );

        if scattered.direction.dot(&hit.normal) <= 0. {
            return None;
        }

        Some(ScatteredRay {
            ray: scattered,
            attenuation: self.albedo,
        })
    }
}
