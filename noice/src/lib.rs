use std::mem;
use std::time;

pub type Float = f64;
pub struct GenInfo {
    seed: Float,
    resolution: (Float, Float),
}

impl Default for GenInfo {
    fn default() -> Self {
        Self {
            seed: prng(time::UNIX_EPOCH.elapsed().unwrap().as_secs_f64()),
            resolution: (10.0, 10.0),
        }
    }
}

pub fn gen(
    width: usize,
    height: usize,
    noisifier: impl Noisifier,
    info: GenInfo,
) -> Box<[Box<[Float]>]> {
    (0..height)
        .map(|j| {
            (0..width)
                .map(|i| {
                    noisifier.noise(
                        i as Float / info.resolution.0,
                        j as Float / info.resolution.1,
                        info.seed,
                    )
                })
                .collect()
        })
        .collect()
}

pub fn prng(seed: Float) -> Float {
    let mut seed = seed as u32;
    let rotation = mem::size_of::<u32>() / 2;

    seed = seed.wrapping_mul(3284157443);
    seed ^= seed << rotation | seed >> rotation;
    seed = seed.wrapping_mul(1911520717);

    (seed as f64) * 152338.34328
}

pub trait Noisifier {
    fn noise(&self, x: Float, y: Float, seed: Float) -> Float;
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
    interp: Interpolation,
}

impl Perlin {
    pub fn new(interp: Interpolation) -> Self {
        Self { interp }
    }
}

impl Noisifier for Perlin {
    fn noise(&self, x: Float, y: Float, seed: Float) -> Float {
        let (x_floor, y_floor) = (x.floor(), y.floor());
        let (sx, sy) = (x - x_floor, y - y_floor);
        let gradient = |cx, cy| {
            let value: Float = prng(cx * cy * seed) * 6.28;
            value.cos() * (x - cx) + value.sin() * (y - cy)
        };

        self.interp.interpolate(
            self.interp.interpolate(
                gradient(x_floor, y_floor),
                gradient(x_floor + 1.0, y_floor),
                sx,
            ),
            self.interp.interpolate(
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
            interp: Interpolation::Linear,
        }
    }
}
