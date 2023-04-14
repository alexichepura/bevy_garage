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

#import shaders::perlin_noise_2d

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

const coeff_l: f32 = 0.03;
const coeff_m: f32 = 0.3;
const coeff_s: f32 = 1.;
const coeff_xs: f32 = 100.;
const bump_d: f32 = 0.125; // 1/8

fn bump(x: f32, y: f32, bump_distance: f32) -> f32 {
    // var rem_x: f32 = x % 1.;
    // var rem_y: f32 = y % 1.;
    // var height = inverseSqrt(rem_x * rem_x + rem_y * rem_y);
    // return height;

    var xy: vec2<f32> = vec2<f32>(x, y);
    var noise: f32;
    var input_l: vec2<f32> = xy * coeff_l;
    var noise_l: f32 = perlinNoise2(input_l);

    if bump_distance > 200. {
        noise = noise_l;
    } else {
        var input_m: vec2<f32> = xy * coeff_m;
        var noise_m: f32 = perlinNoise2(input_m);
        noise = noise_m * 0.2 + noise_l * 0.8;
        // if bump_distance > 100. {
        //     noise = noise_m * 0.2 + noise_l * 0.8;
        // } else {
        //     var input_s: vec2<f32> = xy * coeff_s;
        //     var noise_s: f32 = perlinNoise2(input_s);
        //     noise = noise_s * 0.05 + noise_m * 0.15 + noise_l * 0.8;
        //     // if bump_distance > 10. {
        //     // } else {
        //     //     var input_xs: vec2<f32> = xy * coeff_xs;
        //     //     var noise_xs: f32 = perlinNoise2(input_xs);
        //     //     noise = noise_xs * 0.01 + noise_s * 0.02 + noise_m * 0.07 + noise_l * 0.9;
        //     // }
        // }
    }
    return noise;
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4<f32> {
    var x: f32 = in.world_position.x;
    var z: f32 = in.world_position.z;
    var bump_distance: f32 = distance(in.world_position.xyz, view.world_position.xyz);
    var height: f32 = bump(x, z, bump_distance);
    var h_color: f32 = 0.8 + height * 0.2;
    var output_color: vec4<f32> = material.color * vec4<f32>(height, h_color, h_color, 1.);
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

    var du: f32 = bump(x - bump_d, z, bump_distance) - height;
    var dv: f32 = bump(x, z - bump_d, bump_distance) - height;
    var Nt: vec3<f32> = vec3<f32>(du, dv, 0.1);
    Nt = normalize(Nt);
    Nt.y = -Nt.y;
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
