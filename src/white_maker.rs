use super::NoiseMaker;
use crate::rand::{self};

pub struct WhiteMaker {
    pub seed: u64,
}

impl NoiseMaker for WhiteMaker {
    fn noise(&self, x: f64, y: f64) -> f64 {
        rand::hash(x, y, self.seed as f64)
    }
}

impl Default for WhiteMaker {
    fn default() -> Self {
        Self {
            seed: rand::time_seed(),
        }
    }
}
