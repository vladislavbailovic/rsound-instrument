use super::*;
use crate::oscillator::Oscillator;

enum Operator {
    Add(Box<dyn Signal>),
    Sub(Box<dyn Signal>),
}

pub struct Chain {
    base: Oscillator,
    mods: Vec<Operator>,
}

impl Signal for Chain {
    fn value_at(&self, t: f64, frequency: f64) -> f64 {
        self.mods
            .iter()
            .fold(self.base.get(frequency).at(t), |val, x| match x {
                Operator::Add(x) => val + x.value_at(t, frequency),
                Operator::Sub(x) => val - x.value_at(t, frequency),
            })
    }
}

impl Synth for Chain {}

impl Default for Chain {
    fn default() -> Self {
        Self {
            base: Oscillator::Sine,
            mods: Vec::new(),
        }
    }
}

impl Chain {
    pub fn new(base: Oscillator) -> Self {
        Self {
            base,
            mods: Vec::new(),
        }
    }
    pub fn add(&mut self, what: impl Signal + 'static) -> &mut Self {
        self.mods.push(Operator::Add(Box::new(what)));
        self
    }
    pub fn sub(&mut self, what: impl Signal + 'static) -> &mut Self {
        self.mods.push(Operator::Sub(Box::new(what)));
        self
    }
}
