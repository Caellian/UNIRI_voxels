pub mod color;
pub mod data;
pub mod entity;
pub mod error;
pub mod ext;
pub mod math;
pub mod ui;
pub mod world;

use crate::world::block::{BlockProperties, MaterialID};
use crate::world::render::ChunkMaterial;
use crate::world::vox::{VoxLoader, VoxelData};
use bevy::prelude::*;
use bevy::render::mesh;
use entity::player::fly_cam::FlyCameraPlugin;
use wgpu::PrimitiveTopology;

pub static NAME: &str = env!("CARGO_BIN_NAME");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: format!("{} - v{}", NAME, VERSION),
                ..default()
            }),
            ..default()
        }))
        .add_plugin(FlyCameraPlugin)
        .add_plugin(MaterialPlugin::<ChunkMaterial>::default())
        .add_asset::<VoxelData>()
        .add_asset_loader(VoxLoader)
        .add_startup_system(data::load_content)
        .add_startup_system(entity::player::spawn_player)
        .add_startup_system(world::spawn_world)
        //.add_startup_system(world::spawn_chunk_markers)
        //.add_startup_system(ui::debug::setup)
        //.add_system(world::mesh::rebuild_meshes)
        //.add_startup_system(build_triangle)
        .add_system(world::build_fresh_chunks)
        //.add_system(world::track_player_chunk)
        .run();
}
