use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn plain_start_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane_hx = 400.0;
    let plane_hz = 600.0;
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                max_x: plane_hx,
                min_x: -plane_hx,
                max_y: 0.5,
                min_y: -0.5,
                max_z: plane_hz,
                min_z: -plane_hz,
            })),
            material: materials.add(Color::rgba(0.2, 0.6, 0.2, 0.5).into()),
            ..default()
        })
        .insert(Name::new("Plane"))
        .insert(RigidBody::Fixed)
        // .insert_bundle(TransformBundle::identity())
        .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
            -300., -0.5, 400.,
        )))
        .insert(Velocity::zero())
        .insert(Collider::cuboid(plane_hx, 0.5, plane_hz))
        .insert(Friction::coefficient(100.))
        .insert(Restitution::coefficient(0.1));
}
