use std::fs::read_to_string;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize, Debug)]
enum Type {
    Fire,
    Grass,
    Water,
    Poison,
    Bug,
    Normal,
    Flying,
    Dark,
    Dragon,
    Rock,
    Ground,
    Ice,
    Psychic,
    Fairy,
    Ghost,
    Fighting,
    Electric,
    Steel,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum Typing {
    Single(Type),
    Double(Type, Type),
}

#[derive(Serialize, Deserialize, Debug)]
struct Pokemon {
    id: u32,
    name: String,
    typing: Typing,
}

fn main() -> Result<()> {
    let json = read_to_string("pokemon.json").expect("could not read file");
    let mons: Vec<Pokemon> = serde_json::from_str(&json)?;
    println!("{:?}", mons[1]);
    Ok(())
}
