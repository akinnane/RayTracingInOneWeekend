#![feature(clamp)]
mod pixel;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use hittable::Hittable;
use ppm::PPMu;

use crate::pixel::Pixel;
use crate::pixel::Pixelu;
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

mod material;
use crate::material::Material;

use rand::Rng;
use rayon::prelude::*;
use show_image::{make_window, KeyCode};

use std::sync::mpsc::channel;
use std::thread;

fn ray_color(ray: &Ray, world: &HittableList, depth: usize) -> Pixel {
    let mut hit_record = HitRecord::default();
    if depth <= 0 {
        return Pixel::new(0.0, 0.0, 0.0);
    }
    if world.hit(ray, 0.001, f64::INFINITY, &mut hit_record) {
        let mut scattered = Ray::default();
        let mut attenuation = Pixel::default();
        if hit_record
            .material
            .scatter(ray, &hit_record, &mut attenuation, &mut scattered)
        {
            //dbg!(&scattered);
            let out = ray_color(&scattered, world, depth - 1) * attenuation;

            return out;
        }
        return Pixel::new(0.0, 0.0, 0.0);
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
    //let width = 800;
    let width = 800;
    let height = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 50;
    let max_depth = 10;

    // World
    let mut world = HittableList::default();

    world.add(Box::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Material::Lambertian {
            albedo: Pixel::new(0.8, 0.8, 0.0),
        }),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Material::Dielectric { ref_idx: 1.1 }),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        Box::new(Material::Dielectric { ref_idx: 1.5 }),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Material::Metal {
            albedo: Pixel::new(0.8, 0.6, 0.2),
            fuzz: 1.0,
        }),
    )));

    //world.add(Box::new(Sphere::new(Point::new(0.5, 0.0, -0.8), 0.4)));
    //world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    let camera = Camera::new();

    let mut image = PPMu::new(width, height);

    let (handle, sender, shutdown_receiver) = image_thread(image.clone());

    image
        .pixels
        .par_chunks_mut(width*3)
        .enumerate()
        .for_each_with(sender, |s, pixel_row| {
            let row_index = pixel_row.0;
            dbg!(&row_index);
            let mut rng = rand::thread_rng();
            for (column_index, pixel) in pixel_row.1.chunks_mut(3).enumerate() {
                let mut p = Pixelu{ s: pixel};
                //dbg!(&p);
                for _sample in 0..=samples_per_pixel {
                    let c = (column_index as f64 + rng.gen_range(0.0, 1.0)) / (width - 1) as f64;
                    let r = (row_index as f64 + rng.gen_range(0.0, 1.0)) / (height - 1) as f64;

                    let ray = Ray {
                        origin: camera.origin,
                        direction: camera.lower_left_corner
                            + (camera.horizontal * r)
                            + (camera.vertical * c)
                            - camera.origin,
                    };
                    //dbg!(&p);
                    p += ray_color(&ray, &world, max_depth);
                    //dbg!(&p);
                }
                //p *= 1.0 / samples_per_pixel as f64;
            }
            s.send((row_index.clone(), pixel_row.1.to_vec())).unwrap();
        });

    shutdown_receiver.recv().unwrap();
    handle.join().unwrap()
}

fn image_thread(
    mut image: PPMu,
) -> (
    thread::JoinHandle<()>,
    Sender<(usize, Vec<u8>)>,
    Receiver<usize>,
) {
    let (sender, receiver): (Sender<(usize, Vec<u8>)>, Receiver<(usize, Vec<u8>)>) =
        channel();
    let (shutdown_sender, shutdown_reciever) = channel();
    let handle = thread::spawn(move || {
        let window = make_window("ray_tracing_in_one_weekend").unwrap();

        loop {
            if let Ok(inner) = receiver.recv() {
                let row_start: usize = image.width * inner.0 * 3;
                let row_end: usize = image.width * inner.0 *3 + image.width * 3;
                //dbg!(inner.0, row_start, row_end);
                image
                    .pixels
                    .splice(row_start..row_end, inner.1.iter().cloned());
                window.set_image(&image.clone(), "image-001").unwrap();
            } else {
                break;
            }
        }
        //dbg!(&image);

        while let Ok(event) = window.wait_key(Duration::from_millis(1000)) {
            if let Some(event) = event {
                if event.key == KeyCode::Escape {
                    break;
                }
            }
        }

        show_image::stop().ok();
        shutdown_sender.send(1).unwrap();
    });
    (handle, sender, shutdown_reciever)
}
