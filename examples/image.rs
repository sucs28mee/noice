use noice::noisifier::{GenerateInfo, Noisifier, Perlin, White};

const SAVE_ERROR: &str = "Couldn't save to image";

fn main() {
    Perlin::default()
        .generate("examples/perlin.png", 1000, 1000, GenerateInfo::default())
        .expect(SAVE_ERROR);

    White
        .generate("examples/white.png", 1000, 1000, GenerateInfo::default())
        .expect(SAVE_ERROR);
}
