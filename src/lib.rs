pub mod noisifier;
pub mod rand;

pub type Float = f64;

pub enum Intrp {
    Linear,
    Cubic,
}

impl Intrp {
    pub fn interpolate(&self, a: Float, b: Float, t: Float) -> Float {
        match self {
            Intrp::Linear => a + (b - a) * t,
            Intrp::Cubic => (b - a) * (3.0 - t * 2.0) * t * t + a,
        }
    }
}
