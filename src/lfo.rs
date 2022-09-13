use crate::envelope;
use crate::oscillator::Osc;
use crate::Signal;

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
    // TODO: with envelope::Fixed, LFO not needed?
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

    pub fn with_envelope(mut self, e: impl envelope::Envelope + 'static) -> Self {
        self.envelope = Box::new(e);
        self
    }
}

impl Signal for ELFO {
    fn value_at(&self, t: f64, frequency: f64) -> f64 {
        self.envelope.value_at(t, 1.0, self.envelope.min()) * self.lfo.value_at(t, frequency)
    }
}
