pub mod lfo;
pub mod oscillator;

pub mod generator;
use generator::*;

pub mod envelope;
use envelope::*;

use note::Note;

const SAMPLE_RATE: i32 = 44100;

pub struct Instrument<T, U>
where
    T: Generator,
    U: Envelope,
{
    generator: T,
    envelope: U,
}

impl<T, U> Instrument<T, U>
where
    T: Generator,
    U: Envelope,
{
    pub fn new(generator: T, envelope: U) -> Self {
        Self {
            generator,
            envelope,
        }
    }

    pub fn play(&self, bpm: f64, note: Note, volume: f64) -> Vec<f64> {
        let duration = note.secs(bpm as f32) as f64;
        self.generator
            .play(bpm, note)
            .iter()
            .enumerate()
            .map(|(i, x)| {
                let env = self
                    .envelope
                    .value_at(i as f64 / SAMPLE_RATE as f64, volume, duration);
                env * x
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
