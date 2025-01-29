use std::ops::Index;

use super::pos::ChunkPos;
use super::vec::{IVec2, IVec3, UVec2, UVec3, Vec2, Vec3};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum WorldAxis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl WorldAxis {
    #[inline]
    pub const fn as_vec3(self) -> Vec3 {
        [Vec3::X, Vec3::Y, Vec3::Z][self as usize]
    }

    #[inline]
    pub const fn as_uvec3(self) -> UVec3 {
        [UVec3::X, UVec3::Y, UVec3::Z][self as usize]
    }

    #[inline]
    pub const fn as_ivec3(self) -> IVec3 {
        [IVec3::X, IVec3::Y, IVec3::Z][self as usize]
    }

    #[inline]
    pub const fn slice_plane(self) -> [WorldAxis; 2] {
        [
            [WorldAxis::Y, WorldAxis::Z],
            [WorldAxis::X, WorldAxis::Z],
            [WorldAxis::X, WorldAxis::Y],
        ][self as usize]
    }
}

pub trait AsVecT<T> {
    fn as_vec_t(self) -> T;
}

impl AsVecT<Vec3> for WorldAxis {
    #[inline]
    fn as_vec_t(self) -> Vec3 {
        self.as_vec3()
    }
}
impl AsVecT<UVec3> for WorldAxis {
    #[inline]
    fn as_vec_t(self) -> UVec3 {
        self.as_uvec3()
    }
}
impl AsVecT<IVec3> for WorldAxis {
    #[inline]
    fn as_vec_t(self) -> IVec3 {
        self.as_ivec3()
    }
}
impl AsVecT<Vec2> for WorldAxis {
    #[inline]
    fn as_vec_t(self) -> Vec2 {
        [Vec2::X, Vec2::Y][self as usize]
    }
}
impl AsVecT<UVec2> for WorldAxis {
    #[inline]
    fn as_vec_t(self) -> UVec2 {
        [UVec2::X, UVec2::Y][self as usize]
    }
}
impl AsVecT<IVec2> for WorldAxis {
    #[inline]
    fn as_vec_t(self) -> IVec2 {
        [IVec2::X, IVec2::Y][self as usize]
    }
}

impl Index<WorldAxis> for ChunkPos {
    type Output = i32;

    fn index(&self, index: WorldAxis) -> &Self::Output {
        match index {
            WorldAxis::X => &self.value.x,
            WorldAxis::Y => &self.value.y,
            WorldAxis::Z => &self.value.z,
        }
    }
}
impl Index<WorldAxis> for Vec3 {
    type Output = f32;

    fn index(&self, index: WorldAxis) -> &Self::Output {
        match index {
            WorldAxis::X => &self.x,
            WorldAxis::Y => &self.y,
            WorldAxis::Z => &self.z,
        }
    }
}
impl Index<WorldAxis> for UVec3 {
    type Output = u32;

    fn index(&self, index: WorldAxis) -> &Self::Output {
        match index {
            WorldAxis::X => &self.x,
            WorldAxis::Y => &self.y,
            WorldAxis::Z => &self.z,
        }
    }
}
impl Index<WorldAxis> for IVec3 {
    type Output = i32;

    fn index(&self, index: WorldAxis) -> &Self::Output {
        match index {
            WorldAxis::X => &self.x,
            WorldAxis::Y => &self.y,
            WorldAxis::Z => &self.z,
        }
    }
}

impl Index<WorldAxis> for Vec2 {
    type Output = f32;

    fn index(&self, index: WorldAxis) -> &Self::Output {
        match index {
            WorldAxis::X => &self.x,
            WorldAxis::Y => &self.y,
            WorldAxis::Z => panic!("invalid Vec2 axis: WorldAxis::Z"),
        }
    }
}
impl Index<WorldAxis> for UVec2 {
    type Output = u32;

    fn index(&self, index: WorldAxis) -> &Self::Output {
        match index {
            WorldAxis::X => &self.x,
            WorldAxis::Y => &self.y,
            WorldAxis::Z => panic!("invalid Vec2 axis: WorldAxis::Z"),
        }
    }
}
impl Index<WorldAxis> for IVec2 {
    type Output = i32;

    fn index(&self, index: WorldAxis) -> &Self::Output {
        match index {
            WorldAxis::X => &self.x,
            WorldAxis::Y => &self.y,
            WorldAxis::Z => panic!("invalid Vec2 axis: WorldAxis::Z"),
        }
    }
}
