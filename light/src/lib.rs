use bevy::{pbr::NotShadowCaster, prelude::*};

pub fn light_start_system(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.insert_resource(AmbientLight {
        color: Color::rgb_u8(210, 220, 240),
        brightness: 0.9,
    });

    cmd.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 40_000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0., 0., 0.),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_8),
            ..default()
        },
        ..default()
    });

    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::default())),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("888888").unwrap(),
                unlit: true,
                cull_mode: None,
                ..default()
            }),
            transform: Transform::from_scale(Vec3::splat(10000.0)),
            ..default()
        },
        NotShadowCaster,
    ));
}

const K: f32 = 2.;

pub fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
    input: Res<Input<KeyCode>>,
) {
    if input.pressed(KeyCode::H) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_seconds() * K);
        }
    }
    if input.pressed(KeyCode::L) {
        for mut transform in &mut query {
            transform.rotate_y(-time.delta_seconds() * K);
        }
    }
    if input.pressed(KeyCode::J) {
        for mut transform in &mut query {
            transform.rotate_x(time.delta_seconds() * K);
        }
    }
    if input.pressed(KeyCode::K) {
        for mut transform in &mut query {
            transform.rotate_x(-time.delta_seconds() * K);
        }
    }
}
