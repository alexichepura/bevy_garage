use bevy::prelude::*;

use crate::shader::AsphaltMaterial;
use crate::shader::GroundMaterial;

#[derive(Resource)]
pub struct MaterialHandle {
    pub ground: Handle<GroundMaterial>,
    pub asphalt: Handle<AsphaltMaterial>,
}

impl FromWorld for MaterialHandle {
    fn from_world(world: &mut World) -> Self {
        let ground_color = Color::hex("7b824e").unwrap();
        let mut ground_materials = world.resource_mut::<Assets<GroundMaterial>>();
        let ground_handle = ground_materials.add(GroundMaterial {
            color: ground_color,
        });

        let asphalt_color = Color::hex("333355").unwrap();
        let mut asphalt_materials = world.resource_mut::<Assets<AsphaltMaterial>>();
        let asphalt_handle = asphalt_materials.add(AsphaltMaterial {
            color: asphalt_color,
        });

        Self {
            ground: ground_handle,
            asphalt: asphalt_handle,
        }
    }
}
