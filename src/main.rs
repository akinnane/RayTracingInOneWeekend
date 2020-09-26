mod pixel;
use crate::pixel::Pixel;
mod ppm;
use crate::ppm::PPM;
mod point;
use crate::point::Point;

mod ray;
use crate::ray::Ray;

fn ray_color(ray: Ray) -> Pixel {
    let unit_direction = ray.direction.unit_vector();
    let t = 0.5 * (unit_direction.y + 1.0);
    (Pixel{r:1.0, g:1.0, b:1.0} * (1.0-t) )+ (Pixel{r:0.5, g:0.7, b:1.0} * t)
}

fn main() {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let width = 800;
    let height = (width as f64 / aspect_ratio) as usize;

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
    let lower_left_corner = origin - horizontal / 2.0 - vertical/2.0 - Point{x:0.0, y:0.0, z: focal_length};
    dbg!(&lower_left_corner);
    let mut image = PPM::new(width, height);

    for h in 0..height - 1 {
        eprintln!("\rScanlines remaining: {}", h);
        for w in 0..width {
            let u = w as f64 / (width -1) as f64;
            let v = h as f64 / (height -1) as f64;
            let ray = Ray{origin, direction: lower_left_corner + (horizontal*u) + (vertical*v) - origin};
            let p = image.mut_pixel(w, h);
            *p = ray_color(ray);
        }
    }

    println!("{}", image);
    // Render
}
