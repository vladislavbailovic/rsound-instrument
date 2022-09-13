use std::f64::consts::PI;

#[derive(Debug)]
pub enum Oscillator {
    Sine,
    Square,
    Triangle,
    Saw,
}
impl Oscillator {
    pub fn get(&self, frequency: f64) -> Osc {
        match self {
            Self::Sine => Osc::Sine(frequency),
            Self::Square => Osc::Square(frequency),
            Self::Triangle => Osc::Triangle(frequency),
            Self::Saw => Osc::Saw(frequency),
        }
    }
}

#[derive(Debug)]
pub enum Osc {
    Sine(f64),
    Square(f64),
    Triangle(f64),
    Saw(f64),
}

impl Osc {
    pub fn at(&self, t: f64) -> f64 {
        match self {
            Self::Sine(frequency) => (frequency * t * 2.0 * PI).sin(),
            Self::Square(frequency) => {
                if (Self::Sine(*frequency)).at(t) > 0.0 {
                    1.0
                } else {
                    -1.0
                }
            }
            Self::Triangle(frequency) => (2.0 / PI) * (frequency * t * 2.0 * PI).sin().asin(),
            Self::Saw(frequency) => {
                (2.0 / PI) * (frequency * PI * (t % (1.0 / frequency)) - (PI / 2.0))
            }
        }
    }
}
