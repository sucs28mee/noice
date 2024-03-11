use crate::rand;
use crate::Interpolation;
use std::f64;

use super::NoiseMaker;

pub struct PerlinMaker {
    pub interpolation: Interpolation,
    pub seed: u64,
}

impl NoiseMaker for PerlinMaker {
    fn noise(&self, x: f64, y: f64) -> f64 {
        let (x_floor, y_floor) = (x.floor(), y.floor());
        let (sx, sy) = (x - x_floor, y - y_floor);

        let gradient = |cx, cy| {
            let rot = rand::hash(cx, cy, self.seed as f64) * f64::consts::TAU;
            rot.cos() * (x - cx) + rot.sin() * (y - cy)
        };

        0.5 + 0.5
            * self.interpolation.interpolate(
                self.interpolation.interpolate(
                    gradient(x_floor, y_floor),
                    gradient(x_floor + 1., y_floor),
                    sx,
                ),
                self.interpolation.interpolate(
                    gradient(x_floor, y_floor + 1.),
                    gradient(x_floor + 1., y_floor + 1.),
                    sx,
                ),
                sy,
            )
    }
}

impl Default for PerlinMaker {
    fn default() -> Self {
        Self {
            interpolation: Interpolation::Smoothstep,
            seed: rand::time_seed(),
        }
    }
}
