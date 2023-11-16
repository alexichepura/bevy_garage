use bevy::pbr::{prelude::*, MaterialExtension};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};

use crate::{ExtendedMaterialAsphalt, ExtendedMaterialGround};

// https://github.com/rust-adventure/bevy-examples/blob/main/libs/bevy_shader_utils/src/lib.rs

// const PERLIN_NOISE_2D: Handle<Shader> = Handle::weak_from_u128(11918512342344596158);

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<ExtendedMaterialGround>::default());
        app.add_plugins(MaterialPlugin::<ExtendedMaterialAsphalt>::default());
        // load_internal_asset!(
        //     app,
        //     PERLIN_NOISE_2D,
        //     "perlin_noise_2d.wgsl",
        //     Shader::from_wgsl
        // );
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct GroundExtension {
    #[uniform(100)]
    pub quality: i32,
}
impl MaterialExtension for GroundExtension {
    fn fragment_shader() -> ShaderRef {
        "ground_material_ext.wgsl".into()
    }
}

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
pub struct AsphaltExtension {
    #[uniform(100)]
    pub quality: i32,
}
impl MaterialExtension for AsphaltExtension {
    fn fragment_shader() -> ShaderRef {
        "asphalt_material_ext.wgsl".into()
    }
}
