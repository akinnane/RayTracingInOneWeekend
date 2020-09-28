#![feature(clamp)]
mod pixel;
use std::time::Duration;

use hittable::Hittable;

use crate::pixel::Pixel;
mod ppm;
use crate::ppm::PPM;
mod point;
use crate::point::Point;

mod ray;
use crate::ray::Ray;

mod hittable;
use crate::hittable::HitRecord;

mod hittable_list;
use crate::hittable_list::HittableList;

mod sphere;
use crate::sphere::Sphere;

mod camera;
use crate::camera::Camera;

use rand::Rng;
use rayon::prelude::*;
use show_image::make_window;

// fn hit_sphere(center: &Point, radius: f64, ray: &Ray) -> f64 {
//     let oc = ray.origin - center;
//     let a = ray.direction.length_squared();
//     let half_b = oc.dot(&ray.direction);
//     let c = oc.length_squared() - radius * radius;
//     let discriminant = half_b * half_b - a * c;
//     if discriminant < 0.0 {
//         -1.0
//     } else {
//         -half_b - discriminant.sqrt() / a
//     }
// }

fn ray_color(ray: &Ray, world: &HittableList) -> Pixel {
    let mut hit_record = HitRecord::default();
    if world.hit(ray, 0.0, f64::INFINITY, &mut hit_record) {
        return (Pixel::new(1.0, 1.0, 1.0) + hit_record.normal) * 0.5;
    }
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (Pixel {
        r: 1.0,
        g: 1.0,
        b: 1.0,
    } * (1.0 - t))
        + (Pixel {
            r: 0.5,
            g: 0.7,
            b: 1.0,
        } * t)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 1920;
    let height = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 50;

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point::new(-0.6, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.5, 0.0, -0.8), 0.4)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();

    let mut image = PPM::new(width, height);

    image
        .pixels
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|pixel_row| {
            let row_index = pixel_row.0;
            let mut rng = rand::thread_rng();
            for (column_index, pixel) in pixel_row.1.iter_mut().enumerate() {
                for _sample in 0..=samples_per_pixel {
                    let u = (column_index as f64 + rng.gen_range(0.0, 1.0)) / (width - 1) as f64;
                    let v = (row_index as f64 + rng.gen_range(0.0, 1.0)) / (height - 1) as f64;

                    let ray = Ray {
                        origin: camera.origin,
                        direction: camera.lower_left_corner
                            + (camera.horizontal * u)
                            + (camera.vertical * v)
                            - camera.origin,
                    };
                    *pixel += ray_color(&ray, &world);
                }
                *pixel *= 1.0 / samples_per_pixel as f64;
            }
        });

    let window = make_window("image").unwrap();
    window.set_image(&image, "image-001").unwrap();

    while let Ok(event) = window.wait_key(Duration::from_millis(1000)) {
        if let Some(_event) = event {
            break;
        }
    }

    show_image::stop().ok();
}
