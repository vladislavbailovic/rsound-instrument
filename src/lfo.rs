use crate::envelope;
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

    pub fn with_envelope(mut self, e: impl envelope::Envelope + 'static) -> Self {
        self.envelope = Box::new(e);
        self
    }
}

impl Generator for ELFO {
    fn sample_at(&self, t: f64, frequency: f64, volume: f64) -> f64 {
        self.envelope.value_at(t, volume, self.envelope.min())
            * self.lfo.sample_at(t, frequency, volume)
    }
}
