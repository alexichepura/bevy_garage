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

fn bump(uv_x: f32, uv_y: f32, bump_distance: f32) -> f32 {
    var coeff_l: f32 = 10.;
    var coeff_m: f32 = 100.;
    var coeff_s: f32 = 2000.;
    var noise: f32;
    var input_l: vec3<f32> = vec3<f32>(uv_x * coeff_l, uv_y * coeff_l, 1.);
    var noise_l = perlinNoise3(input_l);

    if bump_distance > 200. {
        noise = noise_l;
    } else {
        var input_m: vec3<f32> = vec3<f32>(uv_x * coeff_m, uv_y * coeff_m, 1.);
        var noise_m = perlinNoise3(input_m);
        if bump_distance > 30. {
            noise = noise_m * 0.05 + noise_l * 0.95;
        } else {
            var input_s: vec3<f32> = vec3<f32>(uv_x * coeff_s, uv_y * coeff_s, 1.);
            var noise_s = perlinNoise3(input_s);
            noise = noise_s * 0.02 + noise_m * 0.05 + noise_l * 0.93;
        }
    }
    return noise * 0.5;
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var bump_distance = distance(in.world_position.xyz, view.world_position.xyz);
    var height = 1. - bump(in.uv.x, in.uv.y, bump_distance);
    var output_color: vec4<f32> = material.color * vec4<f32>(height, height, height, 1.);
#ifdef VERTEX_COLORS
    output_color = output_color * in.color;
#endif
    var pbr_input: PbrInput;
    pbr_input.material.base_color = output_color;
    pbr_input.material.reflectance = 0.5;
    pbr_input.material.alpha_cutoff = 0.5;
    pbr_input.material.perceptual_roughness = 0.75;
    pbr_input.frag_coord = in.frag_coord;
    pbr_input.world_position = in.world_position;
    pbr_input.world_normal = in.world_normal;
    pbr_input.is_orthographic = view.projection[3].w == 1.0;
    var d: f32 = 1. / 1024.;
    var du: f32 = bump(in.uv.x - d, in.uv.y, bump_distance) - bump(in.uv.x + d, in.uv.y, bump_distance);
    var dv: f32 = bump(in.uv.x, in.uv.y - d, bump_distance) - bump(in.uv.x, in.uv.y + d, bump_distance);
    var Nt: vec3<f32> = vec3<f32>(du, dv, 0.2);
    Nt = normalize(Nt);
    var N: vec3<f32> = in.world_normal;
    var T: vec3<f32> = in.world_tangent.xyz;
    var B: vec3<f32> = in.world_tangent.w * cross(N, T);
    N = Nt.x * T + Nt.y * B + Nt.z * N;
    pbr_input.N = normalize(N);
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
