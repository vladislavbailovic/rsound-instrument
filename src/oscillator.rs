use std::f64::consts::PI;

#[cfg(feature = "wasm")]
use serde::{Deserialize, Serialize};
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[cfg_attr(feature = "wasm", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq)]
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

impl From<i32> for Oscillator {
    fn from(x: i32) -> Self {
        match x {
            1 => Self::Square,
            2 => Self::Triangle,
            3 => Self::Saw,
            _ => Self::Sine,
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
