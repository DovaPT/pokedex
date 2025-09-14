use std::{
    fmt::{self, Debug, Display}, str
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

#[derive(Debug)]
struct ParseColorError;

impl str::FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (r, g, b) = s
            .strip_prefix('#')
            .and_then(|s| match s.len() {
                3 => Some((&s[0..1], &s[1..2], &s[2..3])),
                6 => Some((&s[0..2], &s[2..4], &s[4..6])),
                _ => None,
            })
            .ok_or(ParseColorError)?;
        let mut r_fromstr = u8::from_str_radix(r, 16).map_err(|_| ParseColorError)?;
        let mut g_fromstr = u8::from_str_radix(g, 16).map_err(|_| ParseColorError)?;
        let mut b_fromstr = u8::from_str_radix(b, 16).map_err(|_| ParseColorError)?;

        if r.len() < 2 {
            r_fromstr *= 16;
            g_fromstr *= 16;
            b_fromstr *= 16;
        }
        Ok(Color {
            r: r_fromstr,
            g: g_fromstr,
            b: b_fromstr,
        })
    }
}

impl Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{};{};{}", self.r, self.g, self.b)
    }
}

struct TermColor {
    bg: Option<Color>,
    fg: Option<Color>,
}

#[allow(dead_code)]
impl TermColor {
    fn new(bg: Option<Color>, fg: Option<Color>) -> Self {
        Self { bg, fg }
    }

    fn paint(&self, s: &str) -> String {
        if self.bg.is_none() && self.fg.is_none() {
            return String::from("");
        }

        let mut res = "\x1b[".to_string();

        if let Some(bg) = &self.bg {
            res.push_str(&format!("48;2;{};", bg));
        }

        if let Some(fg) = &self.fg {
            res.push_str(&format!("38;2;{};", fg));
        }

        res = res.strip_suffix(';').unwrap().to_string();

        res.push('m');

        res.push_str(s);

        res.push_str("\x1b[0m");

        res
    }

    fn with_bg(&mut self, color: Color) -> &mut Self {
        self.bg = Some(color);
        self
    }

    fn with_fg(&mut self, color: Color) -> &mut Self {
        self.fg = Some(color);

        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "name")]
#[serde(rename_all = "lowercase")]
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
        let mut term_color = TermColor::new(None, Some("#FFF".parse().expect("Failed to parse Color")));

        use Type::*;
        match *self {
            Fire => write!(
                f,
                "{}",
                term_color
                    .with_bg("#FF8030".parse().expect("Failed to parse Color"))
                    .paint(" Fire ")
            ),
            Grass => write!(
                f,
                "{}",
                term_color
                    .with_bg("#70C050".parse().expect("Failed to parse Color"))
                    .paint(" Grass ")
            ),
            Water => write!(
                f,
                "{}",
                term_color
                    .with_bg("#3090F0".parse().expect("Failed to parse Color"))
                    .paint(" Water ")
            ),
            Poison => write!(
                f,
                "{}",
                term_color
                    .with_bg("#A05090".parse().expect("Failed to parse Color"))
                    .paint(" Poison ")
            ),
            Bug => write!(
                f,
                "{}",
                term_color
                    .with_bg("#A0B020".parse().expect("Failed to parse Color"))
                    .paint(" Bug ")
            ),
            Normal => write!(
                f,
                "{}",
                term_color
                    .with_bg("#A0A090".parse().expect("Failed to parse Color"))
                    .paint(" Normal ")
            ),
            Flying => write!(
                f,
                "{}",
                term_color
                    .with_bg("#8090F0".parse().expect("Failed to parse Color"))
                    .paint(" Flying ")
            ),
            Dark => write!(
                f,
                "{}",
                term_color
                    .with_bg("#705040".parse().expect("Failed to parse Color"))
                    .paint(" Dark ")
            ),
            Dragon => write!(
                f,
                "{}",
                term_color
                    .with_bg("#7060E0".parse().expect("Failed to parse Color"))
                    .paint(" Dragon ")
            ),
            Rock => write!(
                f,
                "{}",
                term_color
                    .with_bg("#B0A060".parse().expect("Failed to parse Color"))
                    .paint(" Rock ")
            ),
            Ground => write!(
                f,
                "{}",
                term_color
                    .with_bg("#D0B050".parse().expect("Failed to parse Color"))
                    .paint(" Ground ")
            ),
            Ice => write!(
                f,
                "{}",
                term_color
                    .with_bg("#60C0F0".parse().expect("Failed to parse Color"))
                    .paint(" Ice ")
            ),
            Psychic => write!(
                f,
                "{}",
                term_color
                    .with_bg("#F05090".parse().expect("Failed to parse Color"))
                    .paint(" Psychic ")
            ),
            Fairy => write!(
                f,
                "{}",
                term_color
                    .with_bg("#E090E0".parse().expect("Failed to parse Color"))
                    .paint(" Fairy ")
            ),
            Ghost => write!(
                f,
                "{}",
                term_color
                    .with_bg("#6060B0".parse().expect("Failed to parse Color"))
                    .paint(" Ghost ")
            ),
            Fighting => write!(
                f,
                "{}",
                term_color
                    .with_bg("#B05040".parse().expect("Failed to parse Color"))
                    .paint(" Fighting ")
            ),
            Electric => write!(
                f,
                "{}",
                term_color
                    .with_bg("#F0C030".parse().expect("Failed to parse Color"))
                    .paint(" Electric ")
            ),
            Steel => write!(
                f,
                "{}",
                term_color
                    .with_bg("#A0A0B0".parse().expect("Failed to parse Color"))
                    .paint(" Steel ")
            ),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Types {
    r#type: Type
}

#[derive(Serialize, Deserialize)]
pub struct Pokemon {
    id: u32,
    name: String,
    types: Vec<Types>,
}

impl Debug for Pokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Id: {}\nName: {}\nTyping: {} {}",
            self.id, self.name, self.types[0].r#type, self.types.get(1).map_or_else(|| " ".to_string(), |t| t.r#type.to_string())
        )
    }
}
