use image::{Rgb, RgbImage};
use noice::{Interpolation, Noisifier, Perlin};

fn main() {
    let (width, height) = (1000, 1000);
    // let noise = noice::gen(width, height, Perlin::default(), GenInfo::default());

    let perlin = Perlin::new(Interpolation::Cubic);
    let seed = std::time::UNIX_EPOCH.elapsed().unwrap().as_secs_f64() % 19238.23;
    println!("{seed}");
    RgbImage::from_fn(width as u32, height as u32, |i, j| {
        let noise = perlin.noise(i as f64 * 0.1, j as f64 * 0.1, seed);
        Rgb::from([(noise * 255.0) as u8; 3])
    })
    .save("test.png")
    .unwrap();
}
