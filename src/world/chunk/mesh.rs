use crate::data::{FaceProperties, LoadedMaterials};
use crate::ext::{Convert, VecExt};
use crate::world::chunk::ChunkStore;
use crate::world::material::Side;
use crate::MaterialID;
use bevy::render::mesh::*;
use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use indexmap::IndexSet;
use std::hash::{Hash, Hasher};

use super::chunk_material::ChunkMaterial;

pub fn visible_chunk_sides(player_pos: Vec3, chunk_pos: Vec3) -> [Side; 3] {
    [
        if player_pos.x > chunk_pos.x {
            Side::East
        } else {
            Side::West
        },
        if player_pos.y > chunk_pos.y {
            Side::Top
        } else {
            Side::Bottom
        },
        if player_pos.z > chunk_pos.z {
            Side::South
        } else {
            Side::North
        },
    ]
}

#[derive(Debug, Clone, Component)]
#[repr(transparent)]
pub struct ChunkMesh {
    pub dirty: bool,
}

/// TODO: Meshing optimizations:
/// - Runs for each side, can run once per axis
pub fn greedy_mesh(blocks: &ChunkStore<MaterialID>, loaded: &LoadedMaterials) -> MeshBuilder {
    let mut mesh_builder = MeshBuilder::new();

    for side in Side::ALL {
        let mut ids_above = None;
        let max_depth = (side.axis().to_vec().convert() * blocks.size).sum();

        let [slice_plane_x, slice_plane_y] = side.axis().slice_plane();
        let (size_x, size_y) = (
            (slice_plane_x.to_vec().convert() * blocks.size).sum(),
            (slice_plane_y.to_vec().convert() * blocks.size).sum(),
        );

        for depth in 0..max_depth {
            let mut visited = vec![vec![false; size_x as usize]; size_y as usize];
            let current = {
                if let Some(slice) = blocks.get_side_slice(side, depth) {
                    slice
                } else {
                    continue;
                }
            };

            for y in 0..size_y {
                for x in 0..size_x {
                    if !is_block_face_visible(&visited, &current, ids_above.as_ref(), x, y) {
                        continue;
                    }

                    let id = current[y as usize][x as usize];

                    let (w, h) = {
                        let mut w = 1;
                        for w_test in 1..(size_x - x) {
                            let visible = is_block_face_visible(
                                &visited,
                                &current,
                                ids_above.as_ref(),
                                x + w_test,
                                y,
                            );
                            if !visible || current[y as usize][(x + w_test) as usize] != id {
                                break;
                            }
                            w = w_test + 1;
                        }

                        let mut h = 1;
                        for h_test in 1..(size_y - y) {
                            let mut test = true;
                            for quad_x in x..(x + w) {
                                let visible = is_block_face_visible(
                                    &visited,
                                    &current,
                                    ids_above.as_ref(),
                                    quad_x,
                                    y + h_test,
                                );
                                if !visible || current[quad_x as usize][(y + h_test) as usize] != id
                                {
                                    test = false;
                                    break;
                                }
                            }
                            if !test {
                                break;
                            }

                            h = h_test + 1;
                        }

                        (w, h)
                    };

                    for y in y..(y + h) {
                        for x in x..(x + w) {
                            visited[y as usize][x as usize] = true;
                        }
                    }
                    /*
                    let color = blocks
                        .value_of_index(id)
                        .and_then(|id| loaded.get(id).map(|props| props.base_color))
                        .unwrap_or(Vec4::new(0.1, 0.3, 0.8, 1.0));
                    */

                    // id + side ---insert--> buffer
                    // tex_id

                    let corners = [
                        side.depth_pos(blocks.size, depth, UVec2::new(x, y)),
                        side.depth_pos(blocks.size, depth, UVec2::new(x + w, y)),
                        side.depth_pos(blocks.size, depth, UVec2::new(x, y + h)),
                        side.depth_pos(blocks.size, depth, UVec2::new(x + w, y + h)),
                    ];

                    mesh_builder.push_face(id, side, corners)
                }
            }

            ids_above = Some(current);
        }
    }

    mesh_builder
}

// TODO: Post terrain gen figure out optimal mesh vec capacity
#[inline(always)] // avoids jumps in greedy meshing
fn is_block_face_visible(
    visited: &[Vec<bool>],
    ids: &[Vec<u16>],
    above: Option<&Vec<Vec<u16>>>,
    x: u32,
    y: u32,
) -> bool {
    let x = x as usize;
    let y = y as usize;

    if visited[y][x] {
        return false;
    }

    let id = ids[y][x];
    if id == 0 {
        return false;
    }

    if let Some(above) = above {
        let id_above = above[y][x];

        if id_above != 0 {
            return false;
        }
    }

    true
}

#[derive(Debug, Clone, PartialEq)]
pub struct StagedVertex {
    position: UVec3,
    normal: Vec3,
    uv: Vec2,
    material_side: (u16, Side),
}
impl Eq for StagedVertex {}
impl Hash for StagedVertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // We don't care about correctness of handling floats for quickly
        // checking whether vertices match.
        unsafe {
            let s: *const StagedVertex = self;
            state.write(std::slice::from_raw_parts(
                s as *const u8,
                std::mem::size_of::<StagedVertex>(),
            ))
        }
    }
}

const MISSING_VOXEL_FACE: FaceProperties = FaceProperties {
    base_color: Vec4::new(0.86, 0.08, 0.24, 1.),
    roughness: 0.8,
    metallic: 0.2,
    reflectance: 0.9,
    base_texture: 0,
    base_texture_uv: Vec2::new(0., 0.),
    emissive_color: Vec4::new(0., 0., 0., 1.),
};

#[derive(Debug, Clone)]
pub struct MeshBuilder {
    vertices: IndexSet<StagedVertex, ahash::RandomState>,
    indices: Vec<u32>,
}

impl Default for MeshBuilder {
    fn default() -> Self {
        MeshBuilder {
            vertices: IndexSet::with_capacity_and_hasher(64, ahash::RandomState::new()),
            indices: Vec::with_capacity(128),
        }
    }
}

impl MeshBuilder {
    pub fn new() -> MeshBuilder {
        MeshBuilder::default()
    }

    pub fn push(&mut self, v: StagedVertex) {
        let i = self.vertices.insert_full(v).0;
        self.indices.push(i as u32);
    }

    pub fn push_face(&mut self, id: u16, side: Side, corners: [UVec3; 4]) {
        self.push(StagedVertex {
            position: corners[0],
            normal: side.normal(),
            uv: Vec2::new(0.0, 0.0),
            material_side: (id, side),
        });
        self.push(StagedVertex {
            position: corners[1],
            normal: side.normal(),
            uv: Vec2::new(1.0, 0.0),
            material_side: (id, side),
        });
        self.push(StagedVertex {
            position: corners[2],
            normal: side.normal(),
            uv: Vec2::new(0.0, 1.0),
            material_side: (id, side),
        });
        self.push(StagedVertex {
            position: corners[1],
            normal: side.normal(),
            uv: Vec2::new(1.0, 0.0),
            material_side: (id, side),
        });
        self.push(StagedVertex {
            position: corners[3],
            normal: side.normal(),
            uv: Vec2::new(1.0, 1.0),
            material_side: (id, side),
        });
        self.push(StagedVertex {
            position: corners[2],
            normal: side.normal(),
            uv: Vec2::new(0.0, 1.0),
            material_side: (id, side),
        });
    }

    pub fn build(
        self,
        chunk: &ChunkStore<MaterialID>,
        materials: &LoadedMaterials,
    ) -> (Mesh, Vec<FaceProperties>) {
        let mut face_properties: IndexSet<&FaceProperties> =
            IndexSet::with_capacity(self.vertices.len() / 8);
        face_properties.insert(&MISSING_VOXEL_FACE);

        let mut positions = Vec::with_capacity(self.vertices.len());
        let mut normals = Vec::with_capacity(self.vertices.len());
        let mut uvs = Vec::with_capacity(self.vertices.len());
        let mut face_indices: Vec<u32> = Vec::with_capacity(self.vertices.len() / 8);

        for StagedVertex {
            position,
            normal,
            uv,
            material_side,
        } in &self.vertices
        {
            positions.push(position.as_vec3().to_array());
            normals.push(normal.to_array());
            uvs.push(uv.to_array());

            let id = chunk
                .value_of_index(material_side.0)
                .expect("invalid chunk storage value index");

            if let Some(properties) = materials.properties.get(id) {
                let key = match properties.faces.as_ref().unwrap() {
                    crate::data::BlockFaces::Uniform { face } => face_properties.insert_full(face),
                    crate::data::BlockFaces::Sided {
                        face,
                        face_override,
                    } => {
                        if let Some(f) = face_override.get(&material_side.1) {
                            face_properties.insert_full(f)
                        } else {
                            face_properties.insert_full(face)
                        }
                    }
                };
                face_indices.push(key.0 as u32);
            } else {
                panic!("unable to get material properties for id ({})", id);
            }
        }
        face_properties.shrink_to_fit();
        tracing::info!("Generated face properties: {:#?}", face_properties);

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::all());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.insert_attribute(ChunkMaterial::ATTRIBUTE_FACE_INDEX, face_indices);
        mesh.insert_indices(Indices::U32(self.indices));

        (mesh, face_properties.into_iter().cloned().collect())
    }
}
