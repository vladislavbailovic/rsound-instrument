use crate::oscillator::Osc;
use crate::Generator;

pub struct LFO {
    shape: Osc,
}

impl LFO {
    pub fn sine(freq: f64) -> Self {
        Self {
            shape: Osc::Sine(freq),
        }
    }
    pub fn square(freq: f64) -> Self {
        Self {
            shape: Osc::Square(freq),
        }
    }
    pub fn triangle(freq: f64) -> Self {
        Self {
            shape: Osc::Triangle(freq),
        }
    }
    pub fn saw(freq: f64) -> Self {
        Self {
            shape: Osc::Saw(freq),
        }
    }
}

impl Generator for LFO {
    fn sample_at(&self, t: f64, _frequency: f64, volume: f64) -> f64 {
        volume * self.shape.at(t)
    }
}
