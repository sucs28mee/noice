use std::ops::Add;

use noice::{Interpolation, Noisifier, Perlin};

fn main() {
    let count = 10000;
    let average = (0..count).map(|x| noice::prng(x)).fold(0.0, Add::add) / count as f64;
    println!("{average}");
    Perlin::new(Interpolation::Cubic)
        .gen_image(1000, 1000, 10)
        .save("examples/noise.png")
        .unwrap();
}
