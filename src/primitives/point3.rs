use crate::Vec3;
use num_traits::MulAdd;

/// A 3D point.
pub type Point3 = Vec3;

impl MulAdd<f32, Vec3> for Point3 {
    type Output = Point3;

    fn mul_add(self, a: f32, b: Vec3) -> Self::Output {
        // self * a + b
        Point3::new(
            self.e[0].mul_add(a, b.e[0]),
            self.e[1].mul_add(a, b.e[1]),
            self.e[2].mul_add(a, b.e[2]),
        )
    }
}
