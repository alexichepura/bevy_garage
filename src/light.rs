use bevy::prelude::*;

pub fn light_start_system(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 0.7,
    });

    // const HSIZE: f32 = 100.;
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 100_000.,
            // shadow_projection: OrthographicProjection {
            //     left: -HSIZE,
            //     right: HSIZE,
            //     bottom: -20.,
            //     top: 20.,
            //     near: -HSIZE,
            //     far: HSIZE,
            //     ..default()
            // },
            shadows_enabled: true,
            shadow_depth_bias: 0.3,
            // shadow_normal_bias: 0.5,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0., 20., 0.),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_8),
            ..default()
        },
        ..default()
    });
}
