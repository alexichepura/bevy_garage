use bevy::prelude::*;

pub fn light_start_system(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 0.2,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 40_000.,
            shadows_enabled: true,
            shadow_projection: OrthographicProjection {
                left: -50.,
                right: 50.,
                bottom: -20.,
                top: 20.,
                near: -50.,
                far: 50.,
                ..Default::default()
            },
            shadow_depth_bias: 0.3,
            // shadow_normal_bias: 0.5,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_8),
            ..default()
        },
        ..default()
    });
}
