use std::{
    fmt::{Debug, Display},
    ops::Shr as _,
};
use std::ops::Index;

use serde::{Deserialize, Serialize};

use super::axis::WorldAxis;
use super::mat::Mat3;
use super::vec::{IsVec as _, OuterProductExt as _, UVec2, UVec3, Vec3};

/// Represents sides of a voxel/AABB/cube.
///
/// When visualizing, think of a cube and yourself as external observer.
///
/// Relative sides are the result of rotating the cube along X axis for
/// horizontal sides, and Y for top and bottom (vertical).
#[derive(Default, Copy, Clone, Eq, PartialEq, Serialize, Deserialize, Hash)]
#[repr(u8)]
pub enum Side {
    #[default]
    /// vector +X (right) direction
    East = 0,
    /// vector -X (left) direction
    West = 1,
    /// vector +Y direction
    Top = 2,
    /// vector -Y direction
    Bottom = 3,
    /// vector +Z (front) direction
    South = 4,
    /// vector -Z (back) direction
    North = 5,
}

impl Debug for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::East => f.write_str("Side::East"),
            Side::West => f.write_str("Side::West"),
            Side::Top => f.write_str("Side::Top"),
            Side::Bottom => f.write_str("Side::Bottom"),
            Side::South => f.write_str("Side::South"),
            Side::North => f.write_str("Side::North"),
        }
    }
}

impl Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Side::East => f.write_str("East"),
            Side::West => f.write_str("West"),
            Side::Top => f.write_str("Top"),
            Side::Bottom => f.write_str("Bottom"),
            Side::South => f.write_str("South"),
            Side::North => f.write_str("North"),
        }
    }
}

impl Side {
    pub const COUNT: usize = 6;

    pub const ALL: [Side; Side::COUNT] = [
        Side::East,
        Side::West,
        Side::Top,
        Side::Bottom,
        Side::South,
        Side::North,
    ];

    #[inline(always)]
    pub const fn as_usize(self) -> usize {
        self as usize
    }

    #[inline]
    pub const fn opposite(self) -> Side {
        unsafe {
            // SAFETY: Least significant bit indicates direction. Xor on it
            // always returns a valid variant.
            std::mem::transmute((self as u8) ^ 0x1)
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

    #[inline]
    pub const fn is_negative(self) -> bool {
        (self as u8 & 0x1) == 1
    }

    #[inline]
    pub fn axis(self) -> WorldAxis {
        unsafe {
            // SAFETY: Sides are ordered the same as WorldAxis, least
            // significant bit signifies negative direction. Shifting the side
            // u8 to right by 1 bit discards the direction bit and leaves only
            // the axis bit which matches the WorldAxis one. As there's no way
            // of `self >> 1` returning a value greater than 2, this operation
            // is safe.
            std::mem::transmute::<_, WorldAxis>((self as u8).shr(1))
        }
    }

    #[inline]
    pub fn direction(self) -> Vec3 {
        self.axis().as_vec3() * ((self as u8 & 0x1) as f32 * -2. + 1.)
    }

    pub fn rotation_to(self, other: Side) -> Mat3 {
        if self == other {
            return Mat3::IDENTITY;
        }

        let a = self.direction();
        let b = other.direction();
        let k = (a + b) / 2.;

        if k == Vec3::ZERO {
            Mat3::from_diagonal(Vec3::new(-1., -1., 1.))
        } else {
            k.outer_product() * (2. / k.inner_product()) - Mat3::IDENTITY
        }
    }

    #[inline]
    pub fn normal(self) -> Vec3 {
        self.axis().as_vec3() * ((self as u8 & 0x1) as f32 * 2. - 1.)
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
    pub fn offset(self, position: Vec3, amount: f32) -> Vec3 {
        position + self.direction() * amount
    }
}

impl<T> Index<Side> for [T; Side::COUNT] {
    type Output = T;

    fn index(&self, index: Side) -> &Self::Output {
        &self[index.as_usize()]
    }
}
