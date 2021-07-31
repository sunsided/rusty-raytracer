mod camera;
mod color;
mod hittable;
mod material;
mod point3;
mod random;
mod ray;
mod sphere;
mod vec3;

use camera::Camera;
use color::Color;
use hittable::{HitRecord, Hittable, HittableList};
use material::{Dielectric, Lambertian, Material, MaterialPtr, Metal};
use point3::Point3;
use random::Random;
use ray::Ray;
use sphere::Sphere;
use vec3::Vec3;

use indicatif::{ProgressBar, ProgressStyle};
use num_traits::Float;
use std::fs::File;
use std::io::prelude::*;
use std::sync::Arc;

pub struct Degrees(pub f64);

fn ray_color(ray: &Ray, world: &Box<dyn Hittable>, rng: &mut Random, depth: usize) -> Color {
    if depth <= 0 {
        return Color::default();
    }

    if let Some(hit) = world.hit(ray, 0.001, f64::infinity()) {
        if let Some(scattered) = hit.material.scatter(ray, &hit, rng) {
            return scattered.attenuation * ray_color(&scattered.ray, world, rng, depth - 1);
        }
    }

    // A simple gradient function for the background.
    // The color is blended between blue and white depending on the ray's Y coordinate.
    let unit_direction = ray.direction.as_unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1., 1., 1.) + t * Color::new(0.5, 0.7, 1.0)
}

fn random_scene(rng: &mut Random) -> HittableList {
    let mut world = HittableList::default();

    let ground_material: MaterialPtr =
        Arc::new(Box::new(Lambertian::new(Color::new(0.5, 0.5, 0.5), 1.0)));
    world.add(Box::new(Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.sample();
            let center = Point3::new(
                a as f64 + 0.9 * rng.sample(),
                0.2,
                b as f64 + 0.9 * rng.sample(),
            );

            if (center - Point3::new(4., 0.2, 0.)).len() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random(rng) * Color::random(rng);
                    let material: MaterialPtr = Arc::new(Box::new(Lambertian::new(albedo, 1.0)));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random(rng) * 0.5 + 0.5;
                    let fuzz = rng.sample() * 0.5;
                    let material: MaterialPtr = Arc::new(Box::new(Metal::new(albedo, fuzz)));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                } else {
                    // Glass
                    let material: MaterialPtr = Arc::new(Box::new(Dielectric::new(1.5)));
                    world.add(Box::new(Sphere::new(center, 0.2, material)));
                }
            }
        }
    }

    let material1: MaterialPtr = Arc::new(Box::new(Dielectric::new(1.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0., 1., 0.),
        1.0,
        material1,
    )));

    let material2: MaterialPtr = Arc::new(Box::new(Lambertian::new(Color::new(0.4, 0.2, 0.1), 1.)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4., 1., 0.),
        1.0,
        material2,
    )));

    let material3: MaterialPtr = Arc::new(Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)));
    world.add(Box::new(Sphere::new(
        Point3::new(4., 1., 0.),
        1.0,
        material3,
    )));

    world
}

fn main() -> std::io::Result<()> {
    let mut rng = Random::default();

    // Set up the image.
    const ASPECT_RATIO: f64 = 3.0 / 2.0;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 500;
    const MAX_RAY_DEPTH: usize = 50;
    const GAMMA: f64 = 1.8;

    // Set up the world.
    let world: Box<dyn Hittable> = Box::new(random_scene(&mut rng));

    // Set up the camera.
    const LOOK_FROM: Point3 = Point3::new(13., 2., 3.);
    const LOOK_AT: Point3 = Point3::new(0., 0., 0.);
    const VIEW_UP: Vec3 = Vec3::new(0., 1., 0.);
    const APERTURE: f64 = 0.1;
    let dist_to_focus = 10.;

    let camera = Camera::new(
        LOOK_FROM,
        LOOK_AT,
        VIEW_UP,
        Degrees(20.),
        ASPECT_RATIO,
        APERTURE,
        dist_to_focus,
    );

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
            let mut pixel_color = Color::default();
            for _ in 0..SAMPLES_PER_PIXEL {
                let u_rnd = rng.sample();
                let v_rnd = rng.sample();

                let u = (i as f64 + u_rnd) / (IMAGE_WIDTH as f64 - 1.);
                let v = (j as f64 + v_rnd) / (IMAGE_HEIGHT as f64 - 1.);

                let r = camera.get_ray(u, v, &mut rng);
                pixel_color += ray_color(&r, &world, &mut rng, MAX_RAY_DEPTH);
            }

            write!(
                file,
                "{}",
                pixel_color.write_color(SAMPLES_PER_PIXEL, GAMMA)
            )?;
        }
    }

    bar.finish();
    Ok(())
}
