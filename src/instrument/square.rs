use crate::instrument::Instrument;

pub struct Square;
impl Instrument for Square {
    fn sample(&self, freq: f32, t: f32) -> f32 {
        if (2.0 * std::f32::consts::PI * freq * t).sin() >= 0.0 { 1.0 } else { -1.0 }
    }
}
