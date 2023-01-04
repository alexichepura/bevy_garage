use bevy::prelude::*;

#[derive(Resource)]
pub struct FontHandle {
    pub bold: Handle<Font>,
    pub medium: Handle<Font>,
}

impl FromWorld for FontHandle {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        Self {
            bold: asset_server.load("fonts/FiraSans-Bold.ttf"),
            medium: asset_server.load("fonts/FiraMono-Medium.ttf"),
        }
    }
}
