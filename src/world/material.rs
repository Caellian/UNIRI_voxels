use crate::world::WorldAxis;
use crate::{decl_id_type, Vec3};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Display};
use std::string::ToString;

decl_id_type!(MaterialID);

impl MaterialID {
    #[must_use]
    pub const fn air() -> MaterialID {
        MaterialID::Static("air")
    }
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

    #[inline]
    pub const fn depth_pos(self, size: UVec3, depth: u32, pos: UVec2) -> UVec3 {
        let z = depth;
        let x = pos.x;
        let y = pos.y;

        match self {
            Side::East => UVec3::new(x, y, size.z - z),
            Side::West => UVec3::new(size.x - x, y, z),
            Side::Top => UVec3::new(y, size.y - z, x),
            Side::Bottom => UVec3::new(size.x - y, z, x),
            Side::South => UVec3::new(z, y, x),
            Side::North => UVec3::new(size.x - z, y, size.z - x),
        }
    }

    #[inline]
    pub fn offset(self, position: Vec3, amount: Option<f32>) -> Vec3 {
        position + self.direction() * amount.unwrap_or(1.0)
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
pub enum Voxel {
    /// Air / invalid state
    #[default]
    None,
    /// Voxels with custom colors
    Color(Color),
    /// Voxels with registered materials
    MaterialID(MaterialID),
}

/// Stores the front face of a sided block.
#[derive(Debug, Deserialize, Component)]
pub struct SidedBlock(Side);

/*
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum MatterState {
    Plasma,
    Gaseous,
    Liquid,
    Solid,
}
*/
