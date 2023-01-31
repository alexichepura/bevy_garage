// https://gpuweb.github.io/gpuweb/wgsl/
// https://github.com/rust-adventure/bevy-examples/blob/main/libs/bevy_shader_utils/assets/shaders/custom_material.wgsl
// https://iquilezles.org/articles/voronoise/
// bevy_pbr::mesh
#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings

#import bevy_pbr::pbr_types
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
// #import bevy_pbr::fog
#import bevy_pbr::pbr_functions
#import bevy_pbr::mesh_functions

#import shaders::perlin_noise_3d

struct GroundMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: GroundMaterial;


struct Vertex {
#ifdef VERTEX_POSITIONS
    @location(0) position: vec3<f32>,
#endif
#ifdef VERTEX_NORMALS
    @location(1) normal: vec3<f32>,
#endif
#ifdef VERTEX_UVS
    @location(2) uv: vec2<f32>,
#endif
#ifdef VERTEX_TANGENTS
    @location(3) tangent: vec4<f32>,
#endif
#ifdef VERTEX_COLORS
    @location(4) color: vec4<f32>,
#endif
#ifdef SKINNED
    @location(5) joint_indices: vec4<u32>,
    @location(6) joint_weights: vec4<f32>,
#endif
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    var out: VertexOutput;

#ifdef SKINNED
    var model = skin_model(vertex.joint_indices, vertex.joint_weights);
#else
    var model = mesh.model;
#endif

#ifdef VERTEX_NORMALS
#ifdef SKINNED
    out.world_normal = skin_normals(model, vertex.normal);
#else
    out.world_normal = mesh_normal_local_to_world(vertex.normal);
#endif
#endif

#ifdef VERTEX_POSITIONS
    out.world_position = mesh_position_local_to_world(model, vec4<f32>(vertex.position, 1.0));
    out.clip_position = mesh_position_world_to_clip(out.world_position);
#endif

#ifdef VERTEX_UVS
    out.uv = vertex.uv;
#endif

#ifdef VERTEX_TANGENTS
    out.world_tangent = mesh_tangent_local_to_world(model, vertex.tangent);
#endif

#ifdef VERTEX_COLORS
    out.color = vertex.color;
#endif

    return out;
}

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};


@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    let layer = i32(in.world_position.x) & 0x3;
    var pbr_input: PbrInput = pbr_input_new();
    
    // pbr_input.material.base_color = textureSample(my_array_texture, my_array_texture_sampler, in.uv, layer);
    var input: vec3<f32> = vec3<f32>(in.uv.x * 1000., in.uv.y * 1000., 1.);
    var noise = perlinNoise3(input);
    var noise_01 = (noise + 1.0) / 2.0;
    pbr_input.material.base_color = material.color * vec4<f32>(noise_01, noise_01, noise_01, 1.);
#ifdef VERTEX_COLORS
    pbr_input.material.base_color = pbr_input.material.base_color * in.color;
#endif

    pbr_input.frag_coord = in.frag_coord;
    pbr_input.world_position = in.world_position;
    pbr_input.world_normal = prepare_world_normal(
        in.world_normal,
        (pbr_input.material.flags & STANDARD_MATERIAL_FLAGS_DOUBLE_SIDED_BIT) != 0u,
        in.is_front,
    );

    pbr_input.is_orthographic = view.projection[3].w == 1.0;

    pbr_input.N = apply_normal_mapping(
        pbr_input.material.flags,
        pbr_input.world_normal,
#ifdef VERTEX_TANGENTS
#ifdef STANDARDMATERIAL_NORMAL_MAP
        in.world_tangent,
#endif
#endif
        in.uv,
    );
    pbr_input.V = calculate_view(in.world_position, pbr_input.is_orthographic);

    return tone_mapping(pbr(pbr_input));
}

// @fragment
// fn fragment(
//     in: FragmentInput,
// ) -> @location(0) vec4<f32> {
//     var input: vec3<f32> = vec3<f32>(in.uv.x * 1000., in.uv.y * 1000., 1.);
//     var noise = perlinNoise3(input);
//     var noise_01 = (noise + 1.0) / 2.0;
//     return material.color * vec4<f32>(noise_01, noise_01, noise_01, 1.);
// }
