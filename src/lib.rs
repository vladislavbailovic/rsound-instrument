pub mod generator;
use generator::*;

pub mod envelope;
use envelope::*;

const SAMPLE_RATE: i32 = 44100;

pub struct Instrument<T, U>
where
    T: Generator,
    U: Envelope,
{
    generator: T,
    envelope: U,
}

impl<T, U> Instrument<T, U>
where
    T: Generator,
    U: Envelope,
{
    pub fn new(generator: T, envelope: U) -> Self {
        Self {
            generator,
            envelope,
        }
    }

    pub fn play(&self, frequency: f64, duration: f64, volume: f64) -> Vec<f64> {
        let mut samples: Vec<f64> = Vec::new();
        let sample_duration = (SAMPLE_RATE as f64 * duration).floor() as usize;
        for i in 0..sample_duration {
            let t = i as f64 / SAMPLE_RATE as f64;
            let volume = self.envelope.value_at(t, volume, duration);
            samples.push(self.generator.sample_at(t, frequency, volume));
        }
        samples
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
