use crate::instrument::Instrument;

pub struct Sine;
impl Instrument for Sine {
    fn sample(&self, freq: f32, t: f32) -> f32 {
        (2.0 * std::f32::consts::PI * freq * t).sin()
    }
}
