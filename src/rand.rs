use std::time::{self, Duration, SystemTime};

pub fn time_seed() -> u64 {
    SystemTime::now()
        .duration_since(time::UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(0))
        .as_millis() as u64
}

pub fn random(mut seed: u64) -> u64 {
    let rot = (seed >> 59) as u32;

    // XSH
    let high_bits = (seed >> 32) as u32;
    let low_bits = seed as u32;
    seed = (low_bits ^ high_bits) as u64;

    // RR-RR
    let x_low = seed.rotate_right(rot);
    let x_high = high_bits.rotate_right((seed & 31) as u32);
    (x_high as u64) << 32 | x_low
}

pub fn hash(x: f64, y: f64, seed: f64) -> f64 {
    let (mut x, mut y, mut seed) = (
        (x * 0.131).fract(),
        (y * 0.131).fract(),
        (seed * 0.131).fract(),
    );

    let a = 36.32;
    let dot = x * (seed + a) + y * (y + a) + seed * (x + a);
    x += dot;
    y += dot;
    seed += dot;

    ((x + y) * seed).fract()
}

pub fn uniform_float(seed: u64) -> f64 {
    random(seed) as f64 / u64::MAX as f64
}
