use instrument::*;

#[cfg(feature = "graph")]
use graph::svg::Renderer;
#[cfg(feature = "graph")]
use graph::writer::{FileWriter, Writer};
#[cfg(feature = "graph")]
use graph::{Block, Graph, Hits};


#[cfg(feature = "graph")]
fn main() -> std::io::Result<()> {
    let synth = Instrument::new(generator::Sine {}, envelope::Fixed {});
    let values: Vec<Block> = synth.play(440.0, 1.0, 1.0)
    .iter()
    .map(|x| Block::new(1.0, (1.0+x)*10.0))
    .collect();

    let hits = Hits::new(&values);
    let w = FileWriter::new("foo.svg");
    let renderer = Renderer::new(&hits.size());
    w.write(renderer, hits)?;

    Ok(())
}
