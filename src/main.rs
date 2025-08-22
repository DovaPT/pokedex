use std::{
    fmt::{Debug, Display},
    fs::read_to_string,
};

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

impl Type {
    fn color(&self) -> &'static str {
        match self {
            Self::Fire => "\x1b[48;2;255;129;48;38;2;240;240;240m",
            Self::Grass => "\x1b[48;2;112;192;80;38;2;240;240;240m",
            Self::Water => "\x1b[48;2;48;144;240;38;2;240;240;240m",
            Self::Poison => "\x1b[48;2;160;80;144;38;2;240;240;240m",
            Self::Bug => "\x1b[48;2;160;176;32;38;2;240;240;240m",
            Self::Normal => "\x1b[48;2;160;160;144;38;2;240;240;240m",
            Self::Flying => "\x1b[48;2;128;144;240;38;2;240;240;240m",
            Self::Dark => "\x1b[48;2;112;80;64;38;2;240;240;240m",
            Self::Dragon => "\x1b[48;2;112;96;224;38;2;240;240;240m",
            Self::Rock => "\x1b[48;2;176;160;96;38;2;240;240;240m",
            Self::Ground => "\x1b[48;2;208;176;80;38;2;240;240;240m",
            Self::Ice => "\x1b[48;2;96;192;240;38;2;240;240;240m",
            Self::Psychic => "\x1b[48;2;240;80;144;38;2;240;240;240m",
            Self::Fairy => "\x1b[48;2;224;144;224;38;2;240;240;240m",
            Self::Ghost => "\x1b[48;2;96;96;176;38;2;240;240;240m",
            Self::Fighting => "\x1b[48;2;176;80;64;38;2;240;240;240m",
            Self::Electric => "\x1b[48;2;240;192;48;38;2;240;240;240m",
            Self::Steel => "\x1b[48;2;160;160;176;38;2;240;240;240m",
        }
    }
}

impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fire => write!(f, "{} Fire \x1b[0m", self.color()),
            Self::Grass => write!(f, "{} Grass \x1b[0m", self.color()),
            Self::Water => write!(f, "{} Water \x1b[0m", self.color()),
            Self::Poison => write!(f, "{} Poison \x1b[0m", self.color()),
            Self::Bug => write!(f, "{} Bug \x1b[0m", self.color()),
            Self::Normal => write!(f, "{} Normal \x1b[0m", self.color()),
            Self::Flying => write!(f, "{} Flying \x1b[0m", self.color()),
            Self::Dark => write!(f, "{} Dark \x1b[0m", self.color()),
            Self::Dragon => write!(f, "{} Dragon \x1b[0m", self.color()),
            Self::Rock => write!(f, "{} Rock \x1b[0m", self.color()),
            Self::Ground => write!(f, "{} Ground \x1b[0m", self.color()),
            Self::Ice => write!(f, "{} Ice \x1b[0m", self.color()),
            Self::Psychic => write!(f, "{} Psychic \x1b[0m", self.color()),
            Self::Fairy => write!(f, "{} Fairy \x1b[0m", self.color()),
            Self::Ghost => write!(f, "{} Ghost \x1b[0m", self.color()),
            Self::Fighting => write!(f, "{} Fighting \x1b[0m", self.color()),
            Self::Electric => write!(f, "{} Electric \x1b[0m", self.color()),
            Self::Steel => write!(f, "{} Steel \x1b[0m", self.color()),
        }
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Typing {
    Single([Type; 1]),
    Double([Type; 2]),
}

impl Debug for Typing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Typing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Single(t) => write!(f, "{}", t[0]),
            Self::Double(t) => write!(f, "{}, {}", t[0], t[1]),
        }
    }
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
    println!("{:#?}", mons);
    Ok(())
}
