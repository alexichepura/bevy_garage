#import bevy_pbr::{
    mesh_view_bindings::view,
    pbr_fragment::pbr_input_from_standard_material,
    pbr_functions::alpha_discard,
}

#ifdef PREPASS_PIPELINE
#import bevy_pbr::{
    prepass_io::{VertexOutput, FragmentOutput},
    pbr_deferred_functions::deferred_output,
}
#else
#import bevy_pbr::{
    forward_io::{VertexOutput, FragmentOutput},
    pbr_functions::{apply_pbr_lighting, main_pass_post_lighting_processing},
}
#endif


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
    quality: i32 // 0-10
};
@group(1) @binding(100)
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
fn fragment(
    in: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    var pbr_input = pbr_input_from_standard_material(in, is_front);

    var x: f32 = in.world_position.x;
    var z: f32 = in.world_position.z;
    var bump_distance: f32 = distance(in.world_position.xyz, view.world_position.xyz);

    var output_color: vec4<f32>;
    var height: f32;
    if i32(bump_distance) < 20 * material.quality {
        height = bump(x, z, bump_distance);
        var h_color: f32 = 0.6 + height * 0.4;
        output_color = pbr_input.material.base_color * vec4<f32>(height, h_color, h_color, 1.);
    } else {
        output_color = pbr_input.material.base_color;
    }


    var out: FragmentOutput;
    out.color = apply_pbr_lighting(pbr_input);
    out.color = out.color;
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);
    return out;
}
