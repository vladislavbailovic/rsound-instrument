pub mod chain;

use crate::oscillator::Oscillator;

pub trait Generator {
    fn sample_at(&self, t: f64, frequency: f64, volume: f64) -> f64;
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

impl Generator for Simple {
    fn sample_at(&self, t: f64, frequency: f64, volume: f64) -> f64 {
        volume * self.osc.get(frequency).at(t)
    }
}

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
    fn sine_values() {
        let osc = Simple::default();
        for t in 0..10 {
            eprintln!("{}: {}", t, osc.sample_at(t as f64, 440.0, 1.0));
        }
    }
}
