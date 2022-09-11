use std::f64::consts::PI;

pub trait Generator {
    fn sample_at(&self, t: f64, frequency: f64, volume: f64) -> f64;
}

#[derive(Default)]
pub struct Sine;

impl Generator for Sine {
    fn sample_at(&self, t: f64, frequency: f64, volume: f64) -> f64 {
        volume * (frequency * t * 2.0 * PI).sin()
    }
}

#[derive(Default)]
pub struct Square {
    sine: Sine,
}

impl Generator for Square {
    fn sample_at(&self, t: f64, frequency: f64, volume: f64) -> f64 {
        if self.sine.sample_at(t, frequency, 1.0) > 0.0 {
            volume
        } else {
            volume * -1.0
        }
    }
}

#[derive(Default)]
pub struct Triangle;

impl Generator for Triangle {
    fn sample_at(&self, t: f64, frequency: f64, volume: f64) -> f64 {
        (2.0 * volume / PI) * (frequency * t * 2.0 * PI).sin().asin()
    }
}

#[derive(Default)]
pub struct Saw;

impl Generator for Saw {
    fn sample_at(&self, t: f64, frequency: f64, volume: f64) -> f64 {
        volume * (2.0 / PI) * (frequency * PI * (t % (1.0 / frequency)) - (PI / 2.0))
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
