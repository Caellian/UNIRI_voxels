use bevy::prelude::{Color, Vec4};
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

    fn parse_hex_str(v: &str) -> Option<Vec4> {
        match v.len() {
            3 => Some(Vec4::new(
                Self::parse_hex_single(v, 0)? as f32 / 255.,
                Self::parse_hex_single(v, 1)? as f32 / 255.,
                Self::parse_hex_single(v, 2)? as f32 / 255.,
                1.0,
            )),
            4 => Some(Vec4::new(
                Self::parse_hex_single(v, 0)? as f32 / 255.,
                Self::parse_hex_single(v, 1)? as f32 / 255.,
                Self::parse_hex_single(v, 2)? as f32 / 255.,
                Self::parse_hex_single(v, 4)? as f32 / 255.,
            )),
            6 => Some(Vec4::new(
                Self::parse_hex_pair(v, 0)? as f32 / 255.,
                Self::parse_hex_pair(v, 2)? as f32 / 255.,
                Self::parse_hex_pair(v, 4)? as f32 / 255.,
                1.0,
            )),
            8 => Some(Vec4::new(
                Self::parse_hex_pair(v, 0)? as f32 / 255.,
                Self::parse_hex_pair(v, 2)? as f32 / 255.,
                Self::parse_hex_pair(v, 4)? as f32 / 255.,
                Self::parse_hex_pair(v, 6)? as f32 / 255.,
            )),
            _ => None,
        }
    }
}

impl<'de> Visitor<'de> for HexColorVisitor {
    type Value = Vec4;

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

pub fn deserialize_hex_color<'de, D>(deserializer: D) -> Result<Vec4, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(HexColorVisitor)
}

struct DecimalColor(u32);

impl From<DecimalColor> for Color {
    fn from(val: DecimalColor) -> Self {
        let (a, b, g, r) = (
            val.0 >> 24u32 & 0xFF,
            val.0 >> 16u32 & 0xFF,
            val.0 >> 8u32 & 0xFF,
            val.0 & 0xFF,
        );

        Color::Rgba {
            red: r as f32 / 255.0,
            green: g as f32 / 255.0,
            blue: b as f32 / 255.0,
            alpha: a as f32 / 255.0,
        }
    }
}
