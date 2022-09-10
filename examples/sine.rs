use instrument::*;

#[cfg(feature = "graph")]
fn main() -> std::io::Result<()> {
    let synth = Instrument::new(generator::Sine {}, envelope::Fixed {});
    Ok(())
}
