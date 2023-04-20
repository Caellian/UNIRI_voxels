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
    @location(0) world_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) face_index: u32,
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

    out.clip_position = mesh_position_local_to_clip(mesh.model, vec4<f32>(vertex.position, 1.0));
    out.face_index = vertex.face_index;
    out.normal = vertex.normal;
    out.uv = vertex.uv;
    out.world_pos = vertex.position;

    return out;
}

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    @location(0) world_pos: vec3<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) uv: vec2<f32>,
    @location(3) face_index: u32,
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
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var fd: FaceData = face_data[in.face_index];
    var pbr_input: PbrInput;

    pbr_input.material.base_color = fd.base_color;
    pbr_input.material.reflectance = fd.reflectance;
    //pbr_input.material.flags = fd.flags;
    //pbr_input.material.alpha_cutoff = fd.alpha_cutoff;

    // TODO use .a for exposure compensation in HDR
    pbr_input.material.emissive = fd.emissive_color;
    /*
    if (STANDARD_MATERIAL_FLAGS_EMISSIVE_TEXTURE_BIT != 0u) {
        pbr_input.material.emissive = vec4<f32>(emissive.rgb * textureSample(p_emissive_texture, p_emissive_sampler, uv).rgb, 1.0);
    }
    */

    var metallic: f32 = fd.metallic;
    var perceptual_roughness: f32 = fd.roughness;
    /*
    if (STANDARD_MATERIAL_FLAGS_METALLIC_ROUGHNESS_TEXTURE_BIT != 0u) {
        let metallic_roughness = textureSample(p_metallic_roughness_texture, p_metallic_roughness_sampler, uv);
        // Sampling from GLTF standard channels for now
        metallic = metallic * metallic_roughness.b;
        perceptual_roughness = perceptual_roughness * metallic_roughness.g;
    }
    */
    pbr_input.material.metallic = metallic;
    pbr_input.material.perceptual_roughness = perceptual_roughness;

    var occlusion: f32 = 1.0;
    /*
    if (STANDARD_MATERIAL_FLAGS_OCCLUSION_TEXTURE_BIT != 0u) {
        occlusion = textureSample(p_occlusion_texture, p_occlusion_sampler, uv).r;
    }
    */
    pbr_input.occlusion = occlusion;

    pbr_input.frag_coord = in.frag_coord;
    pbr_input.world_position = vec4<f32>(in.world_pos, 1.0);
    pbr_input.world_normal = in.normal;

    pbr_input.is_orthographic = view.projection[3].w == 1.0;

    var normal: vec3<f32> = in.normal;
    if (!in.is_front) {
        normal = -normal;
    }

    pbr_input.N = in.normal;
    // true - is_ortho
    pbr_input.V = calculate_view(vec4<f32>(in.world_pos, 1.0), false);

    return pbr(pbr_input);
    /*
    #ifdef TONEMAP_IN_SHADER
        output_color = tone_mapping(output_color);
    #endif
    */
}
