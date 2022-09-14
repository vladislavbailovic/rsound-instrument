use instrument::*;
use note::*;

#[cfg(feature = "graph")]
use graph::svg::Renderer;
#[cfg(feature = "graph")]
use graph::writer::{FileWriter, Writer};
#[cfg(feature = "graph")]
use graph::{Block, Graph, Line};

#[cfg(feature = "graph")]
fn main() -> std::io::Result<()> {
    let envelope = envelope::ASR::new(0.015, 0.07);

    let mut rack = Rack::new();
    let s1 = Instrument::new(generator::Simple::default(), envelope);
    rack.add(s1);
    let s2 = Instrument::new(generator::Simple::square(), envelope::ASR::new(0.2, 0.0));
    rack.add(s2);

    let sound = rack.play(90.0, note![A: C4, 1 / 16], 1.0);

    let minimum = sound
        .iter()
        .filter_map(|&x| Some(x))
        .reduce(f64::min)
        .expect("there has to be minimum");
    let values: Vec<Block> = sound
        .iter()
        .map(|y| Block::new(1.0, y + minimum.abs()))
        .collect();

    let hits = Line::new(&values);
    let w = FileWriter::new("foo.svg");
    let renderer = Renderer::new(&hits.size());
    w.write(renderer, hits)?;

    Ok(())
}
