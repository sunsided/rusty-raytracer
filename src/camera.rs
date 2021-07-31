use crate::{Point3, Ray, Vec3};

pub struct Camera {
    pub origin: Point3,
    pub lower_left_corner: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        const ASPECT_RATIO: f64 = 16.0 / 9.0;

        const VIEWPORT_HEIGHT: f64 = 2.0;
        const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
        const FOCAL_LENGTH: f64 = 1.0;

        let origin: Point3 = Point3::default();
        let horizontal = Vec3::new(VIEWPORT_WIDTH as _, 0., 0.);
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
}
