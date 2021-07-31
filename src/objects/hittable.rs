use crate::{Material, Point3, Ray, Vec3};
use std::sync::Arc;

pub struct HitRecord {
    /// The distance from the ray's origin at which the hit occurred.
    pub t: f64,

    /// The hit point.
    pub point: Point3,

    /// The normal of the surface at the hit point.
    pub normal: Vec3,

    /// Indicates whether the normal is facing outward (along the ray) if `true`,
    /// or inside (against the ray) if `false`.
    ///
    /// Doing it like this instead of just observing the dot product
    /// whenever needed is a question of preference, and the author of
    /// the instruction text chose to do it this way.
    pub is_front_facing: bool,

    /// The material that was hit.
    pub material: Arc<Box<dyn Material>>,
}

impl HitRecord {
    pub fn new_from_ray(
        ray: &Ray,
        t: f64,
        p: Point3,
        outward_normal: Vec3,
        material: Arc<Box<dyn Material>>,
    ) -> Self {
        let is_front_facing = ray.direction.dot(&outward_normal) < 0.;
        Self {
            t,
            point: p,
            is_front_facing,
            normal: if is_front_facing {
                outward_normal
            } else {
                -outward_normal
            },
            material,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

#[derive(Default)]
pub struct HittableList {
    objects: Vec<Arc<Box<dyn Hittable>>>,
}

impl HittableList {
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(Arc::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut best_hit = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(hit) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = hit.t;
                best_hit = Some(hit);
            }
        }

        best_hit
    }
}