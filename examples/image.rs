use noice::{ImageGenerateInfo, NoiseMaker, PerlinMaker, WhiteMaker, WorleyMaker};

fn main() {
    PerlinMaker::default()
        .generate_image("examples/perlin.png", ImageGenerateInfo::default())
        .expect("Couldn't save perlin image.");

    WhiteMaker::default()
        .generate_image("examples/white.png", ImageGenerateInfo::default())
        .expect("Couldn't save white image.");

    WorleyMaker::default()
        .generate_image("examples/worley.png", ImageGenerateInfo::default())
        .expect("Couldn't save worley image.");
}
