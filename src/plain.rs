use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn plain_start_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let plane_hx = 800.0;
    let plane_hy = 0.5;
    let plane_hz = 1200.0;
    commands
        .spawn()
        .insert(Name::new("Plane"))
        .insert_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                max_x: plane_hx,
                min_x: -plane_hx,
                max_y: 0.5,
                min_y: -0.5,
                max_z: plane_hz,
                min_z: -plane_hz,
            })),
            material: materials.add(Color::rgba(0.2, 0.4, 0.2, 1.).into()),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
            -600.,
            -plane_hy - 2.,
            800.,
        )));
    // .insert(Collider::from(ColliderShape::cuboid(
    //     plane_hx, plane_hy, plane_hz,
    // )))
    // .insert(ColliderScale::Absolute(Vec3::ONE))
    // .insert(CollisionGroups::new(STATIC_GROUP, u32::MAX))
    // .insert(Friction::coefficient(1.))
    // .insert(Restitution::coefficient(0.));
}
