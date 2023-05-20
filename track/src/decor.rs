use crate::TrackConfig;
use bevy::prelude::*;
use std::f32::consts::PI;

pub fn track_decorations_start_system(
    asset_server: Res<AssetServer>,
    mut cmd: Commands,
    track_config: Res<TrackConfig>,
) {
    let gl_object = asset_server.load("overheadLights.glb#Scene0");
    let (translate, quat) = track_config.get_transform_by_meter(0.);
    let translate = translate + quat.mul_vec3(Vec3::new(2.25, 0., 0.)) - Vec3::new(0., 0.4, 0.);
    let quat = quat.mul_quat(Quat::from_rotation_y(PI));
    cmd.spawn(SceneBundle {
        scene: gl_object,
        transform: Transform::from_scale(Vec3::ONE * 15.)
            .with_translation(translate)
            .with_rotation(quat),
        ..default()
    });
}
