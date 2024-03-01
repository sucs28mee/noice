use std::ops::Add;

use noice::{Interpolation, Noisifier, Perlin};

fn main() {
    Perlin::new(Interpolation::Cubic)
        .gen_image(1000, 1000, 10)
        .save("examples/noise.png")
        .unwrap();
}
