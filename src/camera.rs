use crate::{Point3, Ray, Vec3};
use std::ops::Deref;

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

pub struct Degrees(pub f64);

impl Camera {
    /// Constructs a new camera.
    ///
    /// # Arguments
    /// * `vfov`: The vertical field of view in degrees.
    /// * `aspect_ratio`: The aspect ratio.
    pub fn new(vfov: Degrees, aspect_ratio: f64) -> Self {
        let theta = vfov.0.to_radians();
        let h = (theta * 0.5).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        const FOCAL_LENGTH: f64 = 1.0;

        let origin: Point3 = Point3::default();
        let horizontal = Vec3::new(viewport_width as _, 0., 0.);
        let vertical = Vec3::new(0., viewport_height as _, 0.);
        let lower_left_corner =
            origin - horizontal.half() - vertical.half() - Vec3::new(0., 0., FOCAL_LENGTH);

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
