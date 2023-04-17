use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::mesh::{MeshVertexAttribute, MeshVertexBufferLayout};
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
};
use wgpu::VertexFormat;

use crate::data::FaceProperties;

/// The shader handle for `"parallax_map.wgsl"`.
#[allow(clippy::unreadable_literal)]
pub const CHUNK_SHADER_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Shader::TYPE_UUID, 9592100656503623734);

#[derive(Debug, Clone, AsBindGroup, TypeUuid)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e2"]
pub struct ChunkMaterial {
    pub alpha_mode: AlphaMode,
    pub depth_bias: f32,

    #[storage(0, read_only)]
    pub face_properties: Vec<FaceProperties>,
}

impl Default for ChunkMaterial {
    fn default() -> Self {
        ChunkMaterial {
            alpha_mode: AlphaMode::Opaque,
            depth_bias: 0.0,
            face_properties: Vec::new(),
        }
    }
}

impl ChunkMaterial {
    pub const ATTRIBUTE_FACE_INDEX: MeshVertexAttribute =
        MeshVertexAttribute::new("Voxel_Index", 2349710119055201991, VertexFormat::Uint32);
}

impl Material for ChunkMaterial {
    #[cfg(not(feature = "debug"))]
    fn vertex_shader() -> ShaderRef {
        CHUNK_SHADER_HANDLE.typed::<Shader>().into()
    }
    #[cfg(feature = "debug")]
    fn vertex_shader() -> ShaderRef {
        "world/chunk/chunk_shader.wgsl".into()
    }

    #[cfg(not(feature = "debug"))]
    fn fragment_shader() -> ShaderRef {
        CHUNK_SHADER_HANDLE.typed::<Shader>().into()
    }
    #[cfg(feature = "debug")]
    fn fragment_shader() -> ShaderRef {
        "world/chunk/chunk_shader.wgsl".into()
    }

    fn alpha_mode(&self) -> AlphaMode {
        self.alpha_mode
    }

    fn depth_bias(&self) -> f32 {
        self.depth_bias
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.buffers = vec![layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
            Self::ATTRIBUTE_FACE_INDEX.at_shader_location(3),
        ])?];
        Ok(())
    }
}
