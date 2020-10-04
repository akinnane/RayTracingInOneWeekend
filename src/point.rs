use rand::Rng;
use std::fmt;

use std::ops::{Add, Div, Mul, Neg, Sub};
#[derive(Default, Clone, Copy, Debug)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn random(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            x: rng.gen_range(min, max),
            y: rng.gen_range(min, max),
            z: rng.gen_range(min, max),
        }
    }

    pub fn random_unit_vector() -> Self {
        let mut rng = rand::thread_rng();
        let a: f64 = rng.gen_range(0.0, 2.0 * std::f64::consts::PI);
        let z: f64 = rng.gen_range(-1.0, 1.0);
        let r = (1.0 - z * z).sqrt();
        Point::new(r * a.cos(), r * a.sin(), z)
    }

    pub fn random_in_unit_sphere() -> Self {
        loop {
            let p = Self::random(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }
    pub fn y(&self) -> f64 {
        self.y
    }
    pub fn z(&self) -> f64 {
        self.z
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit_vector(self) -> Point {
        self / self.length()
    }

    pub fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // pub fn reflect(v: &Point, n: &Point) -> Point {
    //     *v - *n * v.dot(n) * 2.0
    // }

    pub fn reflect(self, n: &Point) -> Point {
        self - *n * self.dot(n) * 2.0
    }

    pub fn refract(&self, n: &Point, etai_over_etat: f64) -> Point {
        let cos_theta = (-self).dot(n);
        let r_out_perp = (self + &(n * cos_theta)) * etai_over_etat;
        let r_out_parallel = n * -(((1.0 - r_out_perp.length_squared()).abs()).sqrt());
        r_out_perp + r_out_parallel
    }
}

impl Div<f64> for Point {
    type Output = Self;
    fn div(self, t: f64) -> Self {
        self * (1.0 / t)
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, t: f64) -> Self {
        Point {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, t: f64) -> Point {
        Point {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }
}

impl Mul for Point {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Point {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add for &Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Point {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Neg for &Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl Sub<Point> for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub<&Point> for Point {
    type Output = Point;

    fn sub(self, other: &Point) -> Self::Output {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
