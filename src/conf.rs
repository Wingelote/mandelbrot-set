pub struct MandelbrotSetConf <'a> {
    pub max_iterations: u32,
    pub img_side: u32,
    pub ouput_name: &'a str
}

impl <'a> Default for MandelbrotSetConf <'a> {
    fn default() -> MandelbrotSetConf<'a> {
        Self {
            max_iterations: 250u32,
            img_side: 800u32,
            ouput_name: "mandelbrot.png"
        }
    }
}

impl MandelbrotSetConf<'_> {
    pub fn new(output: Option<&str>) -> MandelbrotSetConf {
        if let Some(ouput_name) = output {
            MandelbrotSetConf{ouput_name, ..MandelbrotSetConf::default()}
        } else {
            MandelbrotSetConf{..MandelbrotSetConf::default()}
        }
    }
}
