use bevy::prelude::*;

pub fn light_start_system(mut commands: Commands) {
    // commands.spawn_bundle(PointLightBundle {
    //     transform: Transform::from_xyz(0., 200., 0.),
    //     point_light: PointLight {
    //         range: 1000.,
    //         intensity: 1_000_000.,
    //         shadows_enabled: true,
    //         // shadow_depth_bias: 0.001,
    //         shadow_normal_bias: 0.9,
    //         ..default()
    //     },
    //     ..default()
    // });

    const HSIZE: f32 = 200.;
    commands.spawn_bundle(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100_000.,
            shadow_projection: OrthographicProjection {
                left: -HSIZE,
                right: HSIZE,
                bottom: -HSIZE,
                top: HSIZE,
                near: -HSIZE,
                far: HSIZE,
                ..default()
            },
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0., 5., 0.),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_8),
            ..default()
        },
        ..default()
    });
}
