mod audio;
mod instrument;

use crate::audio::note::Note;
use crate::audio::wav_writer::generate_wav;
use crate::instrument::sine::Sine;

fn main() {
    let melody = vec![
        Note { freq: 261.63, duration: 0.5 }, // C4
        Note { freq: 329.63, duration: 0.5 }, // E4
        Note { freq: 392.0, duration: 1.0 },  // G4
    ];

    generate_wav(&melody, &Sine, "melody.wav");
}
