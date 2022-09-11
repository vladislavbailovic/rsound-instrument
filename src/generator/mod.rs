use std::f64::consts::PI;

pub trait Generator {
    fn sample_at(&self, t: f64, frequency: f64, volume: f64) -> f64;
}

pub struct Sine;

impl Generator for Sine {
    fn sample_at(&self, t: f64, frequency: f64, volume: f64) -> f64 {
        volume * (frequency * t * 2.0 * PI).sin()
    }
}

pub struct DoubleSine {
    sine: Sine,
    detune: Sine,
}
impl DoubleSine {
    pub fn new() -> Self {
        Self{ sine: Sine{}, detune: Sine{} }
    }
}
impl Generator for DoubleSine {
    fn sample_at(&self, t: f64, frequency: f64, volume: f64) -> f64 {
        volume * (
            self.sine.sample_at(t, frequency, 1.0) +
            self.detune.sample_at(t, frequency * 10.0, 1.0)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sine_values() {
        let osc = Sine {};
        for t in 0..10 {
            eprintln!("{}: {}", t, osc.sample_at(t as f64, 440.0, 1.0));
        }
    }
}
