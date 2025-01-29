use std::future::Future;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::Index;
use std::sync::Arc;

use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use bevy::render::mesh::*;
use indexmap::IndexSet;
use maybe_owned::MaybeOwned;
use ndarray::{Array, Slice};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

use rayon::prelude::*;

use crate::data::{FaceProperties, LoadedMaterials};
use crate::data::MaterialProperties;
use crate::MaterialID;
use crate::math::side::Side;
use crate::world::chunk::ChunkStore;

use super::{
    chunk_material::ChunkMaterial, SIDE_VIEW_TRANSFORMS, SideView, SizedGrid, SliceView,
    wrapped_rows,
};

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

static FLIP_OPPOSITE_TRANSFORMS: Lazy<[(Mat3, Vec3); 6]> = Lazy::new(|| {
    Side::ALL
        .map(|it| {
            SIDE_VIEW_TRANSFORMS[it.opposite() as usize].0
                * SIDE_VIEW_TRANSFORMS[it as usize].0.inverse()
        })
        .map(wrapped_rows)
});
const SIDE_SIZE: f32 = 1.0;

fn face_from_side_pos(side: Side, position: UVec3, size: IVec2) -> [Vec3; 4] {
    let normal = side.normal();
    let position = position.as_vec3() + normal * (SIDE_SIZE / 2.);

    // FIXME: Test opposite case at the opposite ends - 80% says it's wrong
    [
        position,
        position + (size.as_vec2() * Vec2::new(1., 0.)).extend(0.) * SIDE_SIZE,
        position + (size.as_vec2() * Vec2::new(1., 1.)).extend(0.) * SIDE_SIZE,
        position + (size.as_vec2() * Vec2::new(0., 1.)).extend(0.) * SIDE_SIZE,
    ]
}

fn opposite_face_from_side_pos(
    side: Side,
    position: UVec3,
    chunk_size: UVec3,
    size: IVec2,
) -> [Vec3; 4] {
    // FIXME: Normalize to lower octant coordinates.
    let (t_mul, t_add) = FLIP_OPPOSITE_TRANSFORMS[side as usize];
    let position = t_mul * position.as_vec3() + t_add * (chunk_size - UVec3::ONE).as_vec3();
    let size = (t_add.as_ivec3() * -1).xy() * size; // FIXME: blind error fix
    return face_from_side_pos(side, position.as_uvec3(), size);
}

#[derive(Debug, Clone)]
pub struct MeshFace {
    corners: [Vec3; 4],
}

impl MeshFace {
    pub fn new(side: Side, position: UVec3, size: IVec2) -> Self {
        MeshFace {
            corners: face_from_side_pos(side, position, size),
        }
    }

    pub fn new_opposite(side: Side, position: UVec3, chunk_size: UVec3, size: IVec2) -> Self {
        MeshFace {
            corners: opposite_face_from_side_pos(side, position, chunk_size, size),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FaceInfo<'a> {
    face: MeshFace,
    material: &'a MaterialProperties,
}

pub struct MeshingContext<'a, T = MaterialID>
where
    T: PartialEq,
{
    sides: [Option<&'a ChunkStore<T>>; Side::COUNT],
}

type ContextWindow<'a, T> = Option<SliceView<'a, T, SideView<'a, T, ChunkStore<T>>>>;

impl<'a, T> MeshingContext<'a, T>
where
    T: PartialEq,
{
    pub fn has(&self, side: Side) -> bool {
        self.sides[side].is_some()
    }

    pub fn get(&self, side: Side) -> ContextWindow<'a, T> {
        let v = self.sides[side]?;
        Some(SliceView::new(SideView::new(v, side.opposite()), 0))
    }

    pub fn get_entry(&self, side: Side, pos: UVec2) -> Option<&'a T> {
        self.get(side).and_then(|it| it.get_pos_value(pos))
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct MeshingState {
    /// Chunk sides that were provided context and have been meshed.
    meshed_sides: [bool; Side::COUNT],
    /// Whether the insides of the chunk has been meshed.
    meshed_internals: bool,
    /// List of locations that have changed since the cached mesh has been generated.
    invalidated: Vec<UVec3>,
}

impl MeshingState {
    pub fn mark_dirty(&mut self, location: UVec3) {
        todo!()
    }
}

// When first loaded you need to mesh inner as well as 1/2 margins
// Then update state
// When signalled that surroundings changed, state needs to me read and updated after mesing newly available margins
// When signalled internals changed, ...
//  - Don't update for individual changes, use invalidated instead. This complicated, do it last if even
//

pub async fn mesh_inner_slice<'a, 'b, G: SizedGrid<'a, MaterialID>>(
    blocks: &'b G,
    loaded: &'a LoadedMaterials,
    depth: u32,
    top: &'b Option<SliceView<'a, MaterialID, G>>,
    bottom: &'b Option<SliceView<'a, MaterialID, G>>,
) -> Vec<FaceInfo<'a>>
where
    'a: 'b,
    MaybeOwned<'b, G>: From<&'b G>,
{
    let mut result = Vec::with_capacity(4);

    let mut ids_before = SliceView::try_new(blocks, depth as i32 - 1).or_else(|| *top);
    let mut ids_at = SliceView::new(blocks, depth);
    let mut ids_after = SliceView::try_new(blocks, depth as i32 + 1).or_else(|| *bottom);

    let mut front_faces = Vec::new();
    let mut back_faces = Vec::new();

    for x in 0..blocks.size().x as usize {
        let mut y = 0;
        while y < blocks.size().y as usize {
            let pos = UVec2::new(x as u32, y as u32);
            let current = match ids_at.get_pos_value(pos) {
                Some(it) => it,
                None => {
                    consumed[(x, y, 0)] = depth;
                    consumed[(x, y, 1)] = depth;
                    continue;
                }
            };
            let material = {
                #[cfg(debug_assertions)]
                {
                    loaded
                        .properties
                        .get(current)
                        .expect(format!("material registry missing id: {}", current).as_str())
                }
                #[cfg(not(debug_assertions))]
                unsafe {
                    loaded.properties.get(current).unwrap_unchecked()
                }
            };

            let check_side = |side: Option<SliceView<'_, MaterialID, _>>| {
                let mut h = 1;
                let mut w = 1;

                let mut pos = |h, w| UVec2::new(x as u32 + h, y as u32 + w);

                while ids_at.get_pos_value(pos(h, 0)) == Some(current) {
                    let above = side.and_then(|it| it.get_pos_value(pos(h, 0)));
                    if !is_block_face_visible(above, current, material, loaded) {
                        break;
                    }
                    h += 1;
                }
                'outer: loop {
                    for ih in 0..h {
                        if ids_at.get_pos_value(pos(ih, w)) != Some(current) {
                            break 'outer;
                        }
                        let above = side.and_then(|it| it.get_pos_value(pos(ih, w)));

                        if !is_block_face_visible(above, current, material, loaded) {
                            break 'outer;
                        }
                    }
                    w += 1;
                }

                return IVec2::new(w as i32, h as i32);
            };

            let mut min_y = 1;

            // top face
            if consumed[(x, y, 0)] != depth {
                let above = ids_before.and_then(|it| it.get_pos_value(pos));

                if is_block_face_visible(above, current, material, loaded) {
                    let size = check_side(ids_before);

                    front_faces.push(FaceInfo::new(
                        side,
                        UVec3::new(x as u32, y as u32, depth as u32),
                        size,
                        material,
                    ));

                    min_y = std::cmp::min(min_y, size.y);
                }
            }

            // bottom face
            if consumed[(x, y, 1)] != depth {
                let above = ids_after.and_then(|it| it.get_pos_value(pos));

                if is_block_face_visible(above, current, material, loaded) {
                    let size = check_side(ids_after);

                    back_faces.push(FaceInfo::new_opposite(
                        side,
                        UVec3::new(x as u32, y as u32, depth as u32),
                        blocks.size(),
                        size,
                        material,
                    ));

                    min_y = std::cmp::min(min_y, size.y);
                }
            }

            y += min_y as usize;
        }
    }

    [front_faces, back_faces]
}

pub fn mesh_side<'a, G: SizedGrid<'a, MaterialID>>(
    blocks: &G,
    loaded: &'a LoadedMaterials,
    side: Side,
) -> Vec<FaceInfo<'a>> {
    let blocks = SideView::new(blocks, side);

    let mut consumed =
        Array::from_elem((blocks.size().x as usize, blocks.size().y as usize, 2), -1);

    (0..blocks.size().z as i32)
        .into_par_iter()
        .map(|depth| {
            let mut ids_before = SliceView::try_new(&blocks, depth - 1);
            let mut ids_at = SliceView::new(&blocks, depth as u32);
            let mut ids_after = SliceView::try_new(&blocks, depth + 1);

            let mut front_faces = Vec::new();
            let mut back_faces = Vec::new();

            for x in 0..blocks.size().x as usize {
                let mut y = 0;
                while y < blocks.size().y as usize {
                    let pos = UVec2::new(x as u32, y as u32);
                    let current = match ids_at.get_pos_value(pos) {
                        Some(it) => it,
                        None => {
                            consumed[(x, y, 0)] = depth;
                            consumed[(x, y, 1)] = depth;
                            continue;
                        }
                    };
                    let material = {
                        #[cfg(debug_assertions)]
                        {
                            loaded.properties.get(current).expect(
                                format!("material registry missing id: {}", current).as_str(),
                            )
                        }
                        #[cfg(not(debug_assertions))]
                        {
                            loaded.properties.get(current).unwrap_unchecked()
                        }
                    };

                    let check_side = |side: Option<SliceView<'_, MaterialID, _>>| {
                        let mut h = 1;
                        let mut w = 1;

                        let mut pos = |h, w| UVec2::new(x as u32 + h, y as u32 + w);

                        while ids_at.get_pos_value(pos(h, 0)) == Some(current) {
                            let above = side.and_then(|it| it.get_pos_value(pos(h, 0)));
                            if !is_block_face_visible(above, current, material, loaded) {
                                break;
                            }
                            h += 1;
                        }
                        'outer: loop {
                            for ih in 0..h {
                                if ids_at.get_pos_value(pos(ih, w)) != Some(current) {
                                    break 'outer;
                                }
                                let above = side.and_then(|it| it.get_pos_value(pos(ih, w)));

                                if !is_block_face_visible(above, current, material, loaded) {
                                    break 'outer;
                                }
                            }
                            w += 1;
                        }

                        return IVec2::new(w as i32, h as i32);
                    };

                    let mut min_y = 1;

                    // top face
                    if consumed[(x, y, 0)] != depth {
                        let above = ids_before.and_then(|it| it.get_pos_value(pos));

                        if is_block_face_visible(above, current, material, loaded) {
                            let size = check_side(ids_before);

                            front_faces.push(FaceInfo::new(
                                side,
                                UVec3::new(x as u32, y as u32, depth as u32),
                                size,
                                material,
                            ));

                            min_y = std::cmp::min(min_y, size.y);
                        }
                    }

                    // bottom face
                    if consumed[(x, y, 1)] != depth {
                        let above = ids_after.and_then(|it| it.get_pos_value(pos));

                        if is_block_face_visible(above, current, material, loaded) {
                            let size = check_side(ids_after);

                            back_faces.push(FaceInfo::new_opposite(
                                side,
                                UVec3::new(x as u32, y as u32, depth as u32),
                                blocks.size(),
                                size,
                                material,
                            ));

                            min_y = std::cmp::min(min_y, size.y);
                        }
                    }

                    y += min_y as usize;
                }
            }

            [front_faces, back_faces]
        })
        .fold(
            || [Vec::new(), Vec::new()],
            |mut acc, it| {
                let [mut a, mut b] = it;
                acc[0].append(&mut a);
                acc[1].append(&mut b);
                acc
            },
        )
}

pub fn greedy_mesh<'a, G: SizedGrid<MaterialID>>(
    blocks: &G,
    loaded: &'a LoadedMaterials,
) -> [Vec<FaceInfo<'a>>; Side::COUNT] {
    [Side::East, Side::Top, Side::South]
        .into_par_iter()
        .flat_map(|side| {
            let blocks = SideView::new(blocks, side);

            let mut consumed =
                Array::from_elem((blocks.size().x as usize, blocks.size().y as usize, 2), -1);

            (0..blocks.size().z as i32)
                .into_par_iter()
                .map(|depth| {
                    let mut ids_before = SliceView::try_new(&blocks, depth - 1);
                    let mut ids_at = SliceView::new(&blocks, depth as u32);
                    let mut ids_after = SliceView::try_new(&blocks, depth + 1);

                    let mut front_faces = Vec::new();
                    let mut back_faces = Vec::new();

                    for x in 0..blocks.size().x as usize {
                        let mut y = 0;
                        while y < blocks.size().y as usize {
                            let pos = UVec2::new(x as u32, y as u32);
                            let current = match ids_at.get_pos_value(pos) {
                                Some(it) => it,
                                None => {
                                    consumed[(x, y, 0)] = depth;
                                    consumed[(x, y, 1)] = depth;
                                    continue;
                                }
                            };
                            let material = {
                                #[cfg(debug_assertions)]
                                {
                                    loaded.properties.get(current).expect(
                                        format!("material registry missing id: {}", current)
                                            .as_str(),
                                    )
                                }
                                #[cfg(not(debug_assertions))]
                                {
                                    loaded.properties.get(current).unwrap_unchecked()
                                }
                            };

                            let check_side = |side: Option<SliceView<'_, MaterialID, _>>| {
                                let mut h = 1;
                                let mut w = 1;

                                let mut pos = |h, w| UVec2::new(x as u32 + h, y as u32 + w);

                                while ids_at.get_pos_value(pos(h, 0)) == Some(current) {
                                    let above = side.and_then(|it| it.get_pos_value(pos(h, 0)));
                                    if !is_block_face_visible(above, current, material, loaded) {
                                        break;
                                    }
                                    h += 1;
                                }
                                'outer: loop {
                                    for ih in 0..h {
                                        if ids_at.get_pos_value(pos(ih, w)) != Some(current) {
                                            break 'outer;
                                        }
                                        let above =
                                            side.and_then(|it| it.get_pos_value(pos(ih, w)));

                                        if !is_block_face_visible(above, current, material, loaded)
                                        {
                                            break 'outer;
                                        }
                                    }
                                    w += 1;
                                }

                                return IVec2::new(w as i32, h as i32);
                            };

                            let mut min_y = 1;

                            // top face
                            if consumed[(x, y, 0)] != depth {
                                let above = ids_before.and_then(|it| it.get_pos_value(pos));

                                if is_block_face_visible(above, current, material, loaded) {
                                    let size = check_side(ids_before);

                                    front_faces.push(FaceInfo::new(
                                        side,
                                        UVec3::new(x as u32, y as u32, depth as u32),
                                        size,
                                        material,
                                    ));

                                    min_y = std::cmp::min(min_y, size.y);
                                }
                            }

                            // bottom face
                            if consumed[(x, y, 1)] != depth {
                                let above = ids_after.and_then(|it| it.get_pos_value(pos));

                                if is_block_face_visible(above, current, material, loaded) {
                                    let size = check_side(ids_after);

                                    back_faces.push(FaceInfo::new_opposite(
                                        side,
                                        UVec3::new(x as u32, y as u32, depth as u32),
                                        blocks.size(),
                                        size,
                                        material,
                                    ));

                                    min_y = std::cmp::min(min_y, size.y);
                                }
                            }

                            y += min_y as usize;
                        }
                    }

                    [front_faces, back_faces]
                })
                .fold(
                    || [Vec::new(), Vec::new()],
                    |mut acc, it| {
                        let [mut a, mut b] = it;
                        acc[0].append(&mut a);
                        acc[1].append(&mut b);
                        acc
                    },
                )
        })
        .collect()
}

#[inline(always)]
fn is_block_face_visible(
    above: Option<&MaterialID>,
    current: &MaterialID,
    current_mat: &MaterialProperties,
    loaded: &LoadedMaterials,
) -> bool {
    let above_mat = above.and_then(|it| loaded.properties.get(it));

    if let Some(above_mat) = above_mat {
        if above_mat.color.w == 1.0 {
            return false;
        }
        if above == Some(current) {
            return false;
        }
    }

    current_mat.color.w != 0.0
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
    uv: Vec2::new(0., 0.),
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
