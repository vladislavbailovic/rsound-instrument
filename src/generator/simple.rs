use super::*;
use crate::oscillator::Oscillator;

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
    pub fn new(osc: Oscillator) -> Self {
        Self { osc }
    }

    pub fn square() -> Self {
        Self::new(Oscillator::Square)
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
        osc.play(3.0, pause![1 / 1]);
        osc.play(3.0, note![A: C0, 1 / 1]);
        eprintln!("duration: {}ms", start.elapsed().as_millis());
    }
}
