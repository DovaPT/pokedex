use std::fs::read_to_string;
use pokedex::Pokemon;
use serde_json::Result;

fn main() -> Result<()> {
    let json = read_to_string("pokemon.json").expect("could not read file");
    let mons: Vec<Pokemon> = serde_json::from_str(&json)?;
    for p in mons {
        println!("{p:#?}\n");
    }
    Ok(())
}
