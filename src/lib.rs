pub mod rand;

mod perlin_maker;
mod white_maker;
mod worley_maker;

pub use perlin_maker::PerlinMaker;
pub use white_maker::WhiteMaker;
pub use worley_maker::WorleyMaker;

use std::{fs::File, io, path::Path};

#[cfg(feature = "image")]
pub struct ImageGenerateInfo {
    pub width: u32,
    pub height: u32,
    pub frequency: f64,
    pub octaves: u64,
}

#[cfg(feature = "image")]
impl Default for ImageGenerateInfo {
    fn default() -> Self {
        Self {
            frequency: 0.025,
            octaves: 1,
            width: 2 << 8,
            height: 2 << 8,
        }
    }
}

pub trait NoiseMaker {
    fn noise(&self, x: f64, y: f64) -> f64;

    /// Generates a `PNG` image at the specified location.
    #[cfg(feature = "image")]
    fn generate_image<P: AsRef<Path>>(&self, path: P, info: ImageGenerateInfo) -> io::Result<()> {
        let ImageGenerateInfo {
            frequency,
            octaves,
            width,
            height,
        } = info;

        let buffer = (0..height)
            .map(|j| {
                let mut row = Vec::with_capacity(width as usize);
                for i in 0..width {
                    let noise = (1..=info.octaves).fold(0.0, |acc, octave| {
                        let frequency = frequency / octave as f64;
                        acc + self.noise(i as f64 * frequency, j as f64 * frequency)
                    });

                    row.push((noise / octaves as f64 * 255.0) as u8);
                }

                row
            })
            .flatten()
            .collect::<Box<[_]>>();

        let encoder = {
            let mut encoder = png::Encoder::new(File::create(path)?, width, height);
            encoder.set_color(png::ColorType::Grayscale);
            encoder.set_depth(png::BitDepth::Eight);
            encoder
        };

        encoder.write_header()?.write_image_data(&buffer)?;
        Ok(())
    }
}

pub enum Interpolation {
    Linear,
    Cubic,
}

impl Interpolation {
    pub fn interpolate(&self, a: f64, b: f64, t: f64) -> f64 {
        match self {
            Interpolation::Linear => a + (b - a) * t,
            Interpolation::Cubic => a + (b - a) * (3.0 - t * 2.0) * t * t,
        }
    }
}
