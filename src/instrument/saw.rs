use crate::instrument::Instrument;

pub struct Saw;
impl Instrument for Saw {
    fn sample(&self, freq: f32, t: f32) -> f32 {
        2.0 * (freq * t - (freq * t).floor()) - 1.0
    }
}
