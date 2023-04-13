use serde::de::{Error, Visitor};
use serde::{Deserialize, Deserializer};
use std::fmt::Formatter;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}
impl Rgb {
    pub fn to_rgba(&self) -> Rgba {
        Rgba {
            r: self.r,
            g: self.g,
            b: self.b,
            a: u8::MAX,
        }
    }
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Rgba {
    pub fn to_rgb(&self) -> Rgb {
        Rgb {
            r: self.r,
            g: self.g,
            b: self.b,
        }
    }
}

struct ColorVisitor;

#[inline]
fn hex_char_value(c: char) -> Option<u8> {
    Some(match c {
        '0'..='9' => c as u8 - '0' as u8,
        'A'..='F' => c as u8 - 'A' as u8 + 10,
        'a'..='f' => c as u8 - 'a' as u8 + 10,
        _ => return None,
    })
}

#[inline]
fn parse_hex_single(from: &str, pos: usize) -> Option<u8> {
    let a = hex_char_value(from.chars().nth(pos)?)?;
    Some(a << 4 | a)
}

#[inline]
fn parse_hex_pair(from: &str, pos: usize) -> Option<u8> {
    let a = hex_char_value(from.chars().nth(pos)?)?;
    let b = hex_char_value(from.chars().nth(pos + 1)?)?;
    Some(a << 4 | b)
}

fn parse_hex_str(v: &str) -> Option<Rgba> {
    Some(match v.len() {
        3 => Rgba {
            r: parse_hex_single(v, 0)?,
            g: parse_hex_single(v, 1)?,
            b: parse_hex_single(v, 2)?,
            a: u8::MAX,
        },
        4 => Rgba {
            r: parse_hex_single(v, 0)?,
            g: parse_hex_single(v, 1)?,
            b: parse_hex_single(v, 2)?,
            a: parse_hex_single(v, 4)?,
        },
        6 => Rgba {
            r: parse_hex_pair(v, 0)?,
            g: parse_hex_pair(v, 2)?,
            b: parse_hex_pair(v, 4)?,
            a: u8::MAX,
        },
        8 => Rgba {
            r: parse_hex_pair(v, 0)?,
            g: parse_hex_pair(v, 2)?,
            b: parse_hex_pair(v, 4)?,
            a: parse_hex_pair(v, 6)?,
        },
        _ => return None,
    })
}

impl<'de> Visitor<'de> for ColorVisitor {
    type Value = Rgba;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a color")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let mut v = v.trim();

        v = if let Some('#') = v.chars().nth(0) {
            &v[1..]
        } else {
            return Err(E::custom("hex color must start with a '#' char"));
        };

        parse_hex_str(v).ok_or(E::custom("invalid hex color value"))
    }
}

impl<'de> Deserialize<'de> for Rgb {
    fn deserialize<D>(deserializer: D) -> Result<Rgb, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer
            .deserialize_str(ColorVisitor)
            .map(|it| it.to_rgb())
    }
}

impl<'de> Deserialize<'de> for Rgba {
    fn deserialize<D>(deserializer: D) -> Result<Rgba, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_str(ColorVisitor)
    }
}
