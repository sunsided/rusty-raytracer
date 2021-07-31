use crate::random::Random;
use crate::{Degrees, Point3, Ray, Vec3};

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f64,
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
    /// * `aperture`: The aperture size of the lens.
    /// * `focus_distance`: The focus distance of the lens.
    pub fn new(
        look_from: Point3,
        look_at: Point3,
        view_up: Vec3,
        vfov: Degrees,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = vfov.0.to_radians();
        let h = (theta * 0.5).tan();
        let viewport_height = 2. * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (look_from - look_at).as_unit_vector();
        let u = view_up.cross(&w).as_unit_vector();
        let v = w.cross(&u);

        let origin = look_from;
        let horizontal = focus_distance * viewport_width * u;
        let vertical = focus_distance * viewport_height * v;
        let lower_left_corner = origin - horizontal.half() - vertical.half() - focus_distance * w;

        Self {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            u,
            v,
            lens_radius: aperture * 0.5,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64, rng: &Random) -> Ray {
        let rd = self.lens_radius * Vec3::random_in_unit_disk(rng);
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            self.origin + offset,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        )
    }
}
