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
    let synth = Instrument::new(generator::simple::Simple::square(), envelope::Fixed {});
    let sound = synth.play(90.0, note![A: C2, 1 / 64], 1.0);
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
    w.write_image(renderer, hits)?;

    Ok(())
}
