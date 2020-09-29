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
use show_image::{make_window, KeyCode};

fn ray_color(ray: &Ray, world: &HittableList, depth: usize) -> Pixel {
    let mut hit_record = HitRecord::default();
    if depth <= 0 {
        return Pixel::new(0.0, 0.0, 0.0);
    }
    if world.hit(ray, 0.001, f64::INFINITY, &mut hit_record) {
        let target = hit_record.point + hit_record.normal + Point::random_in_unit_sphere();
        return (ray_color(
            &Ray {
                origin: hit_record.point,
                direction: target - hit_record.point,
            },
            &world,
            depth - 1,
        )) * 0.5;
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
    let width = 800;
    let height = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 50;
    let max_depth = 25;

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
            dbg!(&row_index);
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
                    *pixel += ray_color(&ray, &world, max_depth);
                }
                *pixel *= 1.0 / samples_per_pixel as f64;
            }
        });

    let window = make_window("image").unwrap();
    window.set_image(&image, "image-001").unwrap();

    while let Ok(event) = window.wait_key(Duration::from_millis(1000)) {
        if let Some(event) = event {
            if event.key == KeyCode::Escape {
                break;
            }
        }
    }

    show_image::stop().ok();
}
