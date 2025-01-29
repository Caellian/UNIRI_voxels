use std::ptr::addr_of;

use bevy::prelude::*;

pub use view::*;

use crate::math::side::Side;
use crate::math::vec::IsVec;
use crate::util::MybOwned;

pub mod chunk_material;
pub mod mesh;
pub mod view;

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
/// Chunk values are stored in x, z, y order in a contiguous block of memory.
/// Default (front-facing) [`Side`] of a chunk is [`south`](Side::South).
#[derive(Debug, Default, Component)]
pub struct ChunkStore<T: PartialEq> {
    /// Out of order list of values stored in this store
    pub values: Vec<T>,
    /// Dimensions of the store used to convert [`UVec3`] coordinates into [`content`] index.
    pub size: UVec3,
    /// Ordered sequence of value indices.
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

pub trait SizedGrid<'d, T>
where
    T: 'd,
{
    #[must_use]
    fn size(&self) -> UVec3;

    #[must_use]
    fn get_position_index(&self, pos: UVec3) -> usize;

    #[must_use]
    fn get_pos_key(&self, pos: UVec3) -> Option<ChunkValueIndex>;
    #[must_use]
    fn values(&self) -> Vec<&'d T>;
    #[must_use]
    fn index_of_value(&self, value: &T) -> Option<u16>
    where
        T: PartialEq,
    {
        self.values()
            .iter()
            .enumerate()
            .take(u16::MAX as usize - 1)
            .find_map(|(i, it)| {
                if *it == value {
                    Some(i as u16 + 1)
                } else {
                    None
                }
            })
    }
    #[must_use]
    fn value_of_index(&self, index: u16) -> Option<&'d T> {
        if index == 0 {
            None
        } else {
            self.values().get(index as usize - 1).map(|it| *it)
        }
    }
    #[must_use]
    fn get_pos_value(&self, pos: UVec3) -> Option<&'d T> {
        if let Some(i) = self.get_pos_key(pos) {
            self.values().get(i as usize - 1).map(|it| *it)
        } else {
            None
        }
    }
}

pub trait SizedGridMut<'d, T>: SizedGrid<'d, T>
where
    T: 'd,
{
    fn get_pos_key_mut(&mut self, pos: UVec3) -> Option<&mut ChunkValueIndex>;
    #[must_use]
    fn value_list_mut(&mut self) -> &mut Vec<T>;
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
                    self.value_list_mut().push(it);
                    self.values().len() as u16
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
                    self.value_list_mut().push(it.clone());
                    self.values().len() as u16
                })
            })
            .unwrap_or(0);

        self.set_pos_id(pos, i);
    }
}

impl<'d, T: PartialEq + 'd> SizedGrid<'d, T> for ChunkStore<T> {
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
    fn values(&self) -> Vec<&'d T> {
        let mut result = Vec::with_capacity(self.values.len());
        for v in &self.values {
            unsafe {
                // SAFETY: Data is guaranteed to be valid for 'd
                let d = addr_of!(*v);
                result.push(d.as_ref().unwrap_unchecked())
            }
        }
        result
    }
}

impl<'d, T: PartialEq + 'd> SizedGridMut<'d, T> for ChunkStore<T> {
    #[inline]
    fn get_pos_key_mut(&mut self, pos: UVec3) -> Option<&mut ChunkValueIndex> {
        let i = self.get_position_index(pos);
        self.content.get_mut(i)
    }
    #[inline(always)]
    fn value_list_mut(&mut self) -> &mut Vec<T> {
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
