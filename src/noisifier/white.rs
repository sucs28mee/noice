use super::Noisifier;
use crate::rand::RNG;

pub struct White;
impl Noisifier for White {
    fn noise(&self, _: f64, _: f64, rng: &mut impl RNG) -> f64 {
        rng.uniform()
    }
}
