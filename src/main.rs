mod pixel;
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

fn hit_sphere(center: &Point, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin - center;
    let a = ray.direction.length_squared();
    let half_b = oc.dot(&ray.direction);
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        -half_b - discriminant.sqrt() / a
    }
}

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
    let width = 3840;
    let height = (width as f64 / aspect_ratio) as usize;

    // World
    let mut world = HittableList::default();
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let horizontal = Point {
        x: viewport_width,
        y: 0.0,
        z: 0.0,
    };
    let vertical = Point {
        x: 0.0,
        y: viewport_height,
        z: 0.0,
    };
    let lower_left_corner = origin
        - horizontal / 2.0
        - vertical / 2.0
        - Point {
            x: 0.0,
            y: 0.0,
            z: focal_length,
        };
    dbg!(&lower_left_corner);
    let mut image = PPM::new(width, height);

    for h in 0..height - 1 {
        eprintln!("\rScanlines remaining: {}", h);
        for w in 0..width {
            let u = w as f64 / (width - 1) as f64;
            let v = h as f64 / (height - 1) as f64;
            let ray = Ray {
                origin,
                direction: lower_left_corner + (horizontal * u) + (vertical * v) - origin,
            };
            let p = image.mut_pixel(w, h);
            *p = ray_color(&ray, &world);
        }
    }

    image.write("out.ppm");
}
