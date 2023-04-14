use crate::data::LoadedBlocks;
use crate::ext::{Convert, VecExt};
use crate::world::block::Side;
use crate::world::chunk::{ChunkInfo, ChunkStore, Mesher};
use crate::MaterialID;
use bevy::prelude::*;
use bevy::render::mesh::*;
use indexmap::IndexSet;
use std::hash::{Hash, Hasher};

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
pub fn greedy_mesh(blocks: &ChunkStore<MaterialID>, loaded: &LoadedBlocks) -> Mesh {
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

                    let color = blocks
                        .value_of_index(id)
                        .and_then(|id| loaded.get(id).map(|props| props.color))
                        .map(|c| c.as_rgba_f32())
                        .unwrap_or([0.1, 0.3, 0.8, 1.0]);

                    mesh_builder.push(Vertex {
                        position: side.depth_pos(depth, x, y, blocks.size).to_array(),
                        normal: side.normal().to_array(),
                        uv: [0.0, 0.0],
                        color,
                    });
                    mesh_builder.push(Vertex {
                        position: side.depth_pos(depth, x + w, y, blocks.size).to_array(),
                        normal: side.normal().to_array(),
                        uv: [1.0, 0.0],
                        color,
                    });
                    mesh_builder.push(Vertex {
                        position: side.depth_pos(depth, x, y + h, blocks.size).to_array(),
                        normal: side.normal().to_array(),
                        uv: [0.0, 1.0],
                        color,
                    });
                    mesh_builder.push(Vertex {
                        position: side.depth_pos(depth, x + w, y, blocks.size).to_array(),
                        normal: side.normal().to_array(),
                        uv: [1.0, 0.0],
                        color,
                    });
                    mesh_builder.push(Vertex {
                        position: side.depth_pos(depth, x + w, y + h, blocks.size).to_array(),
                        normal: [1.0, 1.0, 0.0],
                        uv: [1.0, 1.0],
                        color,
                    });
                    mesh_builder.push(Vertex {
                        position: side.depth_pos(depth, x, y + h, blocks.size).to_array(),
                        normal: [1.0, 1.0, 0.0],
                        uv: [0.0, 1.0],
                        color,
                    });
                }
            }

            ids_above = Some(current);
        }
    }

    mesh_builder.build()
}

// TODO: Post terrain gen figure out optimal mesh vec capacity
#[inline(always)] // avoids jumps in greedy meshing
fn is_block_face_visible(
    visited: &Vec<Vec<bool>>,
    ids: &Vec<Vec<u16>>,
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
pub struct Vertex {
    position: [f32; 3],
    normal: [f32; 3],
    uv: [f32; 2],
    color: [f32; 4],
}
impl Eq for Vertex {}
impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // We don't care about correctness of handling floats for quickly
        // checking whether vertices match.
        unsafe {
            let s: *const Vertex = self;
            state.write(std::slice::from_raw_parts(
                s as *const u8,
                std::mem::size_of::<Vertex>(),
            ))
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct MeshBuilder {
    vertices: IndexSet<Vertex, ahash::RandomState>,
    indices: Vec<u32>,
}

impl MeshBuilder {
    pub fn new() -> MeshBuilder {
        MeshBuilder::default()
    }

    pub fn push(&mut self, v: Vertex) {
        let i = self.vertices.insert_full(v).0;
        self.indices.push(i as u32);
    }

    pub fn build(self) -> Mesh {
        let mut m = Mesh::new(PrimitiveTopology::TriangleList);

        let mut positions = Vec::with_capacity(self.vertices.len());
        let mut normals = Vec::with_capacity(self.vertices.len());
        let mut uvs = Vec::with_capacity(self.vertices.len());
        let mut colors = Vec::with_capacity(self.vertices.len());

        for Vertex {
            position,
            normal,
            uv,
            color,
        } in self.vertices
        {
            positions.push(position);
            normals.push(normal);
            uvs.push(uv);
            colors.push(color);
        }

        m.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
        m.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        m.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        m.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        m.set_indices(Some(Indices::U32(self.indices)));

        m
    }
}

// TODO: (improvement) ambient occlusion
pub fn rebuild_meshes(
    mut commands: Commands,
    blocks: Res<LoadedBlocks>,
    mut meshes: ResMut<Assets<Mesh>>,
    query: Query<(Entity, &ChunkInfo, &ChunkStore<MaterialID>), Without<ChunkMesh>>,
) {
    for (e, chunk_info, chunk_blocks) in query.iter() {
        if !chunk_blocks.is_empty() {
            let mesh = match chunk_info.mesher {
                Mesher::Greedy => greedy_mesh(chunk_blocks, blocks.as_ref()),
            };

            let mesh_handle = meshes.add(mesh);

            commands
                .entity(e)
                .insert(ChunkMesh { dirty: false })
                .insert(mesh_handle);
        }
    }
}
