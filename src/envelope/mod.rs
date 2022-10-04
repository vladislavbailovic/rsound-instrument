pub trait Envelope {
    fn value_at(&self, t: f64, volume: f64, duration: f64) -> f64;
    fn min(&self) -> f64;
}

pub struct Fixed;

impl Envelope for Fixed {
    fn value_at(&self, _t: f64, volume: f64, _duration: f64) -> f64 {
        volume
    }

    fn min(&self) -> f64 {
        0.0
    }
}

pub struct Relative {
    attack: f64,
    release: f64,
}

impl Relative {
    pub fn new(attack: f64, release: f64) -> Self {
        Self { attack, release }
    }
}

impl Envelope for Relative {
    fn value_at(&self, t: f64, volume: f64, duration: f64) -> f64 {
        if t < self.attack {
            return volume * (t / self.attack);
        }

        let minr = duration - self.release;
        if t > minr {
            let posr = duration - t;
            return volume * (posr / self.release);
        }

        volume
    }

    fn min(&self) -> f64 {
        self.attack + self.release
    }
}

pub struct DRelative {
    delay: f64,
    asr: Relative,
}

impl DRelative {
    pub fn new(delay: f64, attack: f64, release: f64) -> Self {
        let asr = Relative::new(attack, release);
        Self { delay, asr }
    }
}

impl Envelope for DRelative {
    fn value_at(&self, t: f64, volume: f64, duration: f64) -> f64 {
        if t < self.delay {
            0.0
        } else {
            self.asr.value_at(t, volume, duration)
        }
    }

    fn min(&self) -> f64 {
        self.delay + self.asr.min()
    }
}
