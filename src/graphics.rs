use crate::Car;
use bevy::prelude::*;
use bevy::render::{
    camera::Camera3d,
    mesh::{Indices, VertexAttributeValues},
    render_resource::PrimitiveTopology,
};
use bevy_rapier3d::prelude::*;
use core::f32::consts::PI;
use rapier3d::math::Point;
use rapier3d::prelude::ColliderShape;
use smooth_bevy_cameras::controllers::unreal::{UnrealCameraBundle, UnrealCameraController};

pub fn camera_focus_system(
    mut transforms: ParamSet<(
        Query<(&mut Transform, &Camera, With<Camera3d>)>,
        Query<(&Transform, &Car)>,
    )>,
) {
    let p1 = transforms.p1();
    let (car_transform, _car) = p1.single();
    let mut tf = Transform::from_matrix(car_transform.compute_matrix());
    let shift_vec: Vec3 = tf.rotation.mul_vec3(Vec3::new(0., 2.5, -8.));
    tf.translation.x = tf.translation.x + shift_vec.x;
    tf.translation.y = tf.translation.y + shift_vec.y;
    tf.translation.z = tf.translation.z + shift_vec.z;
    tf.rotate(Quat::from_rotation_y(-PI));
    tf.look_at(car_transform.translation + Vec3::new(0., 1., 0.), Vec3::Y);
    for (mut cam_transform, _, _) in transforms.p0().iter_mut() {
        *cam_transform = tf;
    }
}

pub fn graphics_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // commands.spawn_bundle(PerspectiveCameraBundle {
    //     transform: Transform::from_translation(Vec3::new(10., 2.5, 10.))
    //         .looking_at(Vec3::ZERO, Vec3::Y),
    //     ..Default::default()
    // });
    commands
        .spawn_bundle(PerspectiveCameraBundle::default())
        .insert_bundle(UnrealCameraBundle::new(
            UnrealCameraController::default(),
            Vec3::new(0., 5., 10.),
            Vec3::new(0., 0., 0.),
        ));
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(-10., 40., 20.),
        point_light: PointLight {
            range: 100.,
            intensity: 100_000.,
            ..Default::default()
        },
        ..Default::default()
    });
    let plane_half = 100.0;
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane {
                size: plane_half * 2.0,
            })),
            material: materials.add(Color::rgba(0.2, 0.6, 0.2, 0.5).into()),
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert_bundle(TransformBundle::identity()) //  from(Transform::identity())
        .insert(Velocity::zero())
        .insert(Collider::cuboid(plane_half, 0.5, plane_half))
        .insert(Friction::coefficient(100.))
        .insert(Restitution::coefficient(0.1));
    // TOY OBJECT
    commands
        // .spawn_bundle(PbrBundle {
        //     mesh: meshes.add(Mesh::from(shape::Box {
        //         max_x: 0.5,
        //         min_x: -0.5,
        //         max_y: 0.5,
        //         min_y: -0.5,
        //         max_z: 0.5,
        //         min_z: -0.5,
        //     })),
        //     material: materials.add(Color::rgb(0.9, 0.5, 0.5).into()),
        //     ..Default::default()
        // })
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert_bundle(TransformBundle::from(
            Transform::from_translation(Vec3::new(1., 1., 4.0)).with_rotation(
                Quat::from_axis_angle(Vec3::new(PI / 4., PI / 4., PI / 4.), 0.),
            ),
        ))
        .insert(Velocity::zero())
        .insert(Collider::cuboid(0.5, 0.5, 0.5));

    // TOY OBJECT 2
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                max_x: 1.5,
                min_x: -1.5,
                max_y: 1.0,
                min_y: -1.0,
                max_z: 0.5,
                min_z: -0.5,
            })),
            material: materials.add(Color::rgb(0.5, 0.5, 0.9).into()),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert_bundle(TransformBundle::from(
            Transform::from_translation(Vec3::new(10.0, 3.0, -10.0)).with_rotation(
                Quat::from_axis_angle(Vec3::new(PI / 4.0, PI / 4.0, PI / 4.0), 0.),
            ),
        ))
        .insert(Velocity::zero())
        .insert(Collider::cuboid(1.5, 1.0, 0.5));

    let texture_handle = asset_server.load("array_texture.png");

    // TOY OBJECT 3
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                max_x: 1.5,
                min_x: -1.5,
                max_y: 2.0,
                min_y: -2.0,
                max_z: 0.5,
                min_z: -0.5,
            })),
            // material: materials.add(Color::rgb(0.5, 0.9, 0.9).into()),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                // roughness: 0.2,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert_bundle(TransformBundle::from(
            Transform::from_translation(Vec3::new(10.0, 3.0, 10.0))
                .with_rotation(Quat::from_axis_angle(Vec3::new(0., 0., 0.), 0.)),
        ))
        .insert(Velocity::zero())
        .insert(Collider::cuboid(1.5, 2.0, 0.5));

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let vertices: Vec<[f32; 3]> = vec![
        [0., 0., 0.],
        [0., 0., 10.],
        [5., 2., 0.],
        [5., 2., 10.],
        [10., 6., 0.],
        [10., 6., 10.],
        // [15., 12., 0.],
        // [15., 12., 10.],
    ];

    let mut collider_vertices: Vec<Point<Real>> = Vec::new();
    collider_vertices.push(vertices[0].into());
    collider_vertices.push(vertices[1].into());
    collider_vertices.push(vertices[2].into());
    collider_vertices.push(vertices[3].into());
    collider_vertices.push(vertices[4].into());
    collider_vertices.push(vertices[5].into());

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(vertices.clone()),
    );

    let n1: [f32; 3] = face_normal(vertices[0], vertices[2], vertices[1]);
    let n2: [f32; 3] = face_normal(vertices[2], vertices[4], vertices[3]);
    let normals: Vec<[f32; 3]> = vec![n1, n1, n1, n2, n2, n2];
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::from(normals));

    let uvs: Vec<[f32; 2]> = vec![
        [0.0, 0.0],
        [0.0, 1.0],
        [1.0, 0.0],
        [1.0, 1.0],
        [0.0, 0.0],
        [1.0, 1.0],
    ];
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::from(uvs));

    let rinds = vec![0, 1, 2, 2, 1, 3, 2, 3, 4, 4, 3, 5];
    let mut collider_indices: Vec<[u32; 3]> = Vec::new();
    collider_indices.push([rinds[0], rinds[1], rinds[2]]);
    collider_indices.push([rinds[3], rinds[4], rinds[5]]);
    collider_indices.push([rinds[6], rinds[7], rinds[8]]);
    collider_indices.push([rinds[9], rinds[10], rinds[11]]);
    mesh.set_indices(Some(Indices::U32(rinds)));

    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(StandardMaterial {
                base_color_texture: Some(texture_handle.clone()),
                // roughness: 0.5,
                // metallic: 0.9,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(RigidBody::Fixed)
        .insert_bundle(TransformBundle::from(
            Transform::from_translation(Vec3::new(-5.0, 0.0, -5.0))
                .with_rotation(Quat::from_axis_angle(Vec3::new(0., 1., 0.), PI)),
        ))
        .insert(Collider::from(ColliderShape::trimesh(
            collider_vertices,
            collider_indices,
        )));
}
fn face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    let (a, b, c) = (Vec3::from(a), Vec3::from(b), Vec3::from(c));
    (b - a).cross(c - a).normalize().into()
}
