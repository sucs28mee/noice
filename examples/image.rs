use noice::noise_gen::{Perlin, White};
use noice::Noisifier;

const SAVE_ERROR: &str = "Couldn't save to image";

fn main() {
    Perlin::default()
        .gen_image(1000, 1000, 10)
        .save("examples/perlin.png")
        .expect(SAVE_ERROR);
    
    White::default()
        .gen_image(1000, 1000, 10)
        .save("examples/white.png")
        .expect(SAVE_ERROR);
}
