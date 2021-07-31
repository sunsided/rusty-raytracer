use crate::{Point3, Vec3};
use num_traits::MulAdd;

#[derive(Debug, Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.as_unit_vector(),
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
