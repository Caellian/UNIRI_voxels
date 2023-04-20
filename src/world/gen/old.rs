use bevy::prelude::*;
use noise::{Fbm, NoiseFn, Simplex};

use crate::world::material::MaterialID;

use super::{Fill, TerrainGenerator};

pub struct SimplexChunkGen {
    pub seed: u32,
    pub dirt_height: u8,
}

const FILL_STONE: Fill = Fill {
    material: MaterialID::Static("common:stone"),
};

impl TerrainGenerator<MaterialID> for SimplexChunkGen {
    fn generate(&self, pos: Vec3, blocks: &mut crate::world::chunk::ChunkStore<MaterialID>) {
        let fbm: Fbm<Simplex> = Fbm::new(self.seed);

        if pos.y < 0. {
            FILL_STONE.generate(pos, blocks);
        }

        let stone = blocks.insert_key(MaterialID::Static("common:stone"));
        let dirt = blocks.insert_key(MaterialID::Static("common:dirt"));
        let grass = blocks.insert_key(MaterialID::Static("common:grass"));

        for z in 0..blocks.size.z {
            for x in 0..blocks.size.x {
                let precise_height = fbm.get([
                    pos.x as f64 * blocks.size.x as f64 + x as f64,
                    pos.z as f64 * blocks.size.z as f64 + z as f64,
                ]);

                let max_height = blocks.size.y - self.dirt_height as u32;
                let height = (precise_height * max_height as f64) as u32;

                for y in 0..height {
                    blocks.set_pos_id(UVec3::new(x, y, z), stone);
                }
                for y in height..(height + self.dirt_height as u32 - 1) {
                    blocks.set_pos_id(UVec3::new(x, y, z), dirt);
                }
                blocks.set_pos_id(
                    UVec3::new(x, height + self.dirt_height as u32 - 1, z),
                    grass,
                );
            }
        }
    }
}
