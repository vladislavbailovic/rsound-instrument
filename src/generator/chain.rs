use super::*;

enum Operator {
    Add(Box<dyn Generator>),
    Sub(Box<dyn Generator>),
}

pub struct Chain {
    base: Oscillator,
    mods: Vec<Operator>,
}

impl Generator for Chain {
    fn sample_at(&self, t: f64, frequency: f64, volume: f64) -> f64 {
        volume
            * self
                .mods
                .iter()
                .fold(self.base.get(frequency).at(t), |val, x| match x {
                    Operator::Add(x) => val + x.sample_at(t, frequency, volume),
                    Operator::Sub(x) => val - x.sample_at(t, frequency, volume),
                })
    }
}

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
    pub fn add(&mut self, what: impl Generator + 'static) -> &mut Self {
        self.mods.push(Operator::Add(Box::new(what)));
        self
    }
    pub fn sub(&mut self, what: impl Generator + 'static) -> &mut Self {
        self.mods.push(Operator::Sub(Box::new(what)));
        self
    }
}
