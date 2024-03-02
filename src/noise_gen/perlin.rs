use std::f64::consts::TAU;

use crate::rand::{Pcg, Rng};
use crate::{Float, Interpolation, Noisifier};

pub struct Perlin {
    interpolation: Interpolation,
    rng: Box<dyn Rng>,
}

impl Perlin {
    pub fn new(interpolation: Interpolation, rng: Box<dyn Rng>) -> Self {
        Self { interpolation, rng }
    }
}

impl Noisifier for Perlin {
    fn noise(&mut self, x: Float, y: Float) -> Float {
        let (x_floor, y_floor) = (x.floor(), y.floor());
        let (sx, sy) = (x - x_floor, y - y_floor);
        let mut gradient = |cx, cy| {
            let value: Float = self.rng.gen_uniform() * TAU; //not sure if that's what it's supposed to be here, replace if necessary
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
            rng: Box::new(Pcg::default()),
        }
    }
}
