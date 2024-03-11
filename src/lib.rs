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
    /// Width of the image.
    pub width: u32,
    /// Height of the image.
    pub height: u32,
    /// Pixel coordinates are multiplied by this value then fed to the noise maker.
    pub frequency: f64,
    /// The layers of noise to apply onto the image.
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

                    row.push((255.0 * noise / octaves as f64).clamp(0.0, 255.0) as u8);
                }

                row
            })
            .flatten()
            .collect::<Box<[_]>>();

        let encoder = {
            // All this should probably be done with a builder but it is what it is.
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
    Smoothstep,
    Smootherstep,
}

impl Interpolation {
    pub fn interpolate(&self, a: f64, b: f64, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Interpolation::Linear => a + (b - a) * t,
            Interpolation::Smoothstep => a + (b - a) * (3.0 - t * 2.0) * t * t,
            Interpolation::Smootherstep => a + (b - a) * t * t * t * (t * (6.0 * t - 15.0) + 10.0),
        }
    }
}
