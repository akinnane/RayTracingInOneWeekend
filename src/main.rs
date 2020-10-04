#![feature(clamp)]
mod pixel;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use hittable::Hittable;
use ppm::PPM;

use crate::pixel::Pixel;
use crate::{pixel::PixelSlice, point::Point};
mod point;
mod ppm;

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

use pbr::ProgressBar;

fn ray_color(ray: &Ray, world: &HittableList, depth: usize) -> Pixel {
    let mut hit_record = HitRecord::default();
    if depth == 0 {
        return Pixel::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, f64::INFINITY, &mut hit_record) {
        let mut scattered = Ray::default();
        let mut attenuation = Pixel::default();
        if hit_record
            .material
            .scatter(ray, &hit_record, &mut attenuation, &mut scattered)
        {
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
            r: 0.4,
            g: 0.5,
            b: 1.0,
        } * t)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    //let width = 800;
    let width = 4000;
    let height = (width as f64 / aspect_ratio) as usize;
    let samples_per_pixel = 500;
    let max_depth = 100;

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
        Box::new(Material::Dielectric { ref_idx: 1.5 }),
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

    let mut image = PPM::new(width, height);

    let (handle, sender, shutdown_receiver) = image_thread(image.clone());

    image
        .pixels
        .par_chunks_mut(width * 3)
        .enumerate()
        .for_each_with(sender, |s, pixel_row| {
            let row_index = pixel_row.0;
            let mut rng = rand::thread_rng();
            for (column_index, pixel) in pixel_row.1.chunks_mut(3).enumerate() {
                let mut p = PixelSlice { s: pixel };
                for _sample in 0..=samples_per_pixel {
                    let u = (column_index as f64 + rng.gen_range(0.0, 1.0)) / (width - 1) as f64;

                    let v = (row_index as f64 + rng.gen_range(0.0, 1.0)) / (height - 1) as f64;

                    let direction =
                        camera.lower_left_corner + (camera.horizontal * u) + (camera.vertical * v)
                            - camera.origin;
                    let ray = Ray {
                        origin: camera.origin,
                        direction,
                    };
                    p += ray_color(&ray, &world, max_depth);
                }
                p *= 1.0 / samples_per_pixel as f64;
            }
            s.send((row_index, pixel_row.1.to_vec())).unwrap();
        });

    println!("Finished Rendering");
    shutdown_receiver.recv().unwrap();
    handle.join().unwrap()
}

type RowData = (usize, Vec<f64>);

fn image_thread(
    mut image: PPM,
) -> (
    thread::JoinHandle<()>,
    Sender<RowData>,
    Receiver<usize>,
) {
    let (sender, receiver): (Sender<RowData>, Receiver<RowData>) = channel();
    let (shutdown_sender, shutdown_reciever) = channel();
    let handle = thread::spawn(move || {
        let window = make_window("ray_tracing_in_one_weekend").unwrap();
        let mut pb = ProgressBar::new(image.height as u64);
        pb.format("╢▌▌░╟");
        let mut n = 0;
        loop {
            n += 1;
            if let Ok(inner) = receiver.recv() {
                let row_start: usize =
                    ((image.height * image.width) - (inner.0 + 1) * image.width) * 3;
                let row_end: usize = ((image.height * image.width) - (inner.0) * image.width) * 3;

                image
                    .pixels
                    .splice(row_start..row_end, inner.1.iter().cloned());
                if n % 10 == 0 {
                    window.set_image(&image.clone(), "image-001").unwrap();
                }
                pb.inc();
            } else {
                break;
            }
        }
        pb.finish_print("Finished Displaying Image");
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
