use std::fmt;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::point::Point;
#[derive(Default, Clone)]
pub struct Pixel {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Pixel {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
    pub fn r(self) -> f64 {
        self.r
    }
    pub fn g(self) -> f64 {
        self.g
    }
    pub fn b(self) -> f64 {
        self.b
    }
}

impl Mul<f64> for Pixel {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Pixel {
            r: self.r * t,
            g: self.g * t,
            b: self.b * t,
        }
    }
}

impl Add for Pixel {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Pixel {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        }
    }
}

impl Add<Point> for Pixel {
    type Output = Self;
    fn add(self, rhs: Point) -> Self {
        Pixel {
            r: self.r + rhs.x,
            g: self.g + rhs.y,
            b: self.b + rhs.z,
        }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}",
            (255.999 * self.r) as u8,
            (255.999 * self.g) as u8,
            (255.999 * self.b) as u8
        )
    }
}
