use crate::instrument::{Instrument, Sine, Square, Saw};

pub fn get_instrument(name: &str) -> Box<dyn Instrument> {
    match name.to_lowercase().as_str() {
        "sine" => Box::new(Sine),
        "square" => Box::new(Square),
        "saw" => Box::new(Saw),
        _ => Box::new(Sine),
    }
}
