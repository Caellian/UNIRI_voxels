use crate::data::LoadedBlocks;
use crate::entity::player::PlayerChunk;
use crate::world::chunk::chunk_material::ChunkMaterial;
use crate::world::chunk::{Chunk, ChunkInfo, Mesher};
use crate::world::mesh::ChunkMesh;
use bevy::prelude::shape::Cube;
use bevy::prelude::*;
use rand::RngCore;

use self::block::MaterialID;
use self::chunk::ChunkStore;
use self::gen::Fill;
use self::mesh::greedy_mesh;

pub mod block;
pub mod chunk;
pub mod gen;
pub mod info;
pub mod mesh;
pub mod vox;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum WorldAxis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl WorldAxis {
    pub const fn to_vec(self) -> Vec3 {
        match self {
            WorldAxis::X => Vec3::new(1.0, 0.0, 0.0),
            WorldAxis::Y => Vec3::new(0.0, 1.0, 0.0),
            WorldAxis::Z => Vec3::new(0.0, 0.0, 1.0),
        }
    }

    pub const fn slice_plane(self) -> [WorldAxis; 2] {
        match self {
            WorldAxis::X => [WorldAxis::Y, WorldAxis::Z],
            WorldAxis::Y => [WorldAxis::X, WorldAxis::Z],
            WorldAxis::Z => [WorldAxis::X, WorldAxis::Y],
        }
    }
}

#[derive(Debug, Component)]
pub struct WorldInfo {
    pub seed: u32,
    pub chunk_size: UVec3,
}

impl Default for WorldInfo {
    fn default() -> Self {
        let mut r = rand::thread_rng();

        WorldInfo {
            seed: r.next_u32(),
            chunk_size: UVec3::new(32, 32, 32),
        }
    }
}

#[derive(Debug, Bundle)]
pub struct World {
    info: WorldInfo,
    #[bundle]
    spatial: SpatialBundle,
}

impl Default for World {
    fn default() -> Self {
        World {
            info: WorldInfo::default(),
            spatial: SpatialBundle {
                visibility: Visibility::Visible,
                computed: ComputedVisibility::HIDDEN,
                transform: Transform::IDENTITY,
                global_transform: GlobalTransform::IDENTITY,
            },
        }
    }
}

pub fn spawn_world(
    mut commands: Commands,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    /*
        let model: Handle<VoxelData> = asset_server.load("models/monu3.vox");

        commands.spawn(World { ..default() }).with_children(|c| {
            c.spawn((model, SpatialBundle::default()));
        });
    */

    let stone = MaterialID::new("common:stone");
    let grass = MaterialID::new("common:grass");

    let mut fill = Fill { material: stone };

    let ch = {
        let mut ch = Chunk::new_gen(Vec3::new(0.0, 0.0, 0.0), UVec3::new(16, 16, 16), &mut fill);

        for z in 0..16 {
            for y in 0..16 {
                for x in 0..16 {
                    if x == y && y == z {
                        ch.blocks.set_value(UVec3::new(x, y, z), None);
                    }
                }
            }
        }

        for z in 0..16 {
            for y in 7..9 {
                for x in 7..9 {
                    ch.blocks
                        .set_or_clone_value(UVec3::new(x, y, z), Some(&grass));
                }
            }
        }

        //let mut ch = Chunk::new(Vec3::new(0.0, 0.0, 0.0), UVec3::new(16, 16, 16));

        // for z in 0..16 {
        //     for y in 0..16 {
        //         for x in 0..16 {
        //             if x == y && y == z {
        //                 ch.blocks
        //                     .set_value(UVec3::new(x, y, z), Some(MaterialID::new("common:stone")));
        //             }
        //         }
        //     }
        // }

        // ch.blocks
        //     .set_value(UVec3::new(5, 5, 5), Some(MaterialID::new("common:stone")));

        ch
    };

    let cm = materials.add(ChunkMaterial {});

    // TODO: Inserting same chunk material multiple times
    commands.spawn(World::default()).with_children(|c| {
        c.spawn(ch).insert(cm.clone());
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::hex("cceecc").unwrap(),
            illuminance: 800.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 20., 0., 0.)),
        ..default()
    });

    // commands.spawn(PointLightBundle {
    //     point_light: PointLight {
    //         intensity: 1500.0,
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform::from_xyz(-4.0, 8.0, 4.0),
    //     ..default()
    // });
}

pub fn spawn_chunk_markers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let corner = meshes.add(Mesh::from(Cube::new(0.5)));
    let corner_mat = materials.add(Color::GREEN.into());

    commands.spawn(PbrBundle {
        mesh: corner.clone(),
        material: corner_mat.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: corner.clone(),
        material: corner_mat.clone(),
        transform: Transform::from_translation(Vec3::new(16.0, 0.0, 0.0)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: corner.clone(),
        material: corner_mat.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 16.0, 0.0)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: corner.clone(),
        material: corner_mat.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 16.0)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: corner.clone(),
        material: corner_mat.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 16.0, 16.0)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: corner.clone(),
        material: corner_mat.clone(),
        transform: Transform::from_translation(Vec3::new(16.0, 0.0, 16.0)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: corner.clone(),
        material: corner_mat.clone(),
        transform: Transform::from_translation(Vec3::new(16.0, 16.0, 0.0)),
        ..default()
    });
    commands.spawn(PbrBundle {
        mesh: corner,
        material: corner_mat,
        transform: Transform::from_translation(Vec3::new(16.0, 16.0, 16.0)),
        ..default()
    });
}

pub fn build_fresh_chunks(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ChunkMaterial>>,
    loaded: Res<LoadedBlocks>,
    mut unbuilt: Query<(Entity, &ChunkInfo, &ChunkStore<MaterialID>), Without<ChunkMesh>>,
) {
    for (chunk, info, store) in unbuilt.iter_mut() {
        let mesh = match info.mesher {
            Mesher::Greedy => greedy_mesh(store, &loaded),
        };

        commands
            .entity(chunk)
            .insert(meshes.add(mesh))
            .insert(ChunkMesh { dirty: false })
            .insert(Visibility::Visible);
    }
}

pub fn track_player_chunk(
    mut query: Query<(&Transform, &mut PlayerChunk), Changed<Transform>>,
    world_info: Query<&WorldInfo>,
) {
    let (t, chunk) = query.single();
    let pos = t.translation;

    if let Ok(world) = world_info.get_single() {
        let new_x = (pos.x / world.chunk_size.x as f32) as u32;
        let new_y = (pos.y / world.chunk_size.y as f32) as u32;
        let new_z = (pos.z / world.chunk_size.z as f32) as u32;

        if chunk.x != new_x || chunk.y != new_y || chunk.z != new_z {
            let mut chunk = query.single_mut().1;
            chunk.x = new_x;
            chunk.y = new_y;
            chunk.z = new_z;
        }
    }
}

pub fn on_chunk_change(
    _commands: Commands,
    _player_chunk: Query<&PlayerChunk, Changed<PlayerChunk>>,
    _chunks: Query<(Entity, &Transform, &ChunkMesh)>,
) {
}
