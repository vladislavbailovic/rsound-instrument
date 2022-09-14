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

pub struct Rack<T: Generator, U: Envelope> {
    instruments: Vec<Instrument<T, U>>
}
impl<T: Generator, U: Envelope> Rack<T, U> {
    pub fn new() -> Self {
        Self{ instruments: Vec::new() }
    }

    pub fn add(&mut self, i: Instrument<T, U>) {
        self.instruments.push(i);
    }

    pub fn play(&self, bpm: f64, note: Note, volume: f64) -> Vec<f64> {
        let instrs = self.instruments.len();
        let duration = note.secs(bpm);
        let sample_duration = (SAMPLE_RATE as f64 * duration).floor() as usize;
        let mut samples = vec![vec![0.0; instrs]; sample_duration];
        self.instruments.iter().enumerate()
            .for_each(|(i, x)| {
                x.play(bpm, note, volume).iter().enumerate().for_each(|(s, &y)| samples[s][i] = y);
            });
        samples.iter().map(|x| x.iter().sum()).collect()
    }
}
