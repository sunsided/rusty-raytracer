use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub};

#[derive(Debug, PartialEq, PartialOrd, Default, Copy, Clone)]
#[repr(C)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    #[inline]
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    #[inline]
    pub fn x(&self) -> f64 {
        self.e[0]
    }

    #[inline]
    pub fn y(&self) -> f64 {
        self.e[1]
    }

    #[inline]
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    #[inline]
    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }

    #[inline]
    pub fn len_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    #[inline]
    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    #[inline]
    pub fn cross(u: &Vec3, v: &Vec3) -> Self {
        Self::new(
            u.e[1] * v.e[2] - u.e[2] * v.e[1],
            u.e[2] * v.e[0] - u.e[0] * v.e[2],
            u.e[0] * v.e[1] - u.e[1] * v.e[0],
        )
    }

    #[inline]
    pub fn unit_vector(&self) -> Self {
        *self / self.len()
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

impl Add for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: Self) -> Self {
        Vec3::new(
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        )
    }
}

impl AddAssign for Vec3 {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl Sub for Vec3 {
    type Output = Self;

    #[inline]
    fn sub(self, rhs: Self) -> Self {
        Vec3::new(
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        )
    }
}

impl MulAssign<f64> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl Mul for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: Self) -> Self {
        Vec3::new(
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        )
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f64) -> Self {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl DivAssign<f64> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f64) -> Self {
        let inv = 1. / rhs;
        Vec3::new(self.e[0] * inv, self.e[1] * inv, self.e[2] * inv)
    }
}

#[cfg(test)]
pub mod test {
    use super::*;

    #[test]
    pub fn coordinates_work() {
        let vec = Vec3::new(1., 2., 3.);
        assert_eq!(vec.e, [1., 2., 3.]);
        assert_eq!(vec.x(), 1.);
        assert_eq!(vec.y(), 2.);
        assert_eq!(vec.z(), 3.);
    }

    #[test]
    pub fn len_sqrt_works() {
        let vec = Vec3::new(1., 2., 3.);
        assert_eq!(vec.len_squared(), 1. + 4. + 9.);
    }

    #[test]
    pub fn len_works() {
        let vec = Vec3::new(1., 2., 3.);
        assert_eq!(vec.len(), (1f64 + 4. + 9.).sqrt());
    }

    #[test]
    pub fn add_assign_works() {
        let mut vec = Vec3::new(1., 2., 3.);
        let rhs = Vec3::new(1., 1., 1.);
        vec += rhs;
        assert_eq!(vec.e, [2., 3., 4.]);
    }

    #[test]
    pub fn add_works() {
        let lhs = Vec3::new(1., 2., 3.);
        let rhs = Vec3::new(1., 1., 1.);
        let vec = lhs + rhs;
        assert_eq!(vec.e, [2., 3., 4.]);
    }

    #[test]
    pub fn sub_works() {
        let lhs = Vec3::new(1., 2., 3.);
        let rhs = Vec3::new(1., 1., 1.);
        let vec = lhs - rhs;
        assert_eq!(vec.e, [0., 1., 2.]);
    }

    #[test]
    pub fn mul_assign_works() {
        let mut vec = Vec3::new(1., 2., 3.);
        let rhs = 2.;
        vec *= rhs;
        assert_eq!(vec.e, [2., 4., 6.]);
    }

    #[test]
    pub fn mul_vec_works() {
        let lhs = Vec3::new(1., 2., 3.);
        let rhs = Vec3::new(2., 2., 2.);
        let vec = lhs * rhs;
        assert_eq!(vec.e, [2., 4., 6.]);
    }

    #[test]
    pub fn mul_scalar_works() {
        let lhs = Vec3::new(1., 2., 3.);
        let vec = lhs * 2.;
        assert_eq!(vec.e, [2., 4., 6.]);
    }

    #[test]
    pub fn div_assign_works() {
        let mut vec = Vec3::new(1., 2., 3.);
        let rhs = 2.;
        vec /= rhs;
        assert_eq!(vec.e, [0.5, 1., 1.5]);
    }

    #[test]
    pub fn div_scalar_works() {
        let lhs = Vec3::new(1., 2., 3.);
        let vec = lhs / 2.;
        assert_eq!(vec.e, [0.5, 1., 1.5]);
    }

    #[test]
    pub fn unit_vector_works() {
        let lhs = Vec3::new(1., 0., 0.);
        let vec = lhs.unit_vector();
        assert_eq!(vec.e, [1., 0., 0.]);

        let lhs = Vec3::new(12., -3., -4.);
        let vec = lhs.unit_vector();
        assert_eq!(vec.e, [12. / 13., -3. / 13., -4. / 13.]);
    }

    #[test]
    pub fn dot_works() {
        let lhs = Vec3::new(1., 2., 3.);
        let rhs = Vec3::new(20., 30., 40.);
        let dot = Vec3::dot(&lhs, &rhs);
        assert_eq!(dot, 1. * 20. + 2. * 30. + 3. * 40.);
    }

    #[test]
    pub fn cross_works() {
        let lhs = Vec3::new(3., -3., 1.);
        let rhs = Vec3::new(4., 9., 2.);
        let vec = Vec3::cross(&lhs, &rhs);
        assert_eq!(vec.e, [-15., -2., 39.]);

        let lhs = Vec3::new(3., -3., 1.);
        let rhs = Vec3::new(-12., 12., -4.);
        let vec = Vec3::cross(&lhs, &rhs);
        assert_eq!(vec.e, [0., 0., 0.]);
    }
}
