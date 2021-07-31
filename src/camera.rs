use crate::{Point3, Ray, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64) -> Self {
        const FOCAL_LENGTH: f64 = 1.0;
        const VIEWPORT_HEIGHT: f64 = 2.0;
        let viewport_height: f64 = aspect_ratio * VIEWPORT_HEIGHT;

        let origin: Point3 = Point3::default();
        let horizontal = Vec3::new(viewport_height as _, 0., 0.);
        let vertical = Vec3::new(0., VIEWPORT_HEIGHT as _, 0.);
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
