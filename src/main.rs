mod camera;
mod color;
mod hittable;
mod point3;
mod ray;
mod sphere;
mod vec3;

pub use camera::Camera;
pub use color::Color;
pub use hittable::{HitRecord, Hittable, HittableList};
pub use point3::Point3;
pub use ray::Ray;
pub use sphere::Sphere;
pub use vec3::Vec3;

use indicatif::{ProgressBar, ProgressStyle};
use num_traits::Float;
use std::fs::File;
use std::io::prelude::*;

fn ray_color(r: &Ray, world: &Box<dyn Hittable>) -> Color {
    if let Some(hit) = world.hit(r, 0., f64::infinity()) {
        return 0.5 * (hit.normal + Color::new(1., 1., 1.));
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

    // Set up the world.
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(0., -100.5, -1.), 100.)));

    let world: Box<dyn Hittable> = Box::new(world);

    // Set up the camera.
    let camera = Camera::default();

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

            let r = camera.get_ray(u, v);

            let color: Color = ray_color(&r, &world);
            write!(file, "{}", color.write_color())?;
        }
    }

    bar.finish();
    Ok(())
}
