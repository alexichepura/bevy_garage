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

    let mut road_vertices: Vec<[f32; 3]> = vec![];
    let mut road_indices: Vec<u32> = vec![];
    let mut road_collider_indices: Vec<[u32; 3]> = Vec::new();
    let mut road_normals: Vec<[f32; 3]> = vec![];

    let mut left_wall_vertices: Vec<[f32; 3]> = vec![];
    let mut left_wall_indices: Vec<u32> = vec![];
    let mut left_wall_collider_indices: Vec<[u32; 3]> = Vec::new();
    let mut left_wall_normals: Vec<[f32; 3]> = vec![];
    let mut right_wall_vertices: Vec<[f32; 3]> = vec![];
    let mut right_wall_indices: Vec<u32> = vec![];
    let mut right_wall_collider_indices: Vec<[u32; 3]> = Vec::new();
    let mut right_wall_normals: Vec<[f32; 3]> = vec![];

    let normal: [f32; 3] = [0., 1., 0.];
    let width: f32 = 5.;
    for (i, point) in points.iter().enumerate() {
        let last: bool = i + 1 == points.len();
        let ind: u32 = i as u32 * 2;
        let ituple: [[u32; 3]; 2] = [
            [ind, ind + 1, if last { 0 } else { ind + 2 }],
            [
                if last { 0 } else { ind + 2 },
                ind + 1,
                if last { 1 } else { ind + 3 },
            ],
        ];
        let i_next: usize = if last { 0 } else { i + 1 };
        let point_next = points.get(i_next).unwrap();
        let point_direction: Vec3 = (*point_next - *point).normalize();
        let dv: (Vec3, Vec3) = (
            Quat::from_rotation_y(FRAC_PI_2).mul_vec3(point_direction) * width,
            Quat::from_rotation_y(-FRAC_PI_2).mul_vec3(point_direction) * width,
        );
        let sides: (Vec3, Vec3) = (*point + dv.0, *point + dv.1);
        road_vertices.push(sides.0.into());
        road_vertices.push(sides.1.into());
        road_indices.extend(ituple[0]);
        road_indices.extend(ituple[1]);
        road_collider_indices.push(ituple[0]);
        road_collider_indices.push(ituple[1]);
        road_normals.push(normal.clone());
        road_normals.push(normal.clone());

        let walls: (Vec3, Vec3) = (sides.0 + Vec3::Y, sides.1 + Vec3::Y);
        left_wall_vertices.push(sides.0.into());
        left_wall_vertices.push(walls.0.into());
        left_wall_indices.extend(ituple[0]);
        left_wall_indices.extend(ituple[1]);
        left_wall_collider_indices.push(ituple[0]);
        left_wall_collider_indices.push(ituple[1]);
        // left_wall_normals.push(normal.clone());
        // left_wall_normals.push(normal.clone());

        right_wall_vertices.push(sides.1.into());
        right_wall_vertices.push(walls.1.into());
        right_wall_indices.extend(ituple[0]);
        right_wall_indices.extend(ituple[1]);
        right_wall_collider_indices.push(ituple[0]);
        right_wall_collider_indices.push(ituple[1]);
        // right_wall_normals.push(normal.clone());
        // right_wall_normals.push(normal.clone());
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(road_vertices.clone()),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::from(road_normals),
    );
    mesh.set_indices(Some(Indices::U32(road_indices)));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
            transform: Transform::from_xyz(0., 0.1, 0.),
            ..Default::default()
        })
        .insert(Road)
        .insert(Collider::from(ColliderShape::trimesh(
            road_vertices
                .iter()
                .map(|v| Point3::new(v[0], v[1], v[2]))
                .collect(),
            road_collider_indices,
        )))
        .insert(ColliderScale::Absolute(Vec3::ONE))
        .insert(CollisionGroups::new(STATIC_GROUP, Group::ALL))
        .insert(Friction {
            combine_rule: CoefficientCombineRule::Average,
            coefficient: 0.1,
            ..default()
        })
        .insert(Restitution::coefficient(0.));

    let mut left_wall_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    left_wall_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(left_wall_vertices.clone()),
    );
    // left_wall_mesh.insert_attribute(
    //     Mesh::ATTRIBUTE_NORMAL,
    //     VertexAttributeValues::from(road_normals),
    // );
    left_wall_mesh.set_indices(Some(Indices::U32(left_wall_indices)));
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(left_wall_mesh),
            material: materials.add(Color::rgb(0.05, 0.05, 0.35).into()),
            transform: Transform::from_xyz(0., 0.1, 0.),
            ..Default::default()
        })
        .insert(Road)
        .insert(Collider::from(ColliderShape::trimesh(
            left_wall_vertices
                .iter()
                .map(|v| Point3::new(v[0], v[1], v[2]))
                .collect(),
            left_wall_collider_indices,
        )))
        .insert(ColliderScale::Absolute(Vec3::ONE))
        .insert(CollisionGroups::new(STATIC_GROUP, Group::ALL))
        .insert(Friction {
            combine_rule: CoefficientCombineRule::Average,
            coefficient: 0.1,
            ..default()
        })
        .insert(Restitution::coefficient(0.));

    let mut right_wall_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    right_wall_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(right_wall_vertices.clone()),
    );
    // right_wall_mesh.insert_attribute(
    //     Mesh::ATTRIBUTE_NORMAL,
    //     VertexAttributeValues::from(road_normals),
    // );
    right_wall_mesh.set_indices(Some(Indices::U32(right_wall_indices)));
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(right_wall_mesh),
            material: materials.add(Color::rgb(0.05, 0.05, 0.35).into()),
            transform: Transform::from_xyz(0., 0.1, 0.),
            ..Default::default()
        })
        .insert(Road)
        .insert(Collider::from(ColliderShape::trimesh(
            right_wall_vertices
                .iter()
                .map(|v| Point3::new(v[0], v[1], v[2]))
                .collect(),
            right_wall_collider_indices,
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
