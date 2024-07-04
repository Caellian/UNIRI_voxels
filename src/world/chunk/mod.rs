use std::marker::PhantomData;

use crate::world::material::Side;
use crate::{convert::Convert, math::vec::IsVec};
use ahash::HashMapExt as _;
use bevy::prelude::*;
use once_cell::sync::Lazy;

pub mod chunk_material;
pub mod mesh;

#[derive(Debug, Eq, PartialEq, Hash)]
#[repr(u8)]
#[derive(Default)]
pub enum Mesher {
    #[default]
    Greedy,
}

#[derive(Debug, Default, Component)]
pub struct ChunkInfo {
    pub mesher: Mesher,
}

pub type ChunkValueIndex = u16;
pub const MAX_CHUNK_VALUES: usize = ChunkValueIndex::MAX as usize;
pub const CHUNK_FRONT: Side = Side::South;

/// Container for a 3D grid/array of values.
///
/// Chunk values are stored in x, z, y order in an contiguous block of memory.
/// Default (front-facing) [`Side`] of a chunk is [`south`](Side::South).
#[derive(Debug, Default, Component)]
pub struct ChunkStore<T: PartialEq> {
    pub values: Vec<T>,
    pub size: UVec3,
    pub content: Vec<ChunkValueIndex>,
}

impl<T: PartialEq + Clone> Clone for ChunkStore<T> {
    fn clone(&self) -> Self {
        ChunkStore {
            values: self.values.clone(),
            size: self.size,
            content: self.content.clone(),
        }
    }
}

impl<T: PartialEq> ChunkStore<T> {
    pub fn new(size: UVec3) -> ChunkStore<T> {
        ChunkStore {
            values: Vec::with_capacity(((size.x * size.y * size.z) as f32 * 0.01).ceil() as usize),
            size,
            content: vec![0; size.x as usize * size.y as usize * size.z as usize],
        }
    }

    pub fn empty() -> ChunkStore<T> {
        ChunkStore {
            values: vec![],
            size: UVec3::ZERO,
            content: vec![],
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size == UVec3::ZERO || self.values.len() == 0 && self.content.len() == 0
    }

    pub fn map<'a, V: PartialEq>(&'a self, f: fn(&'a T) -> V) -> ChunkStore<V> {
        ChunkStore {
            values: self.values.iter().map(f).collect(),
            size: self.size,
            content: self.content.clone(),
        }
    }

    pub fn map_into<V: PartialEq>(self, f: fn(T) -> V) -> ChunkStore<V> {
        ChunkStore {
            values: self.values.into_iter().map(f).collect(),
            size: self.size,
            content: self.content,
        }
    }
}

pub trait SizedGrid<T> {
    #[must_use]
    fn size(&self) -> UVec3;

    #[must_use]
    fn get_position_index(&self, pos: UVec3) -> usize;

    #[must_use]
    fn get_pos_key(&self, pos: UVec3) -> Option<ChunkValueIndex>;
    #[must_use]
    fn get_values(&self) -> &Vec<T>;
    #[must_use]
    fn index_of_value(&self, value: &T) -> Option<u16>
    where
        T: PartialEq,
    {
        self.get_values()
            .iter()
            .enumerate()
            .take(u16::MAX as usize - 1)
            .find_map(|(i, it)| {
                if it == value {
                    Some(i as u16 + 1)
                } else {
                    None
                }
            })
    }
    #[must_use]
    fn value_of_index(&self, index: u16) -> Option<&T> {
        if index == 0 {
            None
        } else {
            self.get_values().get(index as usize - 1)
        }
    }
    #[must_use]
    fn get_pos_value(&self, pos: UVec3) -> Option<&T> {
        if let Some(i) = self.get_pos_key(pos) {
            self.get_values().get(i as usize - 1)
        } else {
            None
        }
    }
}

pub trait SizedGridMut<T>: SizedGrid<T> {
    fn get_pos_key_mut(&mut self, pos: UVec3) -> Option<&mut ChunkValueIndex>;
    #[must_use]
    fn get_values_mut(&mut self) -> &mut Vec<T>;
    fn insert_key(&mut self, key: T) -> u16;
    #[inline]
    fn set_pos_id(&mut self, pos: UVec3, value: ChunkValueIndex) {
        if let Some(pos) = self.get_pos_key_mut(pos) {
            *pos = value;
        }
    }
    fn set_pos_value(&mut self, pos: UVec3, value: Option<T>)
    where
        T: PartialEq,
    {
        if pos.x >= self.size().x || pos.y >= self.size().y || pos.z >= self.size().z {
            return;
        }
        let i = value
            .map(|it| {
                self.index_of_value(&it).unwrap_or_else(|| {
                    self.get_values_mut().push(it);
                    self.get_values().len() as u16
                })
            })
            .unwrap_or(0);

        self.set_pos_id(pos, i);
    }
    fn set_or_clone_pos_value(&mut self, pos: UVec3, value: Option<&T>)
    where
        T: PartialEq + Clone,
    {
        let i = value
            .map(|it| {
                self.index_of_value(it).unwrap_or_else(|| {
                    self.get_values_mut().push(it.clone());
                    self.get_values().len() as u16
                })
            })
            .unwrap_or(0);

        self.set_pos_id(pos, i);
    }
}

impl<T: PartialEq> SizedGrid<T> for ChunkStore<T> {
    #[inline(always)]
    fn size(&self) -> UVec3 {
        self.size
    }
    #[inline]
    fn get_position_index(&self, pos: UVec3) -> usize {
        pos.x as usize
            + pos.z as usize * self.size.x as usize
            + pos.y as usize * self.size.x as usize * self.size.z as usize
    }
    #[inline]
    fn get_pos_key(&self, pos: UVec3) -> Option<ChunkValueIndex> {
        self.content.get(self.get_position_index(pos)).cloned()
    }
    #[inline(always)]
    fn get_values(&self) -> &Vec<T> {
        &self.values
    }
}

impl<T: PartialEq> SizedGridMut<T> for ChunkStore<T> {
    #[inline]
    fn get_pos_key_mut(&mut self, pos: UVec3) -> Option<&mut ChunkValueIndex> {
        self.content.get_mut(self.get_position_index(pos))
    }
    #[inline(always)]
    fn get_values_mut(&mut self) -> &mut Vec<T> {
        &mut self.values
    }
    fn insert_key(&mut self, key: T) -> u16 {
        if self.values.len() + 1 > MAX_CHUNK_VALUES {
            panic!("overfull chunk palette");
        }
        self.values.push(key);
        self.values.len() as u16
    }
}

/// Used to rotate [`ChunkStore`] indexing.
pub struct SideView<'a, T: PartialEq, G: SizedGrid<T>> {
    inner: &'a G,
    transform: (Mat3, Vec3),
    _phantom: PhantomData<T>,
}

pub(crate) fn wrapped_rows(mul: Mat3) -> (Mat3, Vec3) {
    let cin = |row| {
        if mul.row(0).sum() < 0. {
            1.
        } else {
            0.
        }
    };
    (mul, Vec3::new(cin(0), cin(1), cin(2)))
}

pub(crate) static SIDE_VIEW_TRANSFORMS: Lazy<[(Mat3, Vec3); 6]> = Lazy::new(|| {
    Side::ALL
        .map(|it| CHUNK_FRONT.rotation_to(it))
        .map(wrapped_rows)
});

impl<'a, T: PartialEq, G: SizedGrid<T>> SideView<'a, T, G> {
    pub fn new(inner: &'a G, side: Side) -> Self {
        Self {
            inner,
            transform: unsafe {
                // SAFETY: SIDE_VIEW_TRANSFORMS maps Side::ALL so every side index will yield a value
                let result = *SIDE_VIEW_TRANSFORMS.get_unchecked(side as usize);
                (result.0, result.1 * (inner.size() - UVec3::ONE).convert())
            },
            _phantom: PhantomData,
        }
    }

    pub fn transform_position(&self, pos: UVec3) -> UVec3 {
        (self.transform.0 * pos.convert() + self.transform.1).convert()
    }

    pub fn inverse_position(&self, pos: UVec3) -> UVec3 {
        (self.transform.0 * (pos.convert() - self.transform.1)).convert()
    }
}

impl<'a, T: PartialEq, G: SizedGrid<T>> SizedGrid<T> for SideView<'a, T, G> {
    #[inline(always)]
    fn size(&self) -> UVec3 {
        self.inner.size()
    }
    #[inline(always)]
    fn get_pos_key(&self, pos: UVec3) -> Option<ChunkValueIndex> {
        self.inner.get_pos_key(pos)
    }
    #[inline(always)]
    fn get_values(&self) -> &Vec<T> {
        self.inner.get_values()
    }
    #[inline(always)]
    fn get_position_index(&self, pos: UVec3) -> usize {
        self.inner.get_position_index(self.transform_position(pos))
    }
}

/// Used to slice into a [`ChunkStore`], yielding a 2D array.
///
/// Combine with [`SideView`] to allow slicing into arbitrary chunk sides.
pub struct SliceView<'a, T: PartialEq, G: SizedGrid<T>> {
    inner: &'a G,
    depth: u32,
    _phantom: PhantomData<T>,
}

impl<'a, T: PartialEq, G: SizedGrid<T>> SliceView<'a, T, G> {
    pub fn new(inner: &'a G, depth: u32) -> Self {
        if depth >= inner.size().z {
            panic!("depth is out of bounds");
        }
        SliceView {
            inner,
            depth,
            _phantom: PhantomData,
        }
    }
    pub fn try_new(inner: &'a G, depth: i32) -> Option<Self> {
        if depth > 0 && depth < inner.size().z as i32 {
            Some(SliceView {
                inner,
                depth: depth as u32,
                _phantom: PhantomData,
            })
        } else {
            None
        }
    }

    #[must_use]
    #[inline(always)]
    fn size(&self) -> UVec2 {
        self.inner.size().xy()
    }

    #[must_use]
    fn get_position_index(&self, pos: UVec2) -> usize {
        pos.x as usize
            + self.depth as usize * self.inner.size().x as usize
            + pos.y as usize * self.inner.size().x as usize * self.inner.size().z as usize
    }

    #[must_use]
    #[inline(always)]
    fn get_pos_key(&self, pos: UVec2) -> Option<ChunkValueIndex> {
        self.inner.get_pos_key(pos.extend(self.depth))
    }
    #[must_use]
    #[inline(always)]
    fn get_values(&self) -> &Vec<T> {
        self.inner.get_values()
    }
    #[must_use]
    fn index_of_value(&self, value: &T) -> Option<u16>
    where
        T: PartialEq,
    {
        self.get_values()
            .iter()
            .enumerate()
            .take(u16::MAX as usize - 1)
            .find_map(|(i, it)| {
                if it == value {
                    Some(i as u16 + 1)
                } else {
                    None
                }
            })
    }

    #[must_use]
    fn get_pos_value(&self, pos: UVec2) -> Option<&T> {
        if let Some(i) = self.get_pos_key(pos) {
            self.get_values().get(i as usize - 1)
        } else {
            None
        }
    }
}

/// Used to mutably slice into a [`ChunkStore`], yielding a 2D array.
///
/// Combine with [`SideView`] to allow slicing into arbitrary chunk sides.
pub struct SliceViewMut<'a, T: PartialEq, G: SizedGridMut<T>> {
    inner: &'a mut G,
    depth: u32,
    _phantom: PhantomData<T>,
}

impl<'a, T: PartialEq, G: SizedGridMut<T>> SliceViewMut<'a, T, G> {
    pub fn new(inner: &'a mut G, depth: u32) -> Self {
        if depth >= inner.size().z {
            panic!("depth is out of bounds");
        }
        SliceViewMut {
            inner,
            depth,
            _phantom: PhantomData,
        }
    }
    pub fn try_new(inner: &'a mut G, depth: u32) -> Option<Self> {
        if depth < inner.size().z {
            Some(SliceViewMut {
                inner,
                depth,
                _phantom: PhantomData,
            })
        } else {
            None
        }
    }

    #[must_use]
    #[inline(always)]
    fn size(&self) -> UVec2 {
        self.inner.size().xy()
    }

    #[must_use]
    fn get_position_index(&self, pos: UVec2) -> usize {
        pos.x as usize
            + self.depth as usize * self.inner.size().x as usize
            + pos.y as usize * self.inner.size().x as usize * self.inner.size().z as usize
    }

    #[must_use]
    #[inline(always)]
    fn get_pos_key(&self, pos: UVec2) -> Option<ChunkValueIndex> {
        self.inner.get_pos_key(pos.extend(self.depth))
    }
    #[must_use]
    #[inline(always)]
    fn get_pos_key_mut(&mut self, pos: UVec2) -> Option<&mut ChunkValueIndex> {
        self.inner.get_pos_key_mut(pos.extend(self.depth))
    }
    #[must_use]
    #[inline(always)]
    fn get_values(&self) -> &Vec<T> {
        self.inner.get_values()
    }
    #[must_use]
    #[inline(always)]
    fn get_values_mut(&mut self) -> &mut Vec<T> {
        self.inner.get_values_mut()
    }
    #[inline(always)]
    fn insert_key(&mut self, key: T) -> u16 {
        self.inner.insert_key(key)
    }
    #[must_use]
    fn index_of_value(&self, value: &T) -> Option<u16>
    where
        T: PartialEq,
    {
        self.get_values()
            .iter()
            .enumerate()
            .take(u16::MAX as usize - 1)
            .find_map(|(i, it)| {
                if it == value {
                    Some(i as u16 + 1)
                } else {
                    None
                }
            })
    }

    #[inline]
    fn set_pos_id(&mut self, pos: UVec2, value: ChunkValueIndex) {
        if let Some(pos) = self.get_pos_key_mut(pos) {
            *pos = value;
        }
    }

    #[must_use]
    fn get_pos_value(&self, pos: UVec2) -> Option<&T> {
        if let Some(i) = self.get_pos_key(pos) {
            self.get_values().get(i as usize - 1)
        } else {
            None
        }
    }

    fn set_pos_value(&mut self, pos: UVec2, value: Option<T>)
    where
        T: PartialEq,
    {
        if pos.x >= self.size().x || pos.y >= self.size().y {
            return;
        }
        let i = value
            .map(|it| {
                self.index_of_value(&it).unwrap_or_else(|| {
                    self.get_values_mut().push(it);
                    self.get_values().len() as u16
                })
            })
            .unwrap_or(0);

        self.set_pos_id(pos, i);
    }

    fn set_or_clone_pos_value(&mut self, pos: UVec2, value: Option<&T>)
    where
        T: PartialEq + Clone,
    {
        let i = value
            .map(|it| {
                self.index_of_value(it).unwrap_or_else(|| {
                    self.get_values_mut().push(it.clone());
                    self.get_values().len() as u16
                })
            })
            .unwrap_or(0);

        self.set_pos_id(pos, i);
    }
}

pub struct ChunkPos {
    pub value: UVec3,
    size: UVec3,
}

impl ChunkPos {
    pub fn new(value: UVec3, size: UVec3) -> Self {
        ChunkPos { value, size }
    }

    pub fn size(&self) -> UVec3 {
        self.size
    }
}

impl std::ops::Add for ChunkPos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        ChunkPos {
            value: self.value + rhs.value,
            size: self.size,
        }
    }
}

impl From<ChunkPos> for UVec3 {
    fn from(value: ChunkPos) -> Self {
        value.value
    }
}

impl From<ChunkPos> for IVec3 {
    fn from(value: ChunkPos) -> Self {
        value.value.convert()
    }
}

impl From<ChunkPos> for Vec3 {
    fn from(value: ChunkPos) -> Self {
        value.value.convert()
    }
}
