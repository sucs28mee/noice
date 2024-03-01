use std::mem;
use std::time;

pub type Float = f64;
pub struct GenInfo {
    seed: Float,
    cell_size: (Float, Float),
}

impl Default for GenInfo {
    fn default() -> Self {
        Self {
            seed: prng(time::UNIX_EPOCH.elapsed().unwrap().as_millis()),
            cell_size: (10.0, 10.0),
        }
    }
}

pub fn prng(mut seed: u128) -> Float {
    let rotation = mem::size_of::<u128>() / 2;

    seed ^= seed << rotation | seed >> rotation;
    ((seed as Float * 9340932.598394).sin() + 1.0) / 2.0
}

pub trait Noisifier {
    fn noise(&self, x: Float, y: Float, seed: u128) -> Float;

    #[cfg(feature = "image")]
    fn gen_image(&self, width: u32, height: u32, cell_size: u32) -> image::RgbImage {
        image::RgbImage::from_fn(width, height, |i, j| {
            let noise = self.noise(
                i as Float / cell_size as Float,
                j as Float / cell_size as Float,
                time::UNIX_EPOCH.elapsed().unwrap().as_millis(),
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

pub struct Perlin {
    interpolation: Interpolation,
}

impl Perlin {
    pub fn new(interpolation: Interpolation) -> Self {
        Self { interpolation }
    }
}

impl Noisifier for Perlin {
    fn noise(&self, x: Float, y: Float, seed: u128) -> Float {
        let (x_floor, y_floor) = (x.floor(), y.floor());
        let (sx, sy) = (x - x_floor, y - y_floor);
        let gradient = |cx, cy| {
            let value: Float = prng((cx * seed as Float + cy * seed as Float) as u128) * 6.28;
            value.cos() * (x - cx) + value.sin() * (y - cy)
        };

        self.interpolation.interpolate(
            self.interpolation.interpolate(
                gradient(x_floor, y_floor),
                gradient(x_floor + 1.0, y_floor),
                sx,
            ),
            self.interpolation.interpolate(
                gradient(x_floor, y_floor + 1.0),
                gradient(x_floor + 1.0, y_floor + 1.0),
                sx,
            ),
            sy,
        ) * 0.5
            + 0.5
    }
}

impl Default for Perlin {
    fn default() -> Self {
        Self {
            interpolation: Interpolation::Linear,
        }
    }
}
