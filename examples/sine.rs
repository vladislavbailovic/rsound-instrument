use instrument::*;

#[cfg(feature = "graph")]
use graph::svg::Renderer;
#[cfg(feature = "graph")]
use graph::writer::{FileWriter, Writer};
#[cfg(feature = "graph")]
use graph::{Block, Graph, Line};

#[cfg(feature = "graph")]
fn main() -> std::io::Result<()> {
    let envelope = envelope::ASR::new(0.015, 0.07);
    let synth = Instrument::new(generator::Sine::default(), envelope);
    let sound = synth.play(440.0, 0.1, 1.0);
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
    w.write(renderer, hits)?;

    Ok(())
}
