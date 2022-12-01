use crate::config::Config;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy_rapier3d::na::Point3;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::prelude::ColliderShape;

use obj::*;
use std::f32::consts::{FRAC_PI_2, PI};
use std::fs::File;
use std::io::BufReader;

pub const STATIC_GROUP: Group = Group::GROUP_1;

#[derive(Component)]
pub struct Road;

pub fn track_start_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let polyline_buf = BufReader::new(File::open("assets/track-polyline.obj").unwrap());
    let model = raw::parse_obj(polyline_buf).unwrap();
    let points: Vec<Vec3> = model
        .positions
        .iter()
        .map(|pos| Vec3::new(pos.0, pos.1, pos.2))
        .collect();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    let mut vertices: Vec<[f32; 3]> = vec![];
    let mut indices: Vec<u32> = vec![];
    let mut collider_indices: Vec<[u32; 3]> = Vec::new();
    let mut normals: Vec<[f32; 3]> = vec![];
    let normal: [f32; 3] = [0., 1., 0.];
    let width: f32 = 5.;
    for (i, point) in points.iter().enumerate() {
        let is_last: bool = i + 1 == points.len();
        let i_next: usize = if is_last { 0 } else { i + 1 };
        let point_next = points.get(i_next).unwrap();
        let v: Vec3 = (*point_next - *point).normalize();
        let dv0: Vec3 = Quat::from_rotation_y(FRAC_PI_2).mul_vec3(v) * width;
        let dv1: Vec3 = Quat::from_rotation_y(-FRAC_PI_2).mul_vec3(v) * width;
        let v1: Vec3 = *point + dv0;
        let v2: Vec3 = *point + dv1;
        // commands.spawn(PbrBundle {
        //     mesh: meshes.add(shape::Cube::default().into()),
        //     material: materials.add(Color::rgb(0.5, 0.05, 0.05).into()),
        //     transform: Transform::from_xyz(v1.x, v1.y, v1.z),
        //     ..Default::default()
        // });
        // commands.spawn(PbrBundle {
        //     mesh: meshes.add(shape::Cube::default().into()),
        //     material: materials.add(Color::rgb(0.05, 0.05, 0.5).into()),
        //     transform: Transform::from_xyz(v2.x, v2.y, v2.z),
        //     ..Default::default()
        // });
        let ind: u32 = i as u32 * 2;
        let ituple: [[u32; 3]; 2] = [
            [ind, ind + 1, if is_last { 0 } else { ind + 2 }],
            [
                if is_last { 0 } else { ind + 2 },
                ind + 1,
                if is_last { 1 } else { ind + 3 },
            ],
        ];
        collider_indices.push(ituple[0]);
        collider_indices.push(ituple[1]);
        indices.extend(ituple[0]);
        indices.extend(ituple[1]);

        vertices.push(v1.into());
        vertices.push(v2.into());
        normals.push(normal.clone());
        normals.push(normal.clone());
    }
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(vertices.clone()),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::from(normals));
    mesh.set_indices(Some(Indices::U32(indices)));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgba(0.05, 0.05, 0.05, 0.5).into()),
            transform: Transform::from_xyz(0., 0.1, 0.),
            ..Default::default()
        })
        .insert(Name::new("road"))
        .insert(Road)
        .insert(Collider::from(ColliderShape::trimesh(
            vertices
                .iter()
                .map(|v| Point3::new(v[0], v[1], v[2]))
                .collect(),
            collider_indices,
        )))
        .insert(ColliderScale::Absolute(Vec3::ONE))
        .insert(CollisionGroups::new(STATIC_GROUP, Group::ALL))
        .insert(Friction {
            combine_rule: CoefficientCombineRule::Average,
            coefficient: 0.1,
            ..default()
        })
        .insert(Restitution::coefficient(0.));

    let multiplier: usize = 1;
    let scale = 280. / multiplier as f32;
    let num_cols: usize = 2 * multiplier;
    let num_rows: usize = 3 * multiplier;
    let hx = num_cols as f32 * scale;
    let hy = 0.5;
    let hz = num_rows as f32 * scale;
    let ground_size: Vec3 = 2. * Vec3::new(hx, hy, hz);
    let heights: Vec<Real> = vec![hy; num_rows * num_cols];
    commands
        .spawn_empty()
        .insert(Name::new("road-heightfield"))
        .insert(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box {
                max_x: hx,
                min_x: -hx,
                max_y: hy,
                min_y: -hy,
                max_z: hz,
                min_z: -hz,
            })),
            material: materials.add(Color::rgba(0.2, 0.35, 0.2, 0.5).into()),
            // material: materials.add(Color::rgb(0.1, 0.1, 0.15).into()),
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(TransformBundle::from_transform(Transform::from_xyz(
            -350., -hy, 570.,
        )))
        .insert(Collider::heightfield(
            heights,
            num_rows,
            num_cols,
            ground_size.into(),
        ))
        .insert(ColliderScale::Absolute(Vec3::ONE))
        .insert(CollisionGroups::new(STATIC_GROUP, Group::ALL))
        .insert(Friction::coefficient(1.))
        .insert(Restitution::coefficient(0.));
}

pub fn track_decorations_start_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    config: Res<Config>,
) {
    let gl_object = asset_server.load("overheadLights.glb#Scene0");
    commands.spawn(SceneBundle {
        scene: gl_object,
        transform: Transform::from_scale(Vec3::ONE * 15.)
            .with_translation(Vec3::new(
                config.translation.x + 1.65,
                0.,
                config.translation.z + 1.65,
            ))
            .with_rotation(config.quat.mul_quat(Quat::from_rotation_y(PI))),
        ..default()
    });
}
