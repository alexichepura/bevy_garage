use bevy::reflect::TypeUuid;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::{asset::HandleId, prelude::*};

// inspiration https://github.com/rust-adventure/bevy-examples/tree/main/examples/shader-test-001

pub const PERLIN_NOISE_3D: &str = include_str!("shader/perlin_noise_3d.wgsl");

pub struct ShadersPlugin;

impl Plugin for ShadersPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ShaderUtils>();
    }
}

#[derive(Resource)]
struct ShaderUtils {
    perlin_noise_3d: HandleId,
}

impl FromWorld for ShaderUtils {
    fn from_world(world: &mut World) -> Self {
        let mut shaders = world.get_resource_mut::<Assets<Shader>>().unwrap();

        ShaderUtils {
            perlin_noise_3d: load_shader(&mut shaders, "perlin_noise_3d", PERLIN_NOISE_3D),
        }
    }
}

fn load_shader(
    shaders: &mut Mut<Assets<Shader>>,
    name: &str,
    shader_str: &'static str,
) -> HandleId {
    let mut shader = Shader::from_wgsl(shader_str);
    shader.set_import_path(format!("shaders::{}", name));
    let id = HandleId::random::<Shader>();
    shaders.set_untracked(id, shader);
    id
}

#[derive(AsBindGroup, TypeUuid, Debug, Clone)]
#[uuid = "E7F8955A-CF83-480D-A0C2-C2171898E571"]
pub struct GroundMaterial {
    #[uniform(0)]
    pub color: Color,
}
impl Material for GroundMaterial {
    fn fragment_shader() -> ShaderRef {
        "ground_material.wgsl".into()
    }
}
