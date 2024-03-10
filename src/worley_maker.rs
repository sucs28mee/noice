use crate::rand;

use super::NoiseMaker;

pub struct WorleyMaker {
    pub seed: u64,
    pub ppc: u64,
    pub n: usize,
}

impl NoiseMaker for WorleyMaker {
    fn noise(&self, x: f64, y: f64) -> f64 {
        let (x_floor, y_floor) = (x.floor(), y.floor());
        let mut distances = (-1..=1)
            .map(|j| {
                (-1..=1).flat_map(move |i| {
                    (0..self.ppc).map(move |p| {
                        let (grid_x, grid_y) = (x_floor + i as f64, y_floor + j as f64);
                        let seed = (self.seed + p) as f64;

                        let (rand_x, rand_y) = (
                            (grid_x + rand::hash(grid_x, grid_y, seed)),
                            (grid_y + rand::hash(grid_x, grid_y, seed + 1.0)),
                        );

                        // Calculate distance from (x, y) to a random vector in the grid cell.
                        (x - rand_x).powi(2) + (y - rand_y).powi(2)
                    })
                })
            })
            .flatten()
            .collect::<Vec<_>>();

        distances.sort_by(|a, b| a.total_cmp(b));
        distances
            .into_iter()
            .take(self.n)
            .fold(0.0, |acc, x| acc + x.sqrt() / self.n as f64)
    }
}

impl Default for WorleyMaker {
    fn default() -> Self {
        Self {
            seed: rand::time_seed(),
            n: 1,
            ppc: 1,
        }
    }
}
