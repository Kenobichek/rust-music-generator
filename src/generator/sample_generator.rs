use crate::audio::note::Note;
use crate::instrument::Instrument;

/// Generate raw audio samples for a sequence of notes with a given instrument
///
/// # Arguments
///
/// * `notes` - slice of Note structs (freq, duration)
/// * `instrument` - any struct implementing the Instrument trait
/// * `sample_rate` - samples per second, usually 44100
///
/// # Returns
///
/// A vector of f32 samples in the range [-1.0, 1.0]
pub fn generate_samples(
    notes: &[Note],
    instrument: &dyn Instrument,
    sample_rate: u32,
) -> Vec<f32> {
    let mut samples = Vec::new();

    for note in notes {
        let total_samples = (note.duration * sample_rate as f32) as usize;

        for n in 0..total_samples {
            let t = n as f32 / sample_rate as f32;
            samples.push(instrument.sample(note.freq, t));
        }
    }

    samples
}
