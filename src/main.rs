use std::{fmt::{Debug, Display}, fs::read_to_string};

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


impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fire => write!(f, "\x1b[48;2;{r};{g};{b}m Fire \x1b[0m",r = 0xFF, g = 0x81, b = 0x30),
            Self::Grass => write!(f, "\x1b[48;2;{r};{g};{b};38;2;0;0;0m Grass \x1b[0m", r = 0x7A, g = 0xC7, b = 0x4C),
            Self::Water => write!(f, "\x1b[48;2;{r};{g};{b};m Water \x1b[0m", r = 0x63, g = 0x90, b = 0xF0),
            Self::Poison => write!(f, "\x1b[48;2;{r};{g};{b}m Poison \x1b[0m", r = 0xA3, g = 0x3E, b = 0xA1),
            Self::Bug => write!(f, "\x1b[48;2;{r};{g};{b};38;2;0;0;0m Bug \x1b[0m", r = 0xA6, g = 0xB9, b = 0x1A),
            Self::Normal => write!(f, "\x1b[48;2;{r};{g};{b};38;2;0;0;0m Normal \x1b[0m", r = 0xA8, g = 0xA7, b = 0x7A),
            Self::Flying => write!(f, "\x1b[48;2;{r};{g};{b};38;2;0;0;0m Flying \x1b[0m", r = 0xA9, g = 0x8F, b = 0xF3),
            Self::Dark => write!(f, "\x1b[48;2;{r};{g};{b};m Dark \x1b[0m", r = 0x70, g = 0x50, b = 0x40),
            Self::Dragon => write!(f, "\x1b[48;2;{r};{g};{b};m Dragon \x1b[0m", r = 0x6F, g = 0x35, b = 0xFC),
            Self::Rock => write!(f, "\x1b[48;2;{r};{g};{b};38;2;0;0;0m Rock \x1b[0m", r = 0xB6, g = 0xA1, b = 0x36),
            Self::Ground => write!(f, "\x1b[48;2;{r};{g};{b};38;2;0;0;0m Ground \x1b[0m", r = 0xE2, g = 0xBF, b = 0x65),
            Self::Ice => write!(f, "\x1b[48;2;{r};{g};{b};38;2;0;0;0m Ice \x1b[0m", r = 0x96, g = 0xD9, b = 0xD6),
            Self::Psychic => write!(f, "\x1b[48;2;{r};{g};{b};38;2;0;0;0m Psychic \x1b[0m", r = 0xF9, g = 0x55, b = 0x87),
            Self::Fairy => write!(f, "\x1b[48;2;{r};{g};{b};38;2;0;0;0m Fairy \x1b[0m", r = 0xE0, g = 0x90, b = 0xE0),
            Self::Ghost => write!(f, "\x1b[48;2;{r};{g};{b};m Ghost \x1b[0m", r = 72, g = 72, b = 157),
            Self::Fighting => write!(f, "\x1b[48;2;{r};{g};{b};m Fighting \x1b[0m", r = 0xB0, g = 0x50, b = 0x40),
            Self::Electric => write!(f, "\x1b[48;2;{r};{g};{b};38;2;0;0;0m Electric \x1b[0m", r = 0xEE, g = 0xD5, b = 0x35),
            Self::Steel => write!(f, "\x1b[48;2;{r};{g};{b};38;2;0;0;0m Steel \x1b[0m", r = 0xB7, g = 0xB7, b = 0xCE),
        }
        
    }
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum Typing {
    Single([Type;1]),
    Double([Type;2])
}

impl Debug for Typing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Typing{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::Single(t) => write!(f, "{}", t[0]),
            Self::Double(t) => write!(f, "{}, {}", t[0], t[1])
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
