use bevy::light::{GlobalAmbientLight, NotShadowCaster};
use bevy::prelude::*;

pub fn light_start_system(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.insert_resource(GlobalAmbientLight {
        color: Color::srgb_u8(210, 220, 240),
        brightness: 80.,
        affects_lightmapped_meshes: true,
    });

    cmd.spawn((
        DirectionalLight {
            illuminance: 10_000.,
            shadows_enabled: true,
            ..default()
        },
        Transform {
            translation: Vec3::new(0., 0., 0.),
            rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_8),
            ..default()
        },
    ));

    cmd.spawn((
        Mesh3d(meshes.add(Mesh::from(Cuboid::default()))),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Srgba::hex("888888").unwrap().into(),
            unlit: true,
            cull_mode: None,
            ..default()
        })),
        Transform::from_scale(Vec3::splat(10000.0)),
        NotShadowCaster,
    ));
}

const K: f32 = 2.;

pub fn animate_light_direction(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<DirectionalLight>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.pressed(KeyCode::KeyH) {
        for mut transform in &mut query {
            transform.rotate_y(time.delta_secs() * K);
        }
    }
    if input.pressed(KeyCode::KeyL) {
        for mut transform in &mut query {
            transform.rotate_y(-time.delta_secs() * K);
        }
    }
    if input.pressed(KeyCode::KeyJ) {
        for mut transform in &mut query {
            transform.rotate_x(time.delta_secs() * K);
        }
    }
    if input.pressed(KeyCode::KeyK) {
        for mut transform in &mut query {
            transform.rotate_x(-time.delta_secs() * K);
        }
    }
}
