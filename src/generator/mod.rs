pub mod chain;

use crate::oscillator::Oscillator;
use crate::SAMPLE_RATE;
use note::Note;

pub trait Generator {
    fn play(&self, bpm: f64, note: Note) -> Vec<f64>;
}

pub trait Signal {
    fn value_at(&self, t: f64, frequency: f64) -> f64;
}

pub trait Synth: Signal {}

impl<T> Generator for T
where
    T: Synth,
{
    fn play(&self, bpm: f64, note: Note) -> Vec<f64> {
        let duration = note.secs(bpm);
        let frequency = note.freq();

        // TODO: optimize this
        let sample_duration = (SAMPLE_RATE as f64 * duration).floor() as usize;
        let mut samples: Vec<f64> = vec![0.0; sample_duration];

        for i in 0..sample_duration {
            let t = i as f64 / SAMPLE_RATE as f64;
            if let Some(freq) = frequency {
                samples.push(self.value_at(t, freq));
            } else {
                samples.push(0.0)
            }
        }
        samples
    }
}

pub struct Simple {
    osc: Oscillator,
}

impl Default for Simple {
    fn default() -> Self {
        Self {
            osc: Oscillator::Sine,
        }
    }
}

impl Signal for Simple {
    fn value_at(&self, t: f64, frequency: f64) -> f64 {
        self.osc.get(frequency).at(t)
    }
}

impl Synth for Simple {}

impl Simple {
    pub fn square() -> Self {
        Self {
            osc: Oscillator::Square,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synth_benchmark() {
        use note::*;
        use std::time::Instant;

        let osc = Simple::default();

        let start = Instant::now();
        osc.play(1.5, note![A: C0, 1/1]);
        eprintln!("duration: {}ms", start.elapsed().as_millis());
    }
}
