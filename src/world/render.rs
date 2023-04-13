use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
};

#[derive(Debug, Clone, AsBindGroup, TypeUuid)]
#[uuid = "f690fdae-d598-45ab-8225-97e2a3f056e2"]
pub struct ChunkMaterial {}

impl Material for ChunkMaterial {
    fn vertex_shader() -> ShaderRef {
        "shaders/colored_test.wgsl".into()
    }

    fn fragment_shader() -> ShaderRef {
        "shaders/colored_test.wgsl".into()
    }

    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.vertex.buffers = vec![layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_COLOR.at_shader_location(1),
        ])?];
        Ok(())
    }
}
