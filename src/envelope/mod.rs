pub trait Envelope {
    fn value_at(&self, t: f64, volume: f64, duration: f64) -> f64;
    fn min(&self) -> f64;
}

pub trait Delayed {
    fn get_delay(&self) -> f64;
    fn get_inner(&self) -> &dyn Envelope;
}

impl<T> Envelope for T
where
    T: Delayed,
{
    fn value_at(&self, t: f64, volume: f64, duration: f64) -> f64 {
        if t < self.get_delay() {
            0.0
        } else {
            self.get_inner().value_at(t, volume, duration)
        }
    }

    fn min(&self) -> f64 {
        self.get_delay() + self.get_inner().min()
    }
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
    inner: Relative,
}

impl DRelative {
    pub fn new(delay: f64, attack: f64, release: f64) -> Self {
        let inner = Relative::new(attack, release);
        Self { delay, inner }
    }
}

impl Delayed for DRelative {
    fn get_delay(&self) -> f64 {
        self.delay
    }
    fn get_inner(&self) -> &dyn Envelope {
        &self.inner
    }
}

pub struct ASR {
    attack: f64,
    sustain: f64,
    release: f64,
}

impl ASR {
    pub fn new(attack: f64, sustain: f64, release: f64) -> Self {
        Self {
            attack,
            sustain,
            release,
        }
    }
}
impl Envelope for ASR {
    fn value_at(&self, t: f64, volume: f64, _duration: f64) -> f64 {
        if t < self.attack {
            return volume * (t / self.attack);
        }

        if t > self.sustain + self.release {
            let posr = (self.sustain + self.release) - t;
            return volume * (posr / self.release);
        }

        volume
    }

    fn min(&self) -> f64 {
        self.attack + self.sustain + self.release
    }
}

pub struct DASR {
    delay: f64,
    inner: ASR,
}
impl Delayed for DASR {
    fn get_delay(&self) -> f64 {
        self.delay
    }
    fn get_inner(&self) -> &dyn Envelope {
        &self.inner
    }
}
