use instrument::oscillator::*;
use instrument::*;
use note::*;

#[cfg(feature = "graph")]
use graph::svg::Renderer;
#[cfg(feature = "graph")]
use graph::writer::{FileWriter, ImageWriter};
#[cfg(feature = "graph")]
use graph::{Block, Graph, Line};

#[cfg(feature = "graph")]
fn main() -> std::io::Result<()> {
    let envelope = envelope::Relative::new(0.015, 0.07);
    // let envelope = envelope::Fixed{};
    let mut chain = generator::chain::Chain::new(Oscillator::Square);
    let elfo = lfo::ELFO::triangle(31.0).with_envelope(envelope::Relative::new(0.0, 0.15));
    chain.add(lfo::LFO::sine(12.0));
    chain.sub(lfo::LFO::triangle(131.0));
    chain.sub(elfo);
    let synth = Instrument::new(chain, envelope);
    let sound = synth.play(90.0, note![A: C4, 1 / 16], 1.0);

    let minimum = sound
        .iter()
        .filter_map(|&x| Some(x))
        .reduce(f64::min)
        .expect("there has to be minimum");
    let values: Vec<Block> = sound
        .iter()
        .step_by(10)
        .map(|y| Block::new(1.0, y + minimum.abs()))
        .collect();

    let hits = Line::new(&values);
    let w = FileWriter::new("foo.svg");
    let renderer = Renderer::new(&hits.size());
    w.write_image(renderer, hits)?;

    Ok(())
}
