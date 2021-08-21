use crate::{Point3, Vec3, GRID_SCALE};
use num_traits::MulAdd;
use space_partitioning::intersections::IntersectsWith;
use space_partitioning::quadtree::AABB;

#[derive(Debug, Default, Clone)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    inv_direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        let unit = direction.as_unit_vector();
        Self {
            origin,
            direction: unit,
            inv_direction: Vec3::new(1.0 / unit.x(), 1.0 / unit.y(), 1.0 / unit.z()),
        }
    }

    /// Linearly interpolates the point that is `t` units
    /// away from the ray's origin, along the ray's direction.
    #[inline]
    pub fn at(&self, t: f32) -> Point3 {
        // self.origin + t * self.direction
        self.direction.mul_add(t, self.origin)
    }
}

impl IntersectsWith<AABB> for Ray {
    fn intersects_with(&self, other: &AABB) -> bool {
        // https://gamedev.stackexchange.com/a/18459/10433

        let t1 = (other.tl.x as f32 - self.origin.x() * GRID_SCALE) * self.inv_direction.x();
        let t2 = (other.br.x as f32 - self.origin.x() * GRID_SCALE) * self.inv_direction.x();
        let t3 = (other.br.y as f32 - self.origin.z() * GRID_SCALE) * self.inv_direction.z();
        let t4 = (other.tl.y as f32 - self.origin.z() * GRID_SCALE) * self.inv_direction.z();

        let tmin = t1.min(t2).max(t3.min(t4));
        let tmax = t1.max(t2).min(t3.max(t4));

        // if tmax < 0, ray (line) is intersecting AABB, but the whole AABB is behind us
        // if tmin > tmax, ray doesn't intersect AABB
        if (tmax < 0.) | (tmin > tmax) {
            return false;
        }

        return true;
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn mul_add_works() {
        let origin = Point3::new(1., 2., 3.);
        let direction = Vec3::new(1., 1., 1.);
        let t = 2.;

        let ray = Ray::new(origin.clone(), direction.clone());
        let calculated_at = ray.at(t);

        let expected_at = origin + t * direction;
        assert_eq!(calculated_at, expected_at);
    }
}
