mod perlin;
mod white;

pub use perlin::Perlin;
pub use white::White;

use std::{
    fs::File,
    io::{self},
    path::Path,
};

use crate::rand::{self, PCG, RNG};

#[cfg(feature = "image")]
pub struct GenerateInfo {
    pub frequency: f64,
    pub layers: u64,
    pub seed: u64,
}

#[cfg(feature = "image")]
impl Default for GenerateInfo {
    fn default() -> Self {
        Self {
            frequency: 0.1,
            layers: 1,
            seed: rand::time_seed(),
        }
    }
}

pub trait Noisifier {
    fn noise(&self, x: f64, y: f64, rng: &mut impl RNG) -> f64;

    /// Generates a `PNG` image at the specified location.
    #[cfg(feature = "image")]
    fn generate<P: AsRef<Path>>(
        &self,
        path: P,
        width: u32,
        height: u32,
        info: GenerateInfo,
    ) -> io::Result<()> {
        use png::{BitDepth, ColorType, Encoder};

        let mut rng = PCG::with_seed(info.seed);
        let buffer = (0..height)
            .map(|j| {
                let mut row = Vec::with_capacity(width as usize);
                for i in 0..width {
                    row.push(
                        ((1..=info.layers).fold(0.0, |acc, layer| {
                            let layer = layer as f64;
                            let layers = info.layers as f64;
                            acc + self.noise(
                                i as f64 * info.frequency / layer,
                                j as f64 * info.frequency / layer,
                                &mut rng,
                            ) / layers
                        }) * 255.0) as u8,
                    );
                }

                row.into_iter()
            })
            .flatten()
            .collect::<Box<[_]>>();

        let encoder = {
            let mut encoder = Encoder::new(File::create(path)?, width, height);
            encoder.set_color(ColorType::Grayscale);
            encoder.set_depth(BitDepth::Eight);
            encoder
        };

        encoder.write_header()?.write_image_data(&buffer)?;
        Ok(())
    }
}
