use std::convert::From;
use std::fmt;
use std::ops::{Add, AddAssign, Mul, MulAssign};

use crate::point::Point;
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
    pub fn r(self) -> f64 {
        self.r
    }
    pub fn g(self) -> f64 {
        self.g
    }
    pub fn b(self) -> f64 {
        self.b
    }

    pub fn r_u8(&self) -> u8 {
        Pixel::x_u8(self.r)
    }

    pub fn g_u8(&self) -> u8 {
        Pixel::x_u8(self.g)
    }
    pub fn b_u8(&self) -> u8 {
        Pixel::x_u8(self.b)
    }

    fn x_u8(x: f64) -> u8 {
        const LESS_THAN_ONE: f64 = 1.0 - f64::MIN;
        (255.999 * x.sqrt().clamp(0.0, LESS_THAN_ONE)) as u8
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

impl MulAssign<f64> for Pixel {
    fn mul_assign(&mut self, t: f64) {
        *self = Self {
            r: self.r * t,
            g: self.g * t,
            b: self.b * t,
        };
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

impl AddAssign for Pixel {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
        };
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
        write!(f, "{} {} {}", self.r_u8(), self.g_u8(), self.b_u8())
    }
}



#[derive(Debug)]
pub struct Pixelu<'a> {
    pub s: &'a mut [u8],
}

impl<'a> AddAssign<Pixel> for Pixelu<'a> {
    fn add_assign(&mut self, rhs: Pixel) {
        self.s[0] += (255.0 * rhs.r) as u8;
        self.s[1] += (255.0 * rhs.g) as u8;
        self.s[2] += (255.0 * rhs.b) as u8;
    }
}

impl<'a> MulAssign<f64> for Pixelu<'a> {
    fn mul_assign(&mut self, t: f64) {
        self.s[0] = (self.s[0] as f64 * t) as u8;
        self.s[1] = (self.s[1] as f64 * t) as u8;
        self.s[2] = (self.s[2] as f64 * t) as u8;
    }
}

// impl From<&'a mut [u8; 3]> for Pixelu<'a> {
//     fn from(p: &'a mut [u8; 3]) -> Pixelu {
//         let c: f64 = 1.0 / 255.0;
//         Pixelu {
//             slice: p
//         }
//     }
// }

// impl<'a> Pixelu<'a> {
//     pub fn new(r: f64, g: f64, b: f64) -> Self {
//         Self {  }
//     }


//     pub fn r(self) -> f64 {
//         self.slice[0]
//     }
//     pub fn g(self) -> f64 {
//         self.g
//     }
//     pub fn b(self) -> f64 {
//         self.b
//     }

//     pub fn r_u8(&self) -> u8 {
//         Pixelu::x_u8(self.r())
//     }

//     pub fn g_u8(&self) -> u8 {
//         Pixelu::x_u8(self.g())
//     }
//     pub fn b_u8(&self) -> u8 {
//         Pixelu::x_u8(self.b())
//     }

//     fn x_u8(x: f64) -> u8 {
//         const LESS_THAN_ONE: f64 = 1.0 - f64::MIN;
//         (255.999 * x.sqrt().clamp(0.0, LESS_THAN_ONE)) as u8
//     }
// }

// impl Mul<f64> for Pixelu {
//     type Output = Self;

//     fn mul(self, t: f64) -> Self {
//         Self {
//             r: self.r() * t,
//             g: self.g() * t,
//             b: self.b() * t,
//         }
//     }
// }

// impl Mul<Pixel> for Pixelu {
//     type Output = Self;

//     fn mul(self, rhs: Pixel) -> Self {
//         Self {
//             r: self.r * rhs.r,
//             g: self.g * rhs.g,
//             b: self.b * rhs.b,
//         }
//     }
// }

// impl MulAssign<f64> for Pixelu {
//     fn mul_assign(&mut self, t: f64) {
//         *self = Self {
//             r: self.r * t,
//             g: self.g * t,
//             b: self.b * t,
//         };
//     }
// }

// impl Add for Pixelu {
//     type Output = Self;
//     fn add(self, rhs: Self) -> Self {
//         Self {
//             r: self.r + rhs.r,
//             g: self.g + rhs.g,
//             b: self.b + rhs.b,
//         }
//     }
// }

// impl AddAssign for Pixelu {
//     fn add_assign(&mut self, rhs: Self) {
//         *self = Self {
//             r: self.r + rhs.r,
//             g: self.g + rhs.g,
//             b: self.b + rhs.b,
//         };
//     }
// }

// impl Add<Point> for Pixelu {
//     type Output = Self;
//     fn add(self, rhs: Point) -> Self {
//         Self {
//             r: self.r + rhs.x,
//             g: self.g + rhs.y,
//             b: self.b + rhs.z,
//         }
//     }
// }

// impl fmt::Display for Pixelu {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{} {} {}", self.r_u8(), self.g_u8(), self.b_u8())
//     }
// }

// // pub struct PixelSlice<'a> {
// //     pub slice: &'a [u8; 3]
// // }
