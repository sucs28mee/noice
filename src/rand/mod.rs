use std::ops::Range;
use std::time;
use std::time::{Duration, SystemTime};

pub trait Rng {
    fn gen(&mut self) -> u64;
    fn seed(&self) -> u64;
    fn gen_range(&mut self, bounds: Range<u64>) -> u64 {
        (self.gen() % (bounds.end - bounds.start)) + bounds.start
    }
    fn gen_uniform(&mut self) -> f64 {
        self.gen() as f64 / u64::MAX as f64
    }
}

pub struct Pcg {
    state: u64,
    mul: u64,
    inc: u64,
}

impl Pcg {
    pub fn new(state: u64, mul: u64, inc: u64) -> Self {
        Self { state, mul, inc }
    }

    fn advance(&mut self) -> u64 {
        let x = self.state;
        self.state = self.state.wrapping_mul(self.mul).wrapping_add(self.inc);
        x
    }
}

impl Rng for Pcg {
    fn gen(&mut self) -> u64 {
        let mut x = self.advance();
        let rot = (x >> 59) as u32;

        // XSH
        let high_bits = (x >> 32) as u32;
        let low_bits = x as u32;
        x = (low_bits ^ high_bits) as u64;

        // RR-RR
        let x_low = x.rotate_right(rot);
        let x_high = high_bits.rotate_right((x & 31) as u32);
        (x_high as u64) << 32 | x_low
    }

    fn seed(&self) -> u64 {
        self.state
    }
}

impl Default for Pcg {
    fn default() -> Self {
        Self {
            state: SystemTime::now()
                .duration_since(time::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(0))
                .as_nanos() as u64,
            mul: 6364136223846793005_u64,
            inc: 1442695040888963407_u64,
        }
    }
}
