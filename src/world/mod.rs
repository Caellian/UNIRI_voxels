use crate::data::LoadedMaterials;
use crate::entity::player::PlayerChunk;
use crate::world::chunk::chunk_material::ChunkMaterial;
use crate::world::chunk::{ChunkInfo, Mesher};
use bevy::prelude::*;
use rand::RngCore;

use self::chunk::{mesh::greedy_mesh, ChunkStore};
use self::gen::old::SimplexChunkGen;
use self::material::MaterialID;

pub mod chunk;
pub mod gen;
pub mod material;
pub mod vox;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u8)]
pub enum WorldAxis {
    X = 0,
    Y = 1,
    Z = 2,
}

impl WorldAxis {
    #[inline]
    pub const fn to_vec(self) -> Vec3 {
        [
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
        ][self as usize]
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
    spatial: SpatialBundle,
}

impl Default for World {
    fn default() -> Self {
        World {
            info: WorldInfo::default(),
            spatial: SpatialBundle {
                visibility: Visibility::Visible,
                transform: Transform::IDENTITY,
                global_transform: GlobalTransform::IDENTITY,
                ..default()
            },
        }
    }
}

pub fn spawn_world(
    mut commands: Commands,
    mut _materials: ResMut<Assets<ChunkMaterial>>,
    _asset_server: Res<AssetServer>,
) {
    /*
        let model: Handle<VoxelData> = asset_server.load("models/monu3.vox");

        commands.spawn(World { ..default() }).with_children(|c| {
            c.spawn((model, SpatialBundle::default()));
        });
    */

    let grass = MaterialID::new("common:grass");

    let gen = SimplexChunkGen {
        seed: 34356424,
        dirt_height: 3,
    };

    let ch = {
        let mut ch = Chunk::new_gen(Vec3::new(0.0, 0.0, 0.0), UVec3::new(16, 16, 16), &gen);

        for z in 0..16 {
            for y in 0..16 {
                for x in 0..16 {
                    if x == y && y == z {
                        ch.blocks.set_pos_value(UVec3::new(x, y, z), None);
                    }
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

    // TODO: Inserting same chunk material multiple times
    commands.spawn(World::default()).with_children(|c| {
        c.spawn(ch);
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            color: Color::hex("cceecc").unwrap(),
            illuminance: 10000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0., 1000.0, 0.),
            rotation: Quat::from_euler(EulerRot::XYZ, 20., 0., 0.),
            ..default()
        },
        ..default()
    });

    /*
        commands.spawn(PointLightBundle {
            point_light: PointLight {
                intensity: 1500.0,
                shadows_enabled: true,
                ..default()
            },
            transform: Transform::from_xyz(-4.0, 8.0, 4.0),
            ..default()
        });
    */
}

pub fn spawn_chunk_markers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let corner = meshes.add(Mesh::from(Cuboid::new(0.5, 0.5, 0.5)));
    let corner_mat = materials.add(StandardMaterial {
        base_color: Color::GREEN,
        ..default()
    });

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
    mut unbuilt: Query<(Entity, &ChunkInfo, &ChunkStore<MaterialID>), Without<ChunkMesh>>,

    mut meshes: ResMut<Assets<Mesh>>,
    mut chunk_material_assets: ResMut<Assets<ChunkMaterial>>,

    materials: Res<LoadedMaterials>,
) {
    for (chunk, info, store) in unbuilt.iter_mut() {
        let meshes = match info.mesher {
            Mesher::Greedy => greedy_mesh(store, &materials),
        };

        let (mesh, face_properties) = mesh_builder.build(store, &materials);

        commands
            .entity(chunk)
            .insert(chunk_material_assets.add(ChunkMaterial {
                face_properties,
                ..default()
            }))
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

#[derive(Debug, Bundle)]
pub struct Chunk {
    pub info: ChunkInfo,
    pub blocks: ChunkStore<MaterialID>,
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
        generator: &G,
    ) -> Chunk {
        let mut result = Chunk::new(pos, size);
        generator.generate(pos, &mut result.blocks);
        result
    }
}
