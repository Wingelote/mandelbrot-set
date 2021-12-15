use super::conf::MandelbrotSetConf;
use image::{ImageBuffer, Luma};
use num_complex::Complex;

#[derive(Clone, Copy)]
pub struct MandelbrotSet{
    pub x_min: f32,
    pub x_max: f32,
    pub y_min: f32,
    pub y_max: f32,
}

impl Default for MandelbrotSet {
    fn default() -> MandelbrotSet {
        MandelbrotSet {
            x_min: -2f32,
            x_max: 1f32,
            y_min: -1.5f32,
            y_max: 1.5f32,
        }
    }
}

impl MandelbrotSet {
    pub fn get_scale_x(self, img_side: u32) -> f32 { (self.x_max - self.x_min) / img_side as f32 }
    pub fn get_scale_y(self, img_side: u32) -> f32 { (self.y_max - self.y_min) / img_side as f32 }
    pub fn create(self, mandelbrot_conf: &MandelbrotSetConf, img_buf: &mut ImageBuffer<Luma<u8>, Vec<u8>>) {
        let scale_x = self.get_scale_x(mandelbrot_conf.img_side);
        let scale_y = self.get_scale_y(mandelbrot_conf.img_side);

        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
            let c = Complex::new(
                self.x_min + x as f32 * scale_x,
                self.y_min + y as f32 * scale_y
            );

            let mut z = Complex::new(0f32, 0f32);
            let mut i = 0;
    
            for t in 0..mandelbrot_conf.max_iterations {
                if z.norm() > 2.0 { break; }
                z = z * z + c;
                i = t;
            }

            *pixel = image::Luma([i as u8]);
        }
    }
}