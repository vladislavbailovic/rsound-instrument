pub trait Envelope {
    fn value_at(&self, t: f64, volume: f64, duration: f64) -> f64 {
        volume
    }
}

struct ASR {
    attack: f64,
    release: f64,
}

impl Envelope for ASR {
    fn value_at(&self, t: f64, volume: f64, duration: f64) -> f64 {
        if t < self.attack {
            return volume * (t/self.attack);
        }

        let minr = duration - self.release;
        if t > minr {
            let posr = duration - t;
            return volume * (posr/self.release);
        }

        volume
    }
}
