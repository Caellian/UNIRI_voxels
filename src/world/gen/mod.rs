pub mod old;

use crate::world::chunk::ChunkStore;
use crate::MaterialID;
use bevy::prelude::*;

use super::chunk::SizedGrid as _;

pub trait TerrainGenerator<T: PartialEq> {
    fn generate(&self, pos: Vec3, blocks: &mut ChunkStore<T>);
}

pub enum WriteMode {
    /// Write only over existing content
    Color,
    /// Write over any existing content
    Replace,
    /// Add non-destructively to existing content
    Masked,
}

pub struct Fill {
    pub material: MaterialID,
}

impl TerrainGenerator<MaterialID> for Fill {
    fn generate(&self, _pos: Vec3, blocks: &mut ChunkStore<MaterialID>) {
        let id = blocks.insert_key(self.material.clone());
        for y in 0..(blocks.size.y) {
            for x in 0..(blocks.size.x) {
                for z in 0..(blocks.size.z) {
                    blocks.set_pos_id(UVec3::new(x, y, z), id)
                }
            }
        }
    }
}
