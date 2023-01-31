#import shaders::perlin_noise_3d

// https://gpuweb.github.io/gpuweb/wgsl/

struct GroundMaterial {
    color: vec4<f32>,
};

@group(1) @binding(0)
var<uniform> material: GroundMaterial;

@fragment
fn fragment(
    @location(0) something: vec4<f32>,
    @location(1) dunno: vec3<f32>,
    @location(2) uv: vec2<f32>
) -> @location(0) vec4<f32> {
    var input: vec3<f32> = vec3<f32>(uv.x * 1000., uv.y * 1000., 1.);
    var noise = perlinNoise3(input);
    var noise_01 = (noise + 1.0) / 2.0;
    return material.color * vec4<f32>(noise_01, noise_01, noise_01, 1.);
}
