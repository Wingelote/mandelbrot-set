mod conf;
mod set;

use conf::MandelbrotSetConf;
use set::MandelbrotSet;
use clap::{self, App, Arg};

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


    let mandelbrot_conf = &MandelbrotSetConf::new(matches.value_of("output_name"));
    let mut img_buf = image::ImageBuffer::new(mandelbrot_conf.img_side, mandelbrot_conf.img_side);
    let m_set = MandelbrotSet::default();
    
    m_set.create(mandelbrot_conf, &mut img_buf);
    img_buf.save(mandelbrot_conf.ouput_name).unwrap();
}