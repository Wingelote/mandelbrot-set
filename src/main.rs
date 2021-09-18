use clap::{self, App, Arg};
use image;
use num_complex::Complex;

#[derive(Clone, Copy)]
struct MandelbrotSet<'a> {
    max_iterations: u32,
    img_side: u32,
    x_min: f32,
    x_max: f32,
    y_min: f32,
    y_max: f32,
    ouput_name: &'a str
}

impl <'a> Default for MandelbrotSet <'a> {
    fn default() -> MandelbrotSet<'a> {
        MandelbrotSet {
            max_iterations: 250,
            img_side: 800u32,
            x_min: -2f32,
            x_max: 1f32,
            y_min: -1.5f32,
            y_max: 1.5f32,
            ouput_name: "mandelbrot"
        }
    }
}

impl MandelbrotSet<'_> {
    fn get_scale_x(self) -> f32 { (self.x_max - self.x_min) / self.img_side as f32 }
    fn get_scale_y(self) -> f32 { (self.y_max - self.y_min) / self.img_side as f32 }
}

fn main() {
    let matches = App::new("Mandelbrot set")
                    .version("1.0")
                    .author("Wingelote <wingelote@gmail.com>")
                    .about("Generate and draw the Mandelbrot set.")
                    .arg(Arg::with_name("output_name")
                        .short("o")
                        .long("output")
                        .required(false)
                        .takes_value(true))
                    .get_matches();

    let mandelbrot = MandelbrotSet{ ..Default::default() };
    let scale_x = mandelbrot.get_scale_x();
    let scale_y = mandelbrot.get_scale_y();
 
    let mut img_buf = image::ImageBuffer::new(mandelbrot.img_side, mandelbrot.img_side);
 
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let c = Complex::new(
            mandelbrot.x_min + x as f32 * scale_x,
            mandelbrot.y_min + y as f32 * scale_y
        );

        let mut z = Complex::new(0f32, 0f32);
        let mut i = 0;

        for t in 0..mandelbrot.max_iterations {
            if z.norm() > 2.0 { break; }
            z = z * z + c;
            i = t;
        }
 
        *pixel = image::Luma([i as u8]);
    }
 
    img_buf.save(matches.value_of("output_name").unwrap_or(format!("{}.png", mandelbrot.ouput_name).as_str())).unwrap();
}