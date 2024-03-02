use crate::rand::{Pcg, Rng};
use crate::{Float, Noisifier};

pub struct White {
    rng: Box<dyn Rng>,
}

impl White {
    pub fn new(rng: Box<dyn Rng>) -> Self {
        Self { rng }
    }
}

impl Noisifier for White {
    fn noise(&mut self, _: Float, _: Float) -> Float {
        self.rng.gen_uniform()
    }
}

impl Default for White {
    fn default() -> Self {
        Self {
            rng: Box::new(Pcg::default()),
        }
    }
}
