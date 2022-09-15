pub mod chain;
pub mod detuned;
pub mod simple;

use crate::SAMPLE_RATE;
use note::Note;

pub trait Generator {
    fn play(&self, bpm: f64, note: Note) -> Vec<f64>;
}

pub trait Signal {
    fn value_at(&self, t: f64, frequency: f64) -> f64;
}

pub trait Synth: Signal {
    fn play_note(&self, bpm: f64, note: Note) -> Vec<f64> {
        let note = self.preprocess_note(note);
        let duration = note.secs(bpm);
        let frequency = if let Some(f) = note.freq() { f } else { 0.0 };

        let sample_duration = (SAMPLE_RATE as f64 * duration).floor() as usize;
        let samples: Vec<f64> = vec![0.0; sample_duration];
        if frequency == 0.0 {
            return samples;
        }

        samples
            .iter()
            .enumerate()
            .map(|(i, _)| {
                let t = i as f64 / SAMPLE_RATE as f64;
                self.value_at(t, frequency)
            })
            .collect()
    }

    fn preprocess_note(&self, note: Note) -> Note {
        note
    }
}

impl<T> Generator for T
where
    T: Synth,
{
    fn play(&self, bpm: f64, note: Note) -> Vec<f64> {
        let s: &dyn Synth = self;
        s.play_note(bpm, note)
    }
}
