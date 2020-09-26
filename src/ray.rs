use crate::point::Point;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Point,
}

impl Ray {
    // pub fn origin(&self) -> Point {
    //     self.origin
    // }
    // pub fn direction(&self) -> Point {
    //     self.direction
    // }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }
}
