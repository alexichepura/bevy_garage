use bevy::prelude::*;

pub fn light_start_system(mut commands: Commands) {
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(0., 40., 0.),
        point_light: PointLight {
            range: 1000.,
            intensity: 100_000.,
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // directional 'sun' light
    // const HALF_SIZE: f32 = 10.0;
    // commands.spawn_bundle(DirectionalLightBundle {
    //     directional_light: DirectionalLight {
    //         illuminance: 10000.0,
    //         // Configure the projection to better fit the scene
    //         shadow_projection: OrthographicProjection {
    //             left: -HALF_SIZE,
    //             right: HALF_SIZE,
    //             bottom: -HALF_SIZE,
    //             top: HALF_SIZE,
    //             near: -10.0 * HALF_SIZE,
    //             far: 10.0 * HALF_SIZE,
    //             ..default()
    //         },
    //         shadows_enabled: true,
    //         ..default()
    //     },
    //     transform: Transform {
    //         translation: Vec3::new(0.0, 2.0, 0.0),
    //         rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
    //         ..default()
    //     },
    //     ..default()
    // });
}
