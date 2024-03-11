use std::time::Instant;

use noice::{ImageGenerateInfo, Interpolation, NoiseMaker, PerlinMaker, WhiteMaker, WorleyMaker};

fn main() {
    PerlinMaker {
        interpolation: Interpolation::Smootherstep,
        ..Default::default()
    }
    .generate_image("examples/perlin.png", ImageGenerateInfo::default())
    .expect("Couldn't save perlin image.");

    WhiteMaker::default()
        .generate_image("examples/white.png", ImageGenerateInfo::default())
        .expect("Couldn't save white image.");

    let instant = Instant::now();
    WorleyMaker::default()
        .generate_image("examples/worley.png", ImageGenerateInfo::default())
        .expect("Couldn't save worley image.");
    println!("{:?}", instant.elapsed());
}
