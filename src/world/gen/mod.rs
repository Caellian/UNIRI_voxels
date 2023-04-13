pub mod script;

use crate::world::chunk::ChunkStore;
use crate::MaterialID;
use bevy::prelude::*;

pub trait TerrainGenerator<T: PartialEq> {
    fn generate(&mut self, blocks: &mut ChunkStore<T>);
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
    fn generate(&mut self, blocks: &mut ChunkStore<MaterialID>) {
        for y in 0..(blocks.size.y) {
            for x in 0..(blocks.size.x) {
                for z in 0..(blocks.size.z) {
                    blocks.set_or_clone_value(UVec3::new(x, y, z), Some(&self.material))
                }
            }
        }
    }
}
