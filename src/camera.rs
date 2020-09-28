use crate::point::Point;
//use crate::ray::Ray;

pub struct Camera {
    pub origin: Point,
    pub lower_left_corner: Point,
    pub horizontal: Point,
    pub vertical: Point,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;

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
        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    // pub fn get_ray(self, u: f64, v: f64) -> Ray {
    //     Ray {
    //         origin: self.origin,
    //         direction: self.lower_left_corner + self.horizontal * u + self.vertical * v
    //             - self.origin,
    //     }
    // }
}
