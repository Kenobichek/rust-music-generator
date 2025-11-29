pub trait Instrument {
    fn sample(&self, freq: f32, t: f32) -> f32;
}

pub mod sine;
