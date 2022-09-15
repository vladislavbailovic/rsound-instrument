use super::*;
use note::*;
use simple::Simple;

pub struct Freq {
    source: Simple,
    detune: f64,
}

impl Signal for Freq {
    fn value_at(&self, t: f64, frequency: f64) -> f64 {
        let frequency = frequency + self.detune;
        self.source.value_at(t, frequency)
    }
}

impl Synth for Freq {}

impl Freq {
    pub fn square(by: f64) -> Self {
        Self {
            source: Simple::square(),
            detune: by,
        }
    }
}

pub struct Semitones {
    source: Simple,
    detune: i32,
}

impl Signal for Semitones {
    fn value_at(&self, t: f64, frequency: f64) -> f64 {
        self.source.value_at(t, frequency)
    }
}

impl Synth for Semitones {
    fn preprocess_note(&self, note: Note) -> Note {
        if let Note::Tone(pitch, octave, v) = note {
            let coct = octave as i32;
            let cpit = pitch as i32;
            let current = coct * note::PitchClass::max() + cpit;
            let max = note::Octave::max() * note::PitchClass::max();

            let next = current + self.detune;
            if next > max {
                return note;
            }
            let noct = next / note::PitchClass::max();
            if noct < note::Octave::min() {
                return note;
            }
            let npit = next - (noct * note::PitchClass::max());
            if npit < note::PitchClass::min() {
                return note;
            }

            if let Some(octave) = note::Octave::try_from_int(noct) {
                if let Some(pitch) = note::PitchClass::try_from_int(npit) {
                    Note::Tone(pitch, octave, v)
                } else {
                    note
                }
            } else {
                note
            }
        } else {
            note
        }
    }
}

impl Semitones {
    pub fn square(by: i32) -> Self {
        Self {
            source: Simple::square(),
            detune: by,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detune_1() {
        let src = Semitones::square(1);
        let note = note![A: C4, 1 / 16];

        if let Note::Tone(p, o, _) = src.preprocess_note(note) {
            assert_eq!(note::PitchClass::B, p, "expected pitch class");
            assert_eq!(note::Octave::C4, o, "expected octave");
        } else {
            assert!(false, "preprocess fail");
        }
    }

    #[test]
    fn detune_n1() {
        let src = Semitones::square(-1);
        let note = note![A: C4, 1 / 16];

        if let Note::Tone(p, o, _) = src.preprocess_note(note) {
            assert_eq!(note::PitchClass::Gis, p, "expected pitch class");
            assert_eq!(note::Octave::C4, o, "expected octave");
        } else {
            assert!(false, "preprocess fail");
        }
    }

    #[test]
    fn detune_2() {
        let src = Semitones::square(2);
        let note = note![A: C4, 1 / 16];

        if let Note::Tone(p, o, _) = src.preprocess_note(note) {
            assert_eq!(note::PitchClass::H, p, "expected pitch class");
            assert_eq!(note::Octave::C4, o, "expected octave");
        } else {
            assert!(false, "preprocess fail");
        }
    }

    #[test]
    fn detune_n2() {
        let src = Semitones::square(-2);
        let note = note![A: C4, 1 / 16];

        if let Note::Tone(p, o, _) = src.preprocess_note(note) {
            assert_eq!(note::PitchClass::G, p, "expected pitch class");
            assert_eq!(note::Octave::C4, o, "expected octave");
        } else {
            assert!(false, "preprocess fail");
        }
    }

    #[test]
    fn detune_3() {
        let src = Semitones::square(3);
        let note = note![A: C4, 1 / 16];

        if let Note::Tone(p, o, _) = src.preprocess_note(note) {
            assert_eq!(note::PitchClass::C, p, "expected pitch class");
            assert_eq!(note::Octave::C5, o, "expected octave");
        } else {
            assert!(false, "preprocess fail");
        }
    }

    #[test]
    fn detune_n3() {
        let src = Semitones::square(-3);
        let note = note![A: C4, 1 / 16];

        if let Note::Tone(p, o, _) = src.preprocess_note(note) {
            assert_eq!(note::PitchClass::Fis, p, "expected pitch class");
            assert_eq!(note::Octave::C4, o, "expected octave");
        } else {
            assert!(false, "preprocess fail");
        }
    }

    #[test]
    fn detune_12() {
        let src = Semitones::square(12);
        let note = note![A: C4, 1 / 16];

        if let Note::Tone(p, o, _) = src.preprocess_note(note) {
            assert_eq!(note::PitchClass::A, p, "expected pitch class");
            assert_eq!(note::Octave::C5, o, "expected octave");
        } else {
            assert!(false, "preprocess fail");
        }
    }

    #[test]
    fn detune_n12() {
        let src = Semitones::square(-12);
        let note = note![A: C4, 1 / 16];

        if let Note::Tone(p, o, _) = src.preprocess_note(note) {
            assert_eq!(note::PitchClass::A, p, "expected pitch class");
            assert_eq!(note::Octave::C3, o, "expected octave");
        } else {
            assert!(false, "preprocess fail");
        }
    }

    #[test]
    fn detune_13() {
        let src = Semitones::square(13);
        let note = note![A: C4, 1 / 16];

        if let Note::Tone(p, o, _) = src.preprocess_note(note) {
            assert_eq!(note::PitchClass::B, p, "expected pitch class");
            assert_eq!(note::Octave::C5, o, "expected octave");
        } else {
            assert!(false, "preprocess fail");
        }
    }

    #[test]
    fn detune_n13() {
        let src = Semitones::square(-13);
        let note = note![A: C4, 1 / 16];

        if let Note::Tone(p, o, _) = src.preprocess_note(note) {
            assert_eq!(note::PitchClass::Gis, p, "expected pitch class");
            assert_eq!(note::Octave::C3, o, "expected octave");
        } else {
            assert!(false, "preprocess fail");
        }
    }

    #[test]
    fn detune_plus_failure() {
        let src = Semitones::square(200);
        let note = note![A: C4, 1 / 16];

        if let Note::Tone(p, o, _) = src.preprocess_note(note) {
            assert_eq!(note::PitchClass::A, p, "expected pitch class");
            assert_eq!(note::Octave::C4, o, "expected octave");
        } else {
            assert!(false, "preprocess fail");
        }
    }

    #[test]
    fn detune_minus_failure() {
        let src = Semitones::square(-200);
        let note = note![A: C4, 1 / 16];

        if let Note::Tone(p, o, _) = src.preprocess_note(note) {
            assert_eq!(note::PitchClass::A, p, "expected pitch class");
            assert_eq!(note::Octave::C4, o, "expected octave");
        } else {
            assert!(false, "preprocess fail");
        }
    }
}
