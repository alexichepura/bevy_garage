use bevy::pbr::{prelude::*, MaterialPipeline, MaterialPipelineKey};
use bevy::reflect::{TypePath, TypeUuid};
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{
    AsBindGroup, RenderPipelineDescriptor, ShaderRef, SpecializedMeshPipelineError,
};
use bevy::{asset::HandleId, prelude::*};

// https://github.com/rust-adventure/bevy-examples/tree/main/examples/shader-test-001

pub const PERLIN_NOISE_2D: &str = include_str!("perlin_noise_2d.wgsl");

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShaderUtils>();
    }
}

#[allow(dead_code)]
#[derive(Resource)]
struct ShaderUtils {
    perlin_noise_2d: HandleId,
}

impl FromWorld for ShaderUtils {
    fn from_world(world: &mut World) -> Self {
        let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();

        ShaderUtils {
            perlin_noise_2d: load_shader(&mut shaders, "perlin_noise_2d", PERLIN_NOISE_2D),
        }
    }
}

fn load_shader(
    shaders: &mut Mut<Assets<Shader>>,
    name: &str,
    shader_str: &'static str,
) -> HandleId {
    let shader = Shader::from_wgsl(shader_str, format!("shaders::{}", name));
    let id = HandleId::random::<Shader>();
    shaders.set_untracked(id, shader);
    id
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct GarageMaterialKey {
    depth_bias: i32,
    quality: i32,
}

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
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

#[derive(AsBindGroup, TypeUuid, TypePath, Debug, Clone)]
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
