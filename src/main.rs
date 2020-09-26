use std::fmt;

#[derive(Default, Clone)]
struct Pixel<T> {
    r: T,
    g: T,
    b: T,
}

impl fmt::Display for Pixel<u8> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.r, self.g, self.b)
    }
}



#[derive(Default)]
struct PPM {
    width: usize,
    height: usize,
    pixels: Vec<Pixel<u8>>,
}

impl PPM {
    fn new(width: usize, height: usize) -> Self {
        Self{
            width,
            height,
            pixels: vec![Pixel::<u8>::default(); width * height]
        }
    }


    fn mut_pixel(&mut self, width: usize, height: usize) -> &mut Pixel<u8> {
        self.pixels.get_mut(height * self.width + width).unwrap()
    }

    fn pixel(&self, width: usize, height: usize) -> &Pixel<u8> {
        self.pixels.get(height * self.width + width).unwrap()
    }
}

impl fmt::Display for PPM {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "P3\n{} {}\n255\n", self.width, self.height);
        for h in (0..self.height).rev() {
            for w in 0..self.width {
                write!(f,"{}\n", self.pixel(w,h));
            }
//            writeln!(f);
        }
        write!(f, "")
    }
}

fn main() {
    // Image
    let width = 256;
    let height = 256;

    let mut image = PPM::new(width, height);


    for h in (0..height-1).rev(){
        eprintln!("\rScanlines remaining: {}", h);
        for w in 0..width{
            let p = image.mut_pixel(w,h);
            p.r = (255.999 * (w as f64 / (width  -1) as f64)) as u8;
            p.g = (255.999 * (h as f64 / (height -1) as f64)) as u8;
            p.b = (255.999 * 0.25) as u8;
        }
    }

    println!("{}", image);
    // Render
}
