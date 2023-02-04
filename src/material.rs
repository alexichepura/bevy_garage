use bevy::prelude::*;

use crate::shader::GroundMaterial;

#[derive(Resource)]
pub struct MaterialHandle {
    pub ground: Handle<GroundMaterial>,
}

impl FromWorld for MaterialHandle {
    fn from_world(world: &mut World) -> Self {
        let color = Color::hex("7b824e").unwrap();

        let mut custom_materials = world.resource_mut::<Assets<GroundMaterial>>();
        let handle = custom_materials.add(GroundMaterial { color });
        Self { ground: handle }
    }
}
