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

pub struct ASR {
    attack: f64,
    release: f64,
}

impl ASR {
    pub fn new(attack: f64, release: f64) -> Self {
        Self { attack, release }
    }
}

impl Envelope for ASR {
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
