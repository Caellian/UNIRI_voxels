use bevy::prelude::Color;
use serde::de::{Error, Visitor};
use serde::Deserializer;
use std::fmt::Formatter;

struct HexColorVisitor;

impl HexColorVisitor {
    #[inline]
    fn hex_char_value(c: char) -> Option<u8> {
        Some(match c {
            '0'..='9' => c as u8 - b'0',
            'A'..='F' => c as u8 - b'A' + 10,
            'a'..='f' => c as u8 - b'a' + 10,
            _ => return None,
        })
    }

    #[inline]
    fn parse_hex_single(from: &str, pos: usize) -> Option<u8> {
        let a = Self::hex_char_value(from.chars().nth(pos)?)?;
        Some(a << 4 | a)
    }

    #[inline]
    fn parse_hex_pair(from: &str, pos: usize) -> Option<u8> {
        let a = Self::hex_char_value(from.chars().nth(pos)?)?;
        let b = Self::hex_char_value(from.chars().nth(pos + 1)?)?;
        Some(a << 4 | b)
    }

    fn parse_hex_str(v: &str) -> Option<Color> {
        Some(match v.len() {
            3 => Color::Rgba {
                red: Self::parse_hex_single(v, 0)? as f32 / 255.,
                green: Self::parse_hex_single(v, 1)? as f32 / 255.,
                blue: Self::parse_hex_single(v, 2)? as f32 / 255.,
                alpha: 1.0,
            },
            4 => Color::Rgba {
                red: Self::parse_hex_single(v, 0)? as f32 / 255.,
                green: Self::parse_hex_single(v, 1)? as f32 / 255.,
                blue: Self::parse_hex_single(v, 2)? as f32 / 255.,
                alpha: Self::parse_hex_single(v, 4)? as f32 / 255.,
            },
            6 => Color::Rgba {
                red: Self::parse_hex_pair(v, 0)? as f32 / 255.,
                green: Self::parse_hex_pair(v, 2)? as f32 / 255.,
                blue: Self::parse_hex_pair(v, 4)? as f32 / 255.,
                alpha: 1.0,
            },
            8 => Color::Rgba {
                red: Self::parse_hex_pair(v, 0)? as f32 / 255.,
                green: Self::parse_hex_pair(v, 2)? as f32 / 255.,
                blue: Self::parse_hex_pair(v, 4)? as f32 / 255.,
                alpha: Self::parse_hex_pair(v, 6)? as f32 / 255.,
            },
            _ => return None,
        })
    }
}

impl<'de> Visitor<'de> for HexColorVisitor {
    type Value = Color;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a hex color")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        let mut v = v.trim();

        v = if let Some('#') = v.chars().next() {
            &v[1..]
        } else {
            return Err(E::custom("hex color must start with a '#' char"));
        };

        Self::parse_hex_str(v).ok_or(E::custom("invalid hex color value"))
    }
}

pub fn deserialize_hex_color<'de, D>(
    deserializer: D,
) -> Result<bevy::render::color::Color, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(HexColorVisitor)
}
