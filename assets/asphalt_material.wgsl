// https://www.w3.org/TR/WGSL
// bevy/crates/bevy_pbr/src/render/pbr.wgsl
// https://gpuweb.github.io/gpuweb/wgsl/
// https://github.com/rust-adventure/bevy-examples/blob/main/libs/bevy_shader_utils/assets/shaders/custom_material.wgsl
// https://iquilezles.org/articles/voronoise/

#import bevy_pbr::pbr_functions as pbr_functions

#import bevy_pbr::mesh_vertex_output       MeshVertexOutput
#import bevy_pbr::mesh_bindings            mesh
#import bevy_pbr::mesh_view_bindings       view, fog, screen_space_ambient_occlusion_texture

#import bevy_pbr::mesh_view_types          FOG_MODE_OFF
#import bevy_core_pipeline::tonemapping    screen_space_dither, powsafe, tone_mapping
#import bevy_pbr::parallax_mapping         parallaxed_uv

#import bevy_pbr::prepass_utils

// TODO
// #import shaders::perlin_noise_2d
// MIT License. © Stefan Gustavson, Munrocket
fn permute4(x: vec4<f32>) -> vec4<f32> { return ((x * 34. + 1.) * x) % vec4<f32>(289.); }
fn fade2(t: vec2<f32>) -> vec2<f32> { return t * t * t * (t * (t * 6. - 15.) + 10.); }
fn perlinNoise2(P: vec2<f32>) -> f32 {
    var Pi: vec4<f32> = floor(P.xyxy) + vec4<f32>(0., 0., 1., 1.);
    let Pf = fract(P.xyxy) - vec4<f32>(0., 0., 1., 1.);
    Pi = Pi % vec4<f32>(289.); // To avoid truncation effects in permutation
    let ix = Pi.xzxz;
    let iy = Pi.yyww;
    let fx = Pf.xzxz;
    let fy = Pf.yyww;
    let i = permute4(permute4(ix) + iy);
    var gx: vec4<f32> = 2. * fract(i * 0.0243902439) - 1.; // 1/41 = 0.024...
    let gy = abs(gx) - 0.5;
    let tx = floor(gx + 0.5);
    gx = gx - tx;
    var g00: vec2<f32> = vec2<f32>(gx.x, gy.x);
    var g10: vec2<f32> = vec2<f32>(gx.y, gy.y);
    var g01: vec2<f32> = vec2<f32>(gx.z, gy.z);
    var g11: vec2<f32> = vec2<f32>(gx.w, gy.w);
    let norm = 1.79284291400159 - 0.85373472095314 * vec4<f32>(dot(g00, g00), dot(g01, g01), dot(g10, g10), dot(g11, g11));
    g00 = g00 * norm.x;
    g01 = g01 * norm.y;
    g10 = g10 * norm.z;
    g11 = g11 * norm.w;
    let n00 = dot(g00, vec2<f32>(fx.x, fy.x));
    let n10 = dot(g10, vec2<f32>(fx.y, fy.y));
    let n01 = dot(g01, vec2<f32>(fx.z, fy.z));
    let n11 = dot(g11, vec2<f32>(fx.w, fy.w));
    let fade_xy = fade2(Pf.xy);
    let n_x = mix(vec2<f32>(n00, n01), vec2<f32>(n10, n11), vec2<f32>(fade_xy.x));
    let n_xy = mix(n_x.x, n_x.y, fade_xy.y);
    return 2.3 * n_xy;
}

struct AsphaltMaterial {
    color: vec4<f32>,
    quality: i32 // 0-10
};

@group(1) @binding(0)
var<uniform> material: AsphaltMaterial;

const coeff_l: f32 = 0.035;
const coeff_m: f32 = 0.35;
const coeff_s: f32 = 0.5;
const coeff_xs: f32 = 120.;
const bump_d: f32 = 0.03125; // 1/32

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
        //     noise = noise_s * 0.1 + noise_m * 0.2 + noise_l * 0.7;
        //     // if bump_distance > 10. {
        //     // } else {
        //     //     var input_xs: vec2<f32> = xy * coeff_xs;
        //     //     var noise_xs: f32 = perlinNoise2(input_xs);
        //     //     noise = noise_xs * 0.01 + noise_s * 0.02 + noise_m * 0.17 + noise_l * 0.8;
        //     // }
        // }
    }
    return noise;
}

@fragment
fn fragment(in: MeshVertexOutput) -> @location(0) vec4<f32> {
    var x: f32 = in.world_position.x;
    var z: f32 = in.world_position.z;
    var bump_distance: f32 = distance(in.world_position.xyz, view.world_position.xyz);

    var output_color: vec4<f32>;
    var height: f32;
    if i32(bump_distance) < 20 * material.quality {
        height = bump(x, z, bump_distance);
        var h_color: f32 = 0.6 + height * 0.4;
        output_color = material.color * vec4<f32>(height, h_color, h_color, 1.);
    } else {
        output_color = material.color;
    }
#ifdef VERTEX_COLORS
    output_color = output_color * in.color;
#endif
    var pbr_input: pbr_functions::PbrInput = pbr_functions::pbr_input_new();
    pbr_input.material.base_color = output_color;
    pbr_input.material.reflectance = 0.5;
    pbr_input.material.alpha_cutoff = 0.5;
    pbr_input.material.perceptual_roughness = 0.7;
    pbr_input.frag_coord = in.position;
    pbr_input.world_position = in.world_position;
    pbr_input.world_normal = in.world_normal;
    pbr_input.is_orthographic = view.projection[3].w == 1.0;
    if i32(bump_distance) < 12 * material.quality {
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
    } else {
        pbr_input.N = in.world_normal;
    }
    pbr_input.V = pbr_functions::calculate_view(in.world_position, pbr_input.is_orthographic);
    pbr_input.flags = mesh.flags;
    output_color = pbr_functions::pbr(pbr_input);
    if (fog.mode != FOG_MODE_OFF) {
        output_color = pbr_functions::apply_fog(fog, output_color, in.world_position.xyz, view.world_position.xyz);
    }


#ifdef TONEMAP_IN_SHADER
    output_color = tone_mapping(output_color, view.color_grading);
#endif
#ifdef DEBAND_DITHER
    var output_rgb = output_color.rgb;
    output_rgb = powsafe(output_rgb, 1.0 / 2.2);
    output_rgb = output_rgb + screen_space_dither(in.position.xy);
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
