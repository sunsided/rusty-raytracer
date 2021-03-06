use crate::Random;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub};

#[derive(Debug, PartialEq, PartialOrd, Default, Copy, Clone)]
#[repr(C)]
pub struct Vec3 {
    pub e: [f32; 3],
}

impl Vec3 {
    #[inline]
    pub const fn new(e0: f32, e1: f32, e2: f32) -> Self {
        Self { e: [e0, e1, e2] }
    }

    #[inline]
    pub fn x(&self) -> f32 {
        self.e[0]
    }

    #[inline]
    pub fn y(&self) -> f32 {
        self.e[1]
    }

    #[inline]
    pub fn z(&self) -> f32 {
        self.e[2]
    }

    #[inline]
    pub fn len(&self) -> f32 {
        self.len_squared().sqrt()
    }

    #[inline]
    pub fn len_squared(&self) -> f32 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    #[inline]
    pub fn dot(&self, v: &Vec3) -> f32 {
        self.e[0] * v.e[0] + self.e[1] * v.e[1] + self.e[2] * v.e[2]
    }

    #[inline]
    pub fn cross(&self, v: &Vec3) -> Self {
        Self::new(
            self.e[1] * v.e[2] - self.e[2] * v.e[1],
            self.e[2] * v.e[0] - self.e[0] * v.e[2],
            self.e[0] * v.e[1] - self.e[1] * v.e[0],
        )
    }

    #[inline]
    pub fn as_unit_vector(&self) -> Self {
        *self / self.len()
    }

    #[inline]
    pub fn half(&self) -> Self {
        Vec3::new(self.e[0] * 0.5, self.e[1] * 0.5, self.e[2] * 0.5)
    }

    /// Generates a random vector with components ranging in range `-1.0..1.0`.
    pub fn random(rng: &Random) -> Self {
        let x = rng.sample().mul_add(2., -1.);
        let y = rng.sample().mul_add(2., -1.);
        let z = rng.sample().mul_add(2., -1.);
        Self::new(x, y, z)
    }

    /// Generates a random unit vector.
    pub fn random_unit(rng: &Random) -> Self {
        Self::random_in_unit_sphere(rng).as_unit_vector()
    }

    /// Generates a random vector with components ranging in range `-1.0..1.0` that lies
    /// within the unit sphere.
    pub fn random_in_unit_sphere(rng: &Random) -> Self {
        loop {
            let p = Self::random(rng);
            if p.len_squared() <= 1. {
                return p;
            }
        }
    }

    /// Generates a random vector with components ranging in range `-1.0..1.0` that lies
    /// within the hemisphere of the normal.
    pub fn random_in_hemisphere(normal: &Vec3, rng: &mut Random) -> Self {
        let vector = Self::random_in_unit_sphere(rng);
        if vector.dot(normal) > 0. {
            return vector;
        }
        return -vector;
    }

    pub fn random_in_unit_disk(rng: &Random) -> Vec3 {
        loop {
            let x = rng.sample() * 2. - 1.;
            let y = rng.sample() * 2. - 1.;
            let p = Vec3::new(x, y, 0.);
            if p.len_squared() < 1. {
                return p;
            }
        }
    }

    /// Determines if the vector is new zero
    pub fn near_zero(&self) -> bool {
        const EPS: f32 = 1e-8;
        (self.e[0].abs() < EPS) && (self.e[1].abs() < EPS) && (self.e[2].abs() < EPS)
    }

    /// Reflects a vector along a normal.
    pub fn reflect(&self, normal: &Vec3) -> Self {
        *self - 2. * self.dot(normal) * (*normal)
    }

    pub fn refract(&self, normal: &Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = (-*self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (*self + cos_theta * (*normal));
        let r_out_parallel = -((1.0 - r_out_perp.len_squared()).abs()).sqrt() * (*normal);
        r_out_perp + r_out_parallel
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

impl Add<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn add(self, rhs: f32) -> Self {
        Vec3::new(self.e[0] + rhs, self.e[1] + rhs, self.e[2] + rhs)
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

impl MulAssign<f32> for Vec3 {
    #[inline]
    fn mul_assign(&mut self, rhs: f32) {
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

impl Mul<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn mul(self, rhs: f32) -> Self {
        Vec3::new(self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs)
    }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    #[inline]
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs.mul(self)
    }
}

impl DivAssign<f32> for Vec3 {
    #[inline]
    fn div_assign(&mut self, rhs: f32) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    #[inline]
    fn div(self, rhs: f32) -> Self {
        let inv = 1. / rhs;
        Vec3::new(self.e[0] * inv, self.e[1] * inv, self.e[2] * inv)
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self::new(-self.e[0], -self.e[1], -self.e[2])
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
    pub fn near_zero_works() {
        assert!(Vec3::default().near_zero());
        assert!(Vec3::new(1e-9, 0., 0.).near_zero());
    }

    #[test]
    pub fn len_sqrt_works() {
        let vec = Vec3::new(1., 2., 3.);
        assert_eq!(vec.len_squared(), 1. + 4. + 9.);
    }

    #[test]
    pub fn len_works() {
        let vec = Vec3::new(1., 2., 3.);
        assert_eq!(vec.len(), (1f32 + 4. + 9.).sqrt());
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

        let vec = 2. * lhs;
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
    pub fn neg_works() {
        let lhs = Vec3::new(1., 2., 3.);
        let vec = -lhs;
        assert_eq!(vec.e, [-1., -2., -3.]);
    }

    #[test]
    pub fn half_works() {
        let vec = Vec3::new(1., 2., 3.).half();
        assert_eq!(vec.e, [0.5, 1., 1.5]);
    }

    #[test]
    pub fn unit_vector_works() {
        let lhs = Vec3::new(1., 0., 0.);
        let vec = lhs.as_unit_vector();
        assert_eq!(vec.e, [1., 0., 0.]);

        let lhs = Vec3::new(12., -3., -4.);
        let vec = lhs.as_unit_vector();
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
        let vec = lhs.cross(&rhs);
        assert_eq!(vec.e, [-15., -2., 39.]);

        let lhs = Vec3::new(3., -3., 1.);
        let rhs = Vec3::new(-12., 12., -4.);
        let vec = lhs.cross(&rhs);
        assert_eq!(vec.e, [0., 0., 0.]);
    }
}
