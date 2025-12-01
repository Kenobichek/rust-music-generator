use rust_music_generator::audio::note::Note;
use rust_music_generator::export::export_wav::generate_wav;
use rust_music_generator::instrument::Sine;

fn main() {
    let melody: Vec<Note> = vec![
        Note { freq: 261.63, duration: 0.5 }, // C4
        Note { freq: 261.63, duration: 0.5 }, // C4
        Note { freq: 261.63, duration: 0.5 }, // C4

        Note { freq: 329.63, duration: 0.5 }, // E4
        Note { freq: 392.0, duration: 1.0 },  // G4
    ];

    generate_wav(&melody, &Sine, "melody.wav");
}
