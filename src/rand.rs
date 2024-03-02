use std::{
    ops::{Add, Bound, RangeBounds, Sub},
    time::{self, Duration, SystemTime},
};

// use cast::From;
use min_max_traits::{Max, Min};
use num_traits::{AsPrimitive, ToPrimitive};

pub fn time_seed() -> u64 {
    SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_nanos() as u64
}

pub trait RNG {
    fn random(&mut self) -> u64;
    fn seed(&self) -> u64;
    fn with_seed(seed: u64) -> Self;

    /// Returns an [`f64`]` in range 0..1
    fn uniform(&mut self) -> f64 {
        self.random() as f64 / u64::MAX as f64
    }

    /// # Panics
    /// If the start bound is higher than the end bound.
    fn range<T>(&mut self, bounds: impl RangeBounds<T>) -> T
    where
        T: AsPrimitive<f64>
            + ToPrimitive
            + Min
            + Max
            + Sub<Output = T>
            + Add<Output = T>
            + PartialOrd
            + Copy,
        f64: AsPrimitive<T>,
    {
        let start = match bounds.start_bound() {
            Bound::Included(start) => *start,
            Bound::Excluded(start) => *start,
            Bound::Unbounded => T::MIN,
        };

        let end = match bounds.end_bound() {
            Bound::Included(end) => *end,
            Bound::Excluded(end) => *end,
            Bound::Unbounded => T::MAX,
        };

        if start > end {
            panic!("start bound exceeded the end bound")
        }

        start + (self.uniform() * (end - start).as_()).as_()
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PCG {
    state: u64,
    mul: u64,
    inc: u64,
}

impl PCG {
    pub fn new(state: u64, mul: u64, inc: u64) -> Self {
        Self { state, mul, inc }
    }

    fn advance(&mut self) -> u64 {
        let x = self.state;
        self.state = self.state.wrapping_mul(self.mul).wrapping_add(self.inc);
        x
    }
}

impl RNG for PCG {
    fn random(&mut self) -> u64 {
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

    fn with_seed(seed: u64) -> Self {
        Self {
            state: seed,
            ..Default::default()
        }
    }

    fn seed(&self) -> u64 {
        self.state
    }
}

impl Default for PCG {
    fn default() -> Self {
        Self {
            state: time_seed(),
            mul: 6364136223846793005_u64,
            inc: 1442695040888963407_u64,
        }
    }
}
