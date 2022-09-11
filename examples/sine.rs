use instrument::*;

#[cfg(feature = "graph")]
use graph::svg::Renderer;
#[cfg(feature = "graph")]
use graph::writer::{FileWriter, Writer};
#[cfg(feature = "graph")]
use graph::{Block, Graph, Line};


#[cfg(feature = "graph")]
fn main() -> std::io::Result<()> {
    let synth = Instrument::new(generator::Sine {}, envelope::Fixed {});
    let values: Vec<Block> = synth.play(440.0, 0.01, 1.0)
    .iter()
    .map(|y| Block::new(1.0, y + 1.0))
    .collect();

    let hits = Line::new(&values);
    let w = FileWriter::new("foo.svg");
    let renderer = Renderer::new(&hits.size());
    w.write(renderer, hits)?;

    Ok(())
}
