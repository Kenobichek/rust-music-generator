use hound;
use crate::audio::note::Note;
use crate::instrument::Instrument;

pub fn generate_wav(notes: &[Note], instrument: &dyn Instrument, path: &str) {
    let sample_rate = 44100;
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, spec).unwrap();

    for note in notes {
        let total_samples = (note.duration * sample_rate as f32) as usize;
        for n in 0..total_samples {
            let t = n as f32 / sample_rate as f32;
            let sample = instrument.sample(note.freq, t);
            let sample_i16 = (sample * i16::MAX as f32) as i16;
            writer.write_sample(sample_i16).unwrap();
        }
    }

    writer.finalize().unwrap();
}