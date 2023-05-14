// use crate::config::Config;
use bevy::prelude::*;
// use std::f32::consts::PI;

pub fn track_decorations_start_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    // config: Res<Config>,
) {
    let gl_object = asset_server.load("overheadLights.glb#Scene0");
    commands.spawn(SceneBundle {
        scene: gl_object,
        transform: Transform::from_scale(Vec3::ONE * 15.)
            .with_translation(Vec3::new(1.65, 0., 1.65)),
        // .with_rotation(config.quat.mul_quat(Quat::from_rotation_y(PI))),
        ..default()
    });
}
