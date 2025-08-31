use std::{
    fmt::{self, Debug, Display},
};

use serde::{Deserialize, Serialize};

#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
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

    fn with_bg(&mut self, color: &Color) -> &mut Self {
        self.bg = Some(*color);
        self
    }

    fn with_fg(&mut self, color: &Color) -> &mut Self {
        self.fg = Some(*color);

        self
    }
}

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
        let mut color = TermColor::new(
            None,
            Some(Color {
                r: 240,
                g: 240,
                b: 240,
            }),
        );
        match self {
            Self::Fire => write!(
                f,
                "{}",
                color
                    .with_bg(&Color {
                        r: 255,
                        g: 129,
                        b: 48
                    })
                    .paint(" Fire ")
            ),
            Self::Grass => write!(
                f,
                "{}",
                color
                    .with_bg(&Color {
                        r: 112,
                        g: 192,
                        b: 80
                    })
                    .paint(" Grass ")
            ),
            Self::Water => write!(
                f,
                "{}",
                color
                    .with_bg(&Color {
                        r: 48,
                        g: 144,
                        b: 240
                    })
                    .paint(" Water ")
            ),
            Self::Poison => write!(
                f,
                "{}",
                color
                    .with_bg(&Color {
                        r: 160,
                        g: 80,
                        b: 144
                    })
                    .paint(" Poison ")
            ),
            Self::Bug => write!(
                f,
                "{}",
                color
                    .with_bg(&Color {
                        r: 160,
                        g: 176,
                        b: 32
                    })
                    .paint(" Bug ")
            ),
            Self::Normal => write!(
                f,
                "{}",
                color
                    .with_bg(&Color {
                        r: 160,
                        g: 160,
                        b: 144
                    })
                    .paint(" Normal ")
            ),
            Self::Flying => write!(
                f,
                "{}",
                color
                    .with_bg(&Color { 
                        r: 128,
                        g: 144,
                        b: 240
                    })
                    .paint(" Flying ")
            ),
            Self::Dark => write!(
                f,
                "{}",
                color
                    .with_bg(&Color { 
                        r: 112,
                        g: 80,
                        b: 64
                    })
                    .paint(" Dark ")
            ),
            Self::Dragon => write!(
                f,
                "{}",
                color
                    .with_bg(&Color { 
                        r: 112,
                        g: 96,
                        b: 224
                    })
                    .paint(" Dragon ")
            ),
            Self::Rock => write!(
                f,
                "{}",
                color
                    .with_bg(&Color { 
                        r: 176,
                        g: 160,
                        b: 96
                    })
                    .paint(" Rock ")
            ),
            Self::Ground => write!(
                f,
                "{}",
                color
                    .with_bg(&Color { 
                        r: 208,
                        g: 176,
                        b: 80
                    })
                    .paint(" Ground ")
            ),
            Self::Ice => write!(
                f,
                "{}",
                color
                    .with_bg(&Color { 
                        r: 96,
                        g: 192,
                        b: 240
                    })
                    .paint(" Ice ")
            ),
            Self::Psychic => write!(
                f,
                "{}",
                color
                    .with_bg(&Color { 
                        r: 240,
                        g: 80,
                        b: 144
                    })
                    .paint(" Psychic ")
            ),
            Self::Fairy => write!(
                f,
                "{}",
                color
                    .with_bg(&Color { 
                        r: 224,
                        g: 144,
                        b: 224
                    })
                    .paint(" Fairy ")
            ),
            Self::Ghost => write!(
                f,
                "{}",
                color
                    .with_bg(&Color { 
                        r: 96,
                        g: 96,
                        b: 176
                    })
                    .paint(" Ghost ")
            ),
            Self::Fighting => write!(
                f,
                "{}",
                color
                    .with_bg(&Color { 
                        r: 176,
                        g: 80,
                        b: 64
                    })
                    .paint(" Fighting ")
            ),
            Self::Electric => write!(
                f,
                "{}",
                color
                    .with_bg(&Color { 
                        r: 240,
                        g: 192,
                        b: 48
                    })
                    .paint(" Electric ")
            ),
            Self::Steel => write!(
                f,
                "{}",
                color
                    .with_bg(&Color { 
                        r: 160,
                        g: 160,
                        b: 176
                    })
                    .paint(" Steel ")
            ),
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
            Self::Single([t]) => write!(f, "{}", t),
            Self::Double([t1, t2]) => write!(f, "{} {}", t1, t2),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Pokemon {
    id: u32,
    name: String,
    typing: Typing,
}

impl Debug for Pokemon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Id: {}\nName: {}\nTyping: {}",
            self.id, self.name, self.typing
        )
    }
}
