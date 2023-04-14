use crate::ext::{Convert, VecExt};
use crate::world::block::Side;
use crate::world::gen::TerrainGenerator;
use crate::MaterialID;
use bevy::prelude::*;

pub mod chunk_material;

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
pub const MAX_CHUNK_VALUES: usize = (ChunkValueIndex::MAX - 1) as usize;

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

    #[inline]
    fn get_position_index(&self, pos: UVec3) -> usize {
        pos.x as usize
            + pos.z as usize * self.size.x as usize
            + pos.y as usize * self.size.x as usize * self.size.z as usize
    }

    fn insert_key(&mut self, key: T) {
        if self.values.len() + 1 > MAX_CHUNK_VALUES {
            panic!("overfull chunk palette");
        }
        self.values.push(key)
    }

    pub fn get_side_slice_pos(&self, side: Side, depth: u32, x: u32, y: u32) -> u16 {
        tracing::trace!(
            "get_side_slice_pos(side:{}, depth:{}, x:{}, y:{}) [size: ({},{},{})]",
            side,
            depth,
            x,
            y,
            self.size.x,
            self.size.y,
            self.size.z,
        );

        let i = self.get_position_index(match side {
            Side::Top => UVec3::new(y, self.size.y - 1 - depth, x),
            Side::Bottom => UVec3::new(self.size.x - 1 - y, depth, x),
            Side::West => UVec3::new(self.size.x - 1 - x, y, depth),
            Side::East => UVec3::new(x, y, self.size.z - 1 - depth),
            Side::North => UVec3::new(self.size.x - 1 - depth, y, self.size.z - 1 - x),
            Side::South => UVec3::new(depth, y, x),
        });

        self.content.get(i).cloned().unwrap_or(0)
    }

    #[must_use]
    pub fn value_of_index(&self, index: u16) -> Option<&T> {
        if index == 0 {
            None
        } else {
            self.values.get(index as usize - 1)
        }
    }

    #[must_use]
    pub fn index_of_value(&self, value: &T) -> Option<u16> {
        self.values
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

    pub fn as_ref<'a>(&'a self) -> ChunkStore<&'a T> {
        self.map(|it: &'a T| it)
    }

    pub fn get_value(&self, pos: UVec3) -> Option<&T> {
        if let Some(i) = self.content.get(self.get_position_index(pos)).cloned() {
            self.values.get(i as usize - 1)
        } else {
            None
        }
    }

    pub fn set_value(&mut self, pos: UVec3, value: Option<T>) {
        if pos.x >= self.size.x || pos.y >= self.size.y || pos.z >= self.size.z {
            self.as_ref();
            // TODO: resize chunk
        }
        let i = value
            .map(|it| {
                self.index_of_value(&it).unwrap_or_else(|| {
                    self.values.push(it);
                    self.values.len() as u16
                })
            })
            .unwrap_or(0);

        let pos_i = self.get_position_index(pos);
        if let Some(pos) = self.content.get_mut(pos_i) {
            *pos = i;
        }
    }

    pub fn set_or_clone_value(&mut self, pos: UVec3, value: Option<&T>)
    where
        T: Clone,
    {
        let i = value
            .map(|it| {
                self.index_of_value(it).unwrap_or_else(|| {
                    self.values.push(it.clone());
                    self.values.len() as u16
                })
            })
            .unwrap_or(0);

        let pos_i = self.get_position_index(pos);
        if let Some(pos) = self.content.get_mut(pos_i) {
            *pos = i;
        }
    }

    pub fn get_side_slice(&self, side: Side, depth: u32) -> Option<Vec<Vec<u16>>> {
        tracing::trace!(
            "get_side_slice(side:{}, depth:{}) [size: ({},{},{})]",
            side,
            depth,
            self.size.x,
            self.size.y,
            self.size.z,
        );
        if (side.axis().to_vec().convert() * self.size).sum() <= depth {
            return None;
        }

        let mut result = vec![vec![0; self.size.y as usize]; self.size.x as usize];

        for y in 0..self.size.y {
            for x in 0..self.size.x {
                result[y as usize][x as usize] = self.get_side_slice_pos(side, depth, x, y)
            }
        }

        Some(result)
    }
}

#[derive(Debug, Bundle)]
pub struct Chunk {
    pub info: ChunkInfo,
    pub blocks: ChunkStore<MaterialID>,
    #[bundle]
    pub spatial: SpatialBundle,
}

impl Chunk {
    pub fn new(pos: Vec3, size: UVec3) -> Chunk {
        Chunk {
            info: ChunkInfo {
                mesher: Mesher::Greedy,
            },
            blocks: ChunkStore::new(size),
            spatial: SpatialBundle {
                visibility: Visibility::Hidden,
                transform: Transform::from_translation(pos),
                ..default()
            },
        }
    }

    pub fn new_gen<G: TerrainGenerator<MaterialID>>(
        pos: Vec3,
        size: UVec3,
        generator: &mut G,
    ) -> Chunk {
        let mut result = Chunk::new(pos, size);
        generator.generate(&mut result.blocks);
        result
    }
}
