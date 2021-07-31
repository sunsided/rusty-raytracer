mod color;
mod point3;
mod ray;
mod vec3;

pub use color::Color;
pub use point3::Point3;
pub use ray::Ray;
pub use vec3::Vec3;

use indicatif::{ProgressBar, ProgressStyle};
use std::fs::File;
use std::io::prelude::*;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - *center;
    let a = r.direction.len_squared();
    let half_b = oc.dot(&r.direction);
    let c = oc.len_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0. {
        // Indicate no hit.
        -1.
    } else {
        // Calculate the hit point along the ray.
        (-half_b - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    // TODO: Hard-coded for testing purposes.
    let t = hit_sphere(&Point3::new(0., 0., -1.), 0.5, r);
    if t > 0. {
        let n = (r.at(t) - Vec3::new(0., 0., -1.)).as_unit_vector();
        return 0.5 * Color::new(n.x() + 1., n.y() + 1., n.z() + 1.);
    }

    // A simple gradient function for the background.
    // The color is blended between blue and white depending on the ray's Y coordinate.
    let unit_direction = r.direction.as_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {
    // Set up the image.
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    // Set up the camera.
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin: Point3 = Point3::default();
    let horizontal = Vec3::new(VIEWPORT_WIDTH as _, 0., 0.);
    let vertical = Vec3::new(0., VIEWPORT_HEIGHT as _, 0.);
    let lower_left_corner =
        origin - horizontal.half() - vertical.half() - Vec3::new(0., 0., FOCAL_LENGTH);

    // Prepare progress bar.
    let bar = ProgressBar::new(IMAGE_HEIGHT as _);
    bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .progress_chars("##-"),
    );

    // Render.
    let mut file = File::create("test.ppm")?;
    writeln!(file, "P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT)?;

    for j in (0..IMAGE_HEIGHT).rev() {
        bar.inc(1);
        for i in 0..IMAGE_WIDTH {
            let u = (i as f64) / (IMAGE_WIDTH as f64 - 1.);
            let v = (j as f64) / (IMAGE_HEIGHT as f64 - 1.);

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let color: Color = ray_color(&r);
            write!(file, "{}", color.write_color())?;
        }
    }

    bar.finish();
    Ok(())
}
