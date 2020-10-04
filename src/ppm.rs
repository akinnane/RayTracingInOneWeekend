use show_image::{ImageData, ImageInfo};

#[derive(Default, Clone, Debug)]
pub struct PPM {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<f64>,
}

impl PPM {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0.0_f64; width * height * 3],
        }
    }
}

impl ImageData for &PPM {
    fn info(&self) -> Result<ImageInfo, String> {
        Ok(ImageInfo::rgb8(self.width, self.height))
    }

    fn data(self) -> Box<[u8]> {
        const LESS_THAN_ONE: f64 = 1.0 - f64::MIN;
        self.pixels
            .iter()
            .map(|p| (255.999 * p.sqrt().clamp(0.0, LESS_THAN_ONE)) as u8)
            .collect::<Vec<u8>>()
            .into_boxed_slice()
    }
}
