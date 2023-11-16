use bevy::asset::load_internal_asset;
use bevy::pbr::{prelude::*, MaterialPipeline, MaterialPipelineKey};
use bevy::prelude::*;
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
};

// https://github.com/rust-adventure/bevy-examples/tree/main/examples/shader-test-001
// https://github.com/rust-adventure/bevy-examples/blob/main/libs/bevy_shader_utils/src/lib.rs

// pub const PERLIN_NOISE_2D: &str = include_str!("perlin_noise_2d.wgsl");
const PERLIN_NOISE_2D: Handle<Shader> = Handle::weak_from_u128(11918512342344596158);

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        // app.init_resource::<ShaderUtils>();
        load_internal_asset!(
            app,
            PERLIN_NOISE_2D,
            "perlin_noise_2d.wgsl",
            Shader::from_wgsl
        );
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GarageMaterialKey {
    depth_bias: i32,
    quality: i32,
}

#[derive(Asset, AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "E7F8955A-CF83-480D-A0C2-C2171898E571"]
#[bind_group_data(GarageMaterialKey)]
pub struct GroundMaterial {
    #[uniform(0)]
    pub color: Color,
    pub depth_bias: f32,
    #[uniform(0)]
    pub quality: i32,
}
impl Material for GroundMaterial {
    fn fragment_shader() -> ShaderRef {
        return "ground_material.wgsl".into();
    }
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(label) = &mut descriptor.label {
            *label = format!("pbr_{}", *label).into();
        }
        if let Some(depth_stencil) = descriptor.depth_stencil.as_mut() {
            depth_stencil.bias.constant = key.bind_group_data.depth_bias;
        }
        Ok(())
    }
}
impl From<&GroundMaterial> for GarageMaterialKey {
    fn from(material: &GroundMaterial) -> Self {
        GarageMaterialKey {
            depth_bias: material.depth_bias as i32,
            quality: material.quality,
        }
    }
}

#[derive(Asset, AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
#[uuid = "C147EDEE-9A8A-4B8F-B759-EDB527E56CC9"]
#[bind_group_data(GarageMaterialKey)]
pub struct AsphaltMaterial {
    #[uniform(0)]
    pub color: Color,
    pub depth_bias: f32,
    #[uniform(0)]
    pub quality: i32,
}
impl Material for AsphaltMaterial {
    fn fragment_shader() -> ShaderRef {
        return "asphalt_material.wgsl".into();
    }
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        if let Some(label) = &mut descriptor.label {
            *label = format!("pbr_{}", *label).into();
        }
        if let Some(depth_stencil) = descriptor.depth_stencil.as_mut() {
            depth_stencil.bias.constant = key.bind_group_data.depth_bias;
        }
        Ok(())
    }
}
impl From<&AsphaltMaterial> for GarageMaterialKey {
    fn from(material: &AsphaltMaterial) -> Self {
        GarageMaterialKey {
            depth_bias: material.depth_bias as i32,
            quality: material.quality,
        }
    }
}
