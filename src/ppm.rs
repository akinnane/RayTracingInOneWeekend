use crate::pixel::Pixelu;
use crate::Pixel;
use show_image::{ImageData, ImageInfo};
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use rayon::prelude::*;

#[derive(Default, Clone)]
pub struct PPM {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Pixel>,
}

impl PPM {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Pixel::default(); width * height],
        }
    }

    pub fn mut_pixel(&mut self, width: usize, height: usize) -> &mut Pixel {
        self.pixels.get_mut(height * self.width + width).unwrap()
    }

    pub fn pixel(&self, width: usize, height: usize) -> &Pixel {
        self.pixels.get(height * self.width + width).unwrap()
    }

    pub fn write(&self, filename: &str) -> Result<usize, std::io::Error> {
        let mut f = File::create(filename)?;
        f.write(self.to_string().as_bytes())
    }
}

impl fmt::Display for PPM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P3\n{} {}\n255\n", self.width, self.height).ok();
        for h in (0..self.height).rev() {
            for w in 0..self.width {
                write!(f, "{}\n", self.pixel(w, h)).ok();
            }
        }
        write!(f, "")
    }
}

impl ImageData for &PPM {
    fn info(&self) -> Result<ImageInfo, String> {
        Ok(ImageInfo::rgb8(self.width, self.height))
    }

    fn data(self) -> Box<[u8]> {
        let mut v: Vec<u8> = vec![];
        for pixel in self.pixels.iter() {
            v.push(pixel.r_u8());
            v.push(pixel.g_u8());
            v.push(pixel.b_u8());
        }
        v.into_boxed_slice()
    }
}

#[derive(Default, Clone, Debug)]
pub struct PPMu {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>,
}

impl PPMu {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0_u8; width * height * 3],
        }
    }

    pub fn mut_pixel(&mut self, width: usize, height: usize) -> Pixelu {
        let start = height * self.width + width;
        let p = self.pixels.get_mut(start..start + 3).unwrap();
        dbg!(p.len());
        Pixelu { s: p }
    }


}

impl ImageData for &PPMu {
    fn info(&self) -> Result<ImageInfo, String> {
        Ok(ImageInfo::rgb8(self.width, self.height))
    }

    fn data(self) -> Box<[u8]> {
        let v = self.pixels.clone();
        v.into_boxed_slice()
    }
}
