use crate::color::Rgb;
use crate::world::WorldAxis;
use crate::Vec3;
use bevy::prelude::*;
use derive_more::Deref;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::ops::Deref;
use std::string::ToString;

#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct MaterialID(String);

impl Default for MaterialID {
    fn default() -> Self {
        MaterialID::new("unknown")
    }
}

impl MaterialID {
    pub fn new(id: impl AsRef<str>) -> MaterialID {
        MaterialID(id.as_ref().to_string())
    }

    #[must_use]
    pub fn air() -> MaterialID {
        MaterialID("air".to_string())
    }
}

impl Deref for MaterialID {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Deref, Deserialize)]
#[serde(transparent)]
#[repr(transparent)]
pub struct BlockColor(Rgb);

impl BlockColor {
    pub fn to_array(&self) -> [f32; 4] {
        [
            self.0.r as f32 / 255.0,
            self.0.g as f32 / 255.0,
            self.0.b as f32 / 255.0,
            1.0,
        ]
    }

    pub fn to_bevy_color(&self) -> Color {
        Color::Rgba {
            red: self.0.r as f32 / 255.0,
            green: self.0.g as f32 / 255.0,
            blue: self.0.b as f32 / 255.0,
            alpha: 1.0,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct BlockProperties {
    pub color: BlockColor,
}

/// When visualizing, think of a cube and yourself as external observer.
/// Relative sides are the result of rotating the cube along X axis for
/// horizontal sides, and Y for top and bottom (vertical).
#[derive(Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[repr(u8)]
pub enum Side {
    /// +Y side; looks toward -Y
    Top,
    /// -Y side; looks toward +Y
    Bottom,
    /// -X (left) side; looks toward +X (right)
    West,
    /// +X (right) side; looks toward -X (left)
    East,
    /// -Z (back) side; looks toward +Z (front)
    North,
    /// +Z (front) side; looks toward -Z (back)
    South,
}

impl Debug for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Top => f.write_str("Side::Top"),
            Side::Bottom => f.write_str("Side::Bottom"),
            Side::West => f.write_str("Side::West"),
            Side::East => f.write_str("Side::East"),
            Side::North => f.write_str("Side::North"),
            Side::South => f.write_str("Side::South"),
        }
    }
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::Top => f.write_str("Top"),
            Side::Bottom => f.write_str("Bottom"),
            Side::West => f.write_str("West"),
            Side::East => f.write_str("East"),
            Side::North => f.write_str("North"),
            Side::South => f.write_str("South"),
        }
    }
}

impl Side {
    pub const ALL: [Side; 6] = [
        Side::Top,
        Side::Bottom,
        Side::West,
        Side::East,
        Side::North,
        Side::South,
    ];

    pub const fn index(self) -> usize {
        match self {
            Side::Top => 0,
            Side::Bottom => 1,
            Side::West => 2,
            Side::East => 3,
            Side::North => 4,
            Side::South => 5,
        }
    }

    pub const fn opposite(self) -> Side {
        match self {
            Side::East => Side::West,
            Side::West => Side::East,
            Side::Top => Side::Bottom,
            Side::Bottom => Side::Top,
            Side::South => Side::North,
            Side::North => Side::South,
        }
    }

    pub const fn up(self) -> Side {
        match self {
            Side::Top => Side::North,
            Side::Bottom => Side::South,
            _ => Side::Top,
        }
    }

    pub const fn down(self) -> Side {
        match self {
            Side::Top => Side::South,
            Side::Bottom => Side::North,
            _ => Side::Bottom,
        }
    }

    // TODO: left & right

    
    pub const fn axis(self) -> WorldAxis {
        match self {
            Side::East => WorldAxis::X,
            Side::West => WorldAxis::X,
            Side::Top => WorldAxis::Y,
            Side::Bottom => WorldAxis::Y,
            Side::South => WorldAxis::Z,
            Side::North => WorldAxis::Z,
        }
    }

    pub const fn direction(self) -> Vec3 {
        match self {
            Side::East => Vec3::new(1.0, 0.0, 0.0),
            Side::West => Vec3::new(-1.0, 0.0, 0.0),
            Side::Top => Vec3::new(0.0, 1.0, 0.0),
            Side::Bottom => Vec3::new(0.0, -1.0, 0.0),
            Side::South => Vec3::new(0.0, 0.0, 1.0),
            Side::North => Vec3::new(0.0, 0.0, -1.0),
        }
    }

    pub const fn normal(self) -> Vec3 {
        self.opposite().direction()
    }

    pub fn depth_pos(self, depth: u32, x: u32, y: u32, chunk_size: UVec3) -> Vec3 {
        let depth = depth as f32;
        let x = x as f32;
        let y = y as f32;

        match self {
            Side::East => Vec3::new(x, y, chunk_size.z as f32 - depth),
            Side::West => Vec3::new(chunk_size.x as f32 - x, y, depth),
            Side::Top => Vec3::new(y, chunk_size.y as f32 - depth, x),
            Side::Bottom => Vec3::new(chunk_size.x as f32 - y, depth, x),
            Side::South => Vec3::new(depth, y, x),
            Side::North => Vec3::new(chunk_size.x as f32 - depth, y, chunk_size.z as f32 - x),
        }
    }

    #[inline]
    pub fn offset(self, position: Vec3, amount: Option<f32>) -> Vec3 {
        position + self.direction() * amount.unwrap_or(1.0)
    }
}

/// Stores the front face of a sided block.
#[derive(Debug, Deserialize, Component)]
pub struct SidedBlock(Side);

/*
#[derive(Deref)]
pub struct MaterialRegistry(pub BTreeMap<MaterialID, Material>);

impl Default for MaterialRegistry {
    fn default() -> Self {
        let blocks: Vec<PathBuf> = std::fs::read_dir("./blocks")
            .unwrap()
            .filter_map(|it| it.ok().map(|e| e.path()))
            .collect();

        let mut result = BTreeMap::new();
        for path in blocks {
            let block_str = std::fs::read_to_string(path).unwrap();
            let mut material: Material = ron::from_str(&block_str).expect("unable to read block");

            let id = path
                .file_name()
                .expect("no filename")
                .to_str()
                .unwrap()
                .to_string();

            material.id = MaterialID(id.clone());

            result.insert(MaterialID(id), material);
        }

        MaterialRegistry(result)
    }
}

impl MaterialRegistry {
    pub fn get_material(&self, id: &MaterialID) -> Option<&Material> {
        self.0.get(id)
    }
}
*/
