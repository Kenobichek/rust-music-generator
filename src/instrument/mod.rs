pub mod sine;
pub mod square;
pub mod saw;
pub mod factory;

pub trait Instrument {
    fn sample(&self, freq: f32, t: f32) -> f32;
}

pub use sine::Sine;
pub use square::Square;
pub use saw::Saw;
pub use factory::get_instrument;
