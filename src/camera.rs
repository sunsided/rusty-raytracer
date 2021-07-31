use crate::{Degrees, Point3, Ray, Vec3};
use std::ops::Deref;

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    /// Constructs a new camera.
    ///
    /// # Arguments
    /// * `look_from`: The origin point of the camera.
    /// * `look_at`: The point to look at.
    /// * `view_up`: The up axis of the camera.
    /// * `vfov`: The vertical field of view in degrees.
    /// * `aspect_ratio`: The aspect ratio.
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        view_up: Vec3,
        vfov: Degrees,
        aspect_ratio: f64,
    ) -> Self {
        let theta = vfov.0.to_radians();
        let h = (theta * 0.5).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).as_unit_vector();
        let u = view_up.cross(&w).as_unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - horizontal.half() - vertical.half() - w;

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
