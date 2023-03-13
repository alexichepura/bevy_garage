// bevy/crates/bevy_pbr/src/render/pbr.wgsl
// https://gpuweb.github.io/gpuweb/wgsl/
// https://github.com/rust-adventure/bevy-examples/blob/main/libs/bevy_shader_utils/assets/shaders/custom_material.wgsl
// https://iquilezles.org/articles/voronoise/

#import bevy_pbr::mesh_view_bindings
#import bevy_pbr::mesh_bindings
#import bevy_pbr::pbr_types
#import bevy_pbr::utils
#import bevy_pbr::clustered_forward
#import bevy_pbr::lighting
#import bevy_pbr::shadows
#import bevy_pbr::fog
#import bevy_pbr::pbr_functions
#import bevy_pbr::pbr_ambient
#import bevy_pbr::mesh_functions

#import shaders::perlin_noise_3d

struct GroundMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: GroundMaterial;

struct FragmentInput {
    @builtin(front_facing) is_front: bool,
    @builtin(position) frag_coord: vec4<f32>,
    #import bevy_pbr::mesh_vertex_output
};


@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var input_s: vec3<f32> = vec3<f32>(in.uv.x * 2000., in.uv.y * 2000., 1.);
    var noise_s = perlinNoise3(input_s);
    var input_m: vec3<f32> = vec3<f32>(in.uv.x * 100., in.uv.y * 100., 1.);
    var noise_m = perlinNoise3(input_m);
    var input_l: vec3<f32> = vec3<f32>(in.uv.x * 10., in.uv.y * 10., 1.);
    var noise_l = perlinNoise3(input_l);

    var noise_01 = (noise_s + noise_m + noise_l + 1.0) / 2.0;

    var output_color: vec4<f32> = material.color * vec4<f32>(noise_01, noise_01, noise_01, 1.);
#ifdef VERTEX_COLORS
    output_color = output_color * in.color;
#endif
    var pbr_input: PbrInput;
    pbr_input.material.base_color = output_color;
    pbr_input.material.reflectance = 0.5;
    pbr_input.material.alpha_cutoff = 0.5;
    pbr_input.frag_coord = in.frag_coord;
    pbr_input.world_position = in.world_position;
    pbr_input.world_normal = in.world_normal;
    pbr_input.is_orthographic = view.projection[3].w == 1.0;
    pbr_input.N = apply_normal_mapping(
        pbr_input.material.flags,
        pbr_input.world_normal,
#ifdef VERTEX_TANGENTS
#ifdef STANDARDMATERIAL_NORMAL_MAP
        in.world_tangent,
#endif
#endif
#ifdef VERTEX_UVS
        in.uv,
#endif
    );
    pbr_input.V = calculate_view(in.world_position, pbr_input.is_orthographic);
    pbr_input.flags = mesh.flags;
    output_color = pbr(pbr_input);


    output_color = apply_fog(output_color, in.world_position.xyz, view.world_position.xyz);


#ifdef TONEMAP_IN_SHADER
    output_color = tone_mapping(output_color);
#endif
#ifdef DEBAND_DITHER
    var output_rgb = output_color.rgb;
    output_rgb = powsafe(output_rgb, 1.0 / 2.2);
    output_rgb = output_rgb + screen_space_dither(in.frag_coord.xy);
    // This conversion back to linear space is required because our output texture format is
    // SRGB; the GPU will assume our output is linear and will apply an SRGB conversion.
    output_rgb = powsafe(output_rgb, 2.2);
    output_color = vec4(output_rgb, output_color.a);
#endif
#ifdef PREMULTIPLY_ALPHA
    output_color = premultiply_alpha(material.flags, output_color);
#endif
    return output_color;
}
