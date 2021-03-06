use crate::{HitRecord, Hittable, Material, Point3, Ray, GRID_SCALE};
use space_partitioning::quadtree::AABB;
use std::sync::Arc;

pub struct Sphere {
    pub center: Point3,
    pub radius: f32,
    pub material: Arc<Box<dyn Material>>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f32, material: Arc<Box<dyn Material>>) -> Self {
        Self {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = r.origin - self.center;
        let a = r.direction.len_squared();
        let half_b = oc.dot(&r.direction);
        let c = oc.len_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }

        let sqrt_d = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        // Since it's a quadratic equation there may be two solutions,
        // and we need to check both for validity.
        let mut root = (-half_b - sqrt_d) / a;
        if root < t_min || root > t_max {
            // Check the second solution.
            root = (-half_b + sqrt_d) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let hit_point = r.at(root);
        let outward_normal = (hit_point - self.center) / self.radius;
        Some(HitRecord::new_from_ray(
            r,
            root,
            hit_point,
            outward_normal,
            self.material.clone(),
        ))
    }

    fn to_aabb(&self) -> AABB {
        let x = self.center.x() * GRID_SCALE;
        let z = self.center.z() * GRID_SCALE;
        let r = self.radius * GRID_SCALE;

        AABB::new(
            (x - r).floor() as i32,
            (z - r).floor() as i32,
            (x + r).ceil() as i32,
            (z + r).ceil() as i32,
        )
    }
}
