pub mod arguments;
pub mod color;
pub mod data;
pub mod entity;
pub mod error;
pub mod ext;
pub mod math;
pub mod ui;
pub mod util;
pub mod world;

use crate::world::chunk::chunk_material::{ChunkMaterial, CHUNK_SHADER_HANDLE};
use crate::world::material::MaterialID;
use crate::world::vox::{Vox, VoxLoader};
use bevy::asset::load_internal_asset;
use bevy::prelude::*;

use clap::Parser;
use entity::player::fly_cam::FlyCameraPlugin;

pub static NAME: &str = env!("CARGO_BIN_NAME");
pub static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let mut app = App::new();

    app.insert_resource(arguments::Context::parse());

    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: format!("{} - v{}", NAME, VERSION),
            ..default()
        }),
        ..default()
    }));

    #[cfg(not(feature = "debug"))]
    load_internal_asset!(
        app,
        CHUNK_SHADER_HANDLE,
        "world/chunk/chunk_shader.wgsl",
        Shader::from_wgsl
    );
    // TODO: register shader type

    app.add_plugins(FlyCameraPlugin)
        .add_plugins(MaterialPlugin::<ChunkMaterial>::default())
        .register_asset_loader(VoxLoader)
        .init_asset::<Vox>()
        .add_systems(Startup, data::load_content)
        .add_systems(Startup, entity::player::spawn_player)
        .add_systems(Startup, world::spawn_world)
        //.add_startup_system(world::spawn_chunk_markers)
        //.add_startup_system(ui::debug::setup)
        //.add_system(world::mesh::rebuild_meshes)
        //.add_startup_system(build_triangle)
        .add_systems(Update, world::build_fresh_chunks);
    //.add_system(world::track_player_chunk)

    app.run();
}
