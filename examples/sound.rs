use instrument::oscillator::*;
use instrument::*;
use note::*;
use rsound_output::{Buffer, FileWriter, OutputRenderer, Writer};

// https://stackoverflow.com/a/12370312
// http://soundfile.sapp.org/doc/WaveFormat/
// https://doc.rust-lang.org/std/primitive.u8.html#method.to_le

struct WaveRenderer {
    pcm: PcmRenderer,
}

impl WaveRenderer {
    pub fn new(raw: &[f64]) -> Self {
        Self{ pcm: PcmRenderer::new(raw) }
    }
}

impl Buffer for WaveRenderer {
    fn get_buffer(&self) -> &[u8] {
        &self.pcm.buffer
    }
}

impl OutputRenderer for WaveRenderer {
    fn get_header(&self) -> Option<Vec<u8>> {
        let header = vec![0u8; 44];
        Some(header)
    }
    fn get_footer(&self) -> Option<Vec<u8>> {
        None
    }
}

// -------------------------------

struct PcmRenderer {
    buffer: Vec<u8>,
}

impl PcmRenderer {
    pub fn new(raw: &[f64]) -> Self {
        let buffer = raw
            .iter()
            .map(|x| x.to_le_bytes())
            .flatten()
            .collect::<Vec<u8>>();
        Self { buffer }
    }
}

impl Buffer for PcmRenderer {
    fn get_buffer(&self) -> &[u8] {
        &self.buffer
    }
}

impl OutputRenderer for PcmRenderer {
    fn get_header(&self) -> Option<Vec<u8>> {
        None
    }
    fn get_footer(&self) -> Option<Vec<u8>> {
        None
    }
}

fn sine(note: Note) -> Vec<f64> {
    let envelope = envelope::ASR::new(0.015, 0.07);
    let synth = Instrument::new(generator::simple::Simple::default(), envelope);
    synth.play(90.0, note, 1.0)
}

fn chain(note: Note) -> Vec<f64> {
    let envelope = envelope::ASR::new(0.015, 0.07);
    let mut chain = generator::chain::Chain::new(Oscillator::Square);
    let elfo = lfo::ELFO::triangle(31.0).with_envelope(envelope::ASR::new(0.0, 0.15));
    chain.add(lfo::LFO::sine(12.0));
    chain.sub(lfo::LFO::triangle(131.0));
    chain.sub(elfo);
    let synth = Instrument::new(chain, envelope);
    synth.play(90.0, note, 1.0)
}

fn detuned(note: Note) -> Vec<f64> {
    let e1 = envelope::ASR::new(0.05, 0.05);

    let mut rack = Rack::default();
    let s1 = Instrument::new(generator::simple::Simple::default(), e1);
    rack.add(s1);
    let s2 = Instrument::new(generator::detuned::Semitones::square(3), envelope::Fixed {});
    rack.add(s2);

    let s3 = Instrument::new(generator::detuned::Freq::square(13.0), envelope::Fixed {});
    rack.add_with_volume(s3, 0.5);
    let s4 = Instrument::new(generator::detuned::Freq::square(-12.0), envelope::Fixed {});
    rack.add_with_volume(s4, 0.5);

    rack.play(90.0, note, 1.0)
}

fn rack(note: Note) -> Vec<f64> {
    let e1 = envelope::ASR::new(0.0, 0.1);
    let e2 = envelope::ASR::new(0.1, 0.0);

    let mut rack = Rack::default();
    let s1 = Instrument::new(generator::simple::Simple::default(), e1);
    rack.add(s1);
    let s2 = Instrument::new(generator::simple::Simple::square(), e2);
    rack.add(s2);
    rack.play(90.0, note, 1.0)
}

// Play with `ffplay -autoexit -f f64le -ar 44100 -ac 1 -nodisp foo.pcm`
#[cfg(feature = "rsound-output")]
fn main() -> std::io::Result<()> {
    let sound = chain(note![A: C2, 1 / 2]);

    // let renderer = PcmRenderer::new(&sound);
    // let w = FileWriter::new("foo.pcm");
    let renderer = WaveRenderer::new(&sound);
    let w = FileWriter::new("foo.wav");
    w.write(renderer)?;

    Ok(())
}
