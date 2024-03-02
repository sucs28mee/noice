pub mod rand;
pub mod noise_gen;

pub type Float = f64;

pub trait Noisifier {
    fn noise(&mut self, x: Float, y: Float) -> Float;

    #[cfg(feature = "image")]
    fn gen_image(&mut self, width: u32, height: u32, cell_size: u32) -> image::RgbImage {
        image::RgbImage::from_fn(width, height, |i, j| {
            let noise = self.noise(
                i as Float / cell_size as Float,
                j as Float / cell_size as Float,
            );

            [(noise * 255.0) as u8; 3].into()
        })
    }
}

pub enum Interpolation {
    Linear,
    Cubic,
}

impl Interpolation {
    pub fn interpolate(&self, a: Float, b: Float, t: Float) -> Float {
        match self {
            Interpolation::Linear => a + (b - a) * t,
            Interpolation::Cubic => (b - a) * (3.0 - t * 2.0) * t * t + a,
        }
    }
}
