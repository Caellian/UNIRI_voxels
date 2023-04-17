#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::pbr_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::mesh_functions

#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::pbr_ambient
#import bevy_pbr::shadows
#import bevy_pbr::fog
#import bevy_pbr::pbr_functions

struct Vertex {
    @location(0) position: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) face_index: u32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) face_index: u32,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    out.face_index = vertex.face_index;
    out.normal = vertex.normal;
    out.uv = vertex.uv;

    return out;
}

struct FragmentInput {
    @location(0) face_index: u32,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
};

struct FaceData {
    base_color: vec4<f32>,

    base_texture: u32,
    base_texture_uv: vec2<f32>,

    emissive_color: vec4<f32>,

    roughness: f32,
    metallic: f32,
    reflectance: f32,
}

@group(1) @binding(0)
var<storage> face_data: array<FaceData>;

@fragment
fn fragment(input: FragmentInput) -> @location(0) vec4<f32> {
    return face_data[input.face_index].base_color;
}
