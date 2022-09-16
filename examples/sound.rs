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
        let buflen = self.pcm.buffer.len();
        let fsize = buflen + 44;
        let bits_per_sample = 64;

        let byte_rate = SAMPLE_RATE * 1 /* # channels */ * bits_per_sample / 8;
        let block_align = (1 /* # channels */ * bits_per_sample / 8) as u8;

        let mut header = vec![0u8; 44];

        header[0] = ('R' as u8).to_be();
        header[1] = ('I' as u8).to_be();
        header[2] = ('F' as u8).to_be();
        header[3] = ('F' as u8).to_be();

        header[4] = ((fsize & 0xff) as u8).to_be();
        header[5] = (((fsize >> 8) & 0xff) as u8).to_be();
        header[6] = (((fsize >> 16) & 0xff) as u8).to_be();
        header[7] = (((fsize >> 24) & 0xff) as u8).to_be();

        header[8] = ('W' as u8).to_be();
        header[9] = ('A' as u8).to_be();
        header[10] = ('V' as u8).to_be();
        header[11] = ('E' as u8).to_be();

        header[12] = ('f' as u8).to_be();
        header[13] = ('m' as u8).to_be();
        header[14] = ('t' as u8).to_be();
        header[15] = (' ' as u8).to_be();

        header[16] = 16u8.to_be(); //Subchunk1Size    16 for PCM
        header[17] = 0u8.to_be();
        header[18] = 0u8.to_be();
        header[19] = 0u8.to_be();

        header[20] = 1u8.to_be(); // AudioFormat      PCM = 1 (i.e. Linear quantization)
        header[21] = 0u8.to_be();

        header[22] = 1u8.to_be(); // NumChannels      Mono = 1, Stereo = 2, etc.
        header[23] = 0u8.to_be();

        header[24] = ((SAMPLE_RATE & 0xff) as u8).to_be(); // SampleRate       8000, 44100, etc.
        header[25] = (((SAMPLE_RATE >> 8) & 0xff) as u8).to_be();
        header[26] = (((SAMPLE_RATE >> 16) & 0xff) as u8).to_be();
        header[27] = (((SAMPLE_RATE >> 24) & 0xff) as u8).to_be();

        header[28] = ((byte_rate & 0xff) as u8).to_be(); // ByteRate         == SampleRate * NumChannels * BitsPerSample/8
        header[29] = (((byte_rate >> 8) & 0xff) as u8).to_be();
        header[30] = (((byte_rate >> 16) & 0xff) as u8).to_be();
        header[31] = (((byte_rate >> 24) & 0xff) as u8).to_be();

        header[32] = block_align.to_be(); // BlockAlign       == NumChannels * BitsPerSample/8
        header[33] = 0u8.to_be();

        header[34] = (bits_per_sample as u8).to_be(); // BitsPerSample    8 bits = 8, 16 bits = 16, etc.
        header[35] = 0u8.to_be();
        
        header[36] = ('d' as u8).to_be();
        header[37] = ('a' as u8).to_be();
        header[38] = ('t' as u8).to_be();
        header[39] = ('a' as u8).to_be();

        header[40] = ((buflen & 0xff) as u8).to_be(); // Subchunk2Size    == NumSamples * NumChannels * BitsPerSample/8
        header[41] = (((buflen >> 8) & 0xff) as u8).to_be();
        header[42] = (((buflen >> 16) & 0xff) as u8).to_be();
        header[43] = (((buflen >> 24) & 0xff) as u8).to_be();

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
    let sound = sine(note![A: C3, 1 / 2]);

    // let renderer = PcmRenderer::new(&sound);
    // let w = FileWriter::new("foo.pcm");
    let renderer = WaveRenderer::new(&sound);
    let w = FileWriter::new("foo.wav");
    w.write(renderer)?;

    Ok(())
}
