use std::f64::consts::PI;

pub trait Generator {
    fn sample_at(&self, t: f64, frequency: f64, volume: f64) -> f64;
}

struct Sine;

impl Generator for Sine {
    fn sample_at(&self, t: f64, frequency: f64, volume: f64) -> f64 {
        volume * (frequency * t * 2.0 * PI).sin()
    }
}
