use std::ops::{Add, AddAssign, Mul, MulAssign};
use rand::Rng;

#[derive(Default, Clone, Debug, Copy)]
pub struct Pixel {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Pixel {
    pub fn new(r: f64, g: f64, b: f64) -> Self {
        Self { r, g, b }
    }
    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        Self {
            r: rng.gen_range(0.0, 1.0),
            g: rng.gen_range(0.0, 1.0),
            b: rng.gen_range(0.0, 1.0),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Self {
        let mut rng = rand::thread_rng();
        Self {
            r: rng.gen_range(min, max),
            g: rng.gen_range(min, max),
            b: rng.gen_range(min, max),
        }
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

impl Mul<Pixel> for Pixel {
    type Output = Self;

    fn mul(self, rhs: Pixel) -> Self {
        Pixel {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
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

#[derive(Debug)]
pub struct PixelSlice<'a> {
    pub s: &'a mut [f64],
}

impl<'a> AddAssign<Pixel> for PixelSlice<'a> {
    fn add_assign(&mut self, rhs: Pixel) {
        self.s[0] += rhs.r;
        self.s[1] += rhs.g;
        self.s[2] += rhs.b;
    }
}

impl<'a> MulAssign<f64> for PixelSlice<'a> {
    fn mul_assign(&mut self, t: f64) {
        self.s[0] *= t;
        self.s[1] *= t;
        self.s[2] *= t;
    }
}
