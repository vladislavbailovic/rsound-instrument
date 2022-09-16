pub mod lfo;
pub mod oscillator;

pub mod generator;
use generator::*;

pub mod envelope;
use envelope::*;

use note::Note;

pub const SAMPLE_RATE: i32 = 44100;

pub struct Instrument {
    generator: Box<dyn Generator>,
    envelope: Box<dyn Envelope>,
}

impl Instrument {
    pub fn new<T, U>(generator: T, envelope: U) -> Self
    where
        T: Generator + 'static,
        U: Envelope + 'static,
    {
        Self {
            generator: Box::new(generator),
            envelope: Box::new(envelope),
        }
    }

    pub fn play(&self, bpm: f64, note: Note, volume: f64) -> Vec<f64> {
        let duration = note.secs(bpm);
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

#[derive(Default)]
pub struct Rack {
    instruments: Vec<(Instrument, f64)>,
}

impl Rack {
    pub fn add(&mut self, i: Instrument) {
        self.instruments.push((i, 1.0));
    }

    pub fn add_with_volume(&mut self, i: Instrument, volume: f64) {
        self.instruments.push((i, volume));
    }

    pub fn play(&self, bpm: f64, note: Note, volume: f64) -> Vec<f64> {
        let duration = note.secs(bpm);
        let sample_duration = (SAMPLE_RATE as f64 * duration).floor() as usize;
        /*
        let instrs = self.instruments.len();
        let mut samples = vec![vec![0.0; instrs]; sample_duration];
        self.instruments.iter().enumerate().for_each(|(i, x)| {
            x.0.play(bpm, note, volume * x.1)
                .iter()
                .enumerate()
                .for_each(|(s, &y)| samples[s][i] = y);
        });
        samples.iter().map(|x| x.iter().sum()).collect()
        */

        let mut result = vec![0.0; sample_duration];
        for instr in &self.instruments {
            for (i, val) in instr.0.play(bpm, note, volume * instr.1).iter().enumerate() {
                result[i] += val;
            }
        }
        result
    }
}
