use crate::point::Point;
use crate::ray::Ray;

#[derive(Debug)]
pub struct Camera {
    pub origin: Point,
    pub lower_left_corner: Point,
    pub horizontal: Point,
    pub vertical: Point,
    pub lens_radius: f64,
    pub u: Point,
    pub v: Point,
    pub w: Point,
}

impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Point,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_width * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        let lens_radius = aperture / 2.0;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius,
            w,
            u,
            v,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = Point::random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
        }
    }
}
