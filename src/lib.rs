pub mod audio;
pub mod generator;
pub mod export;
pub mod instrument;

pub use audio::note::Note;
pub use generator::generate_samples;
pub use export::export_wav;
pub use instrument::{Instrument, Sine, Square, Saw, get_instrument};

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg(feature = "wasm")]
#[wasm_bindgen]
pub fn generate_samples_wasm(freqs: &[f32], durations: &[f32], wave_type: &str) -> Vec<f32> {
    let mut notes = Vec::new();
    for i in 0..freqs.len().min(durations.len()) {
        notes.push(Note { freq: freqs[i], duration: durations[i] });
    }
    let instrument = get_instrument(wave_type);
    generate_samples(&notes, &*instrument, 44100)
}
