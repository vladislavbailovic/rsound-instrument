use crate::envelope;
use crate::oscillator::Osc;
use crate::Signal;

#[derive(Debug)]
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

impl Signal for LFO {
    fn value_at(&self, t: f64, _frequency: f64) -> f64 {
        self.shape.at(t)
    }
}

pub struct ELFO {
    lfo: LFO,
    envelope: Box<dyn envelope::Envelope>,
}

impl ELFO {
    pub fn sine(freq: f64) -> Self {
        Self {
            lfo: LFO::sine(freq),
            envelope: Box::new(envelope::Fixed {}),
        }
    }
    pub fn square(freq: f64) -> Self {
        Self {
            lfo: LFO::square(freq),
            envelope: Box::new(envelope::Fixed {}),
        }
    }
    pub fn triangle(freq: f64) -> Self {
        Self {
            lfo: LFO::triangle(freq),
            envelope: Box::new(envelope::Fixed {}),
        }
    }
    pub fn saw(freq: f64) -> Self {
        Self {
            lfo: LFO::saw(freq),
            envelope: Box::new(envelope::Fixed {}),
        }
    }

    pub fn with_env_box(mut self, e: Box<dyn envelope::Envelope>) -> Self {
        self.envelope = e;
        self
    }

    pub fn with_envelope(self, e: impl envelope::Envelope + 'static) -> Self {
        self.with_env_box(Box::new(e))
    }
}

impl Signal for ELFO {
    fn value_at(&self, t: f64, frequency: f64) -> f64 {
        let cycle_length = self.envelope.min();
        let diff = if t > cycle_length {
            t - ((t / cycle_length).floor() * cycle_length)
        } else {
            t
        };

        self.envelope.value_at(diff, 1.0) * self.lfo.value_at(t, frequency)
    }
}
