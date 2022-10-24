use anyhow::{Ok, Result};
use substreams_ethereum::Abigen;

fn main() -> Result<(), anyhow::Error> {
    Abigen::new("ENS", "abi/ens.json")?
        .generate()?
        .write_to_file("src/abi/ens.rs")?;

    Ok(())
}
