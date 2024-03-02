use crate::rand::RNG;
use crate::Intrp;
use std::{f64, mem};

use super::Noisifier;

pub struct Perlin {
    intrp: Intrp,
}

impl Perlin {
    pub fn new(intrp: Intrp) -> Self {
        Self { intrp }
    }
}

impl Noisifier for Perlin {
    fn noise(&self, x: f64, y: f64, rng: &mut impl RNG) -> f64 {
        let seed = rng.seed();
        let (x_floor, y_floor) = (x.floor(), y.floor());
        let (sx, sy) = (x - x_floor, y - y_floor);

        let gradient = |cx, cy| {
            let value = {
                let w = 8 * mem::size_of::<u64>();
                let s = w / 2;
                let (mut a, mut b) = (cx as u64, cy as u64);
                a = a.wrapping_mul(seed);

                b ^= a << s | a >> w - s;
                b = b.wrapping_mul(seed);

                a ^= b << s | b >> w - s;
                a.wrapping_mul(2048419325) as f64 * f64::consts::TAU
            };

            value.cos() * (x - cx) + value.sin() * (y - cy)
        };

        self.intrp.interpolate(
            self.intrp.interpolate(
                gradient(x_floor, y_floor),
                gradient(x_floor + 1., y_floor),
                sx,
            ),
            self.intrp.interpolate(
                gradient(x_floor, y_floor + 1.),
                gradient(x_floor + 1., y_floor + 1.),
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
            intrp: Intrp::Cubic,
        }
    }
}
