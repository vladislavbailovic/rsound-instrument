use instrument::oscillator::*;
use instrument::*;
use note::*;
use rsound_output::{Buffer, FileWriter, OutputRenderer, Writer};

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

// Play with `ffplay -autoexit -f f32le -ar 44100 -ac 1 foo.pcm`
#[cfg(feature = "rsound-output")]
fn main() -> std::io::Result<()> {
    let envelope = envelope::ASR::new(0.015, 0.07);
    // let envelope = envelope::Fixed{};
    let mut chain = generator::chain::Chain::new(Oscillator::Square);
    let elfo = lfo::ELFO::triangle(31.0).with_envelope(envelope::ASR::new(0.0, 0.15));
    chain.add(lfo::LFO::sine(12.0));
    chain.sub(lfo::LFO::triangle(131.0));
    chain.sub(elfo);
    let synth = Instrument::new(chain, envelope);
    let sound = synth.play(90.0, note![A: C4, 1 / 1], 1.0);

    let renderer = PcmRenderer::new(&sound);
    let w = FileWriter::new("foo.pcm");
    w.write(renderer)?;

    Ok(())
}
