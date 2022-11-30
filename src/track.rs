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
    let mut normals: Vec<[f32; 3]> = vec![];
    let normal: [f32; 3] = [0., 1., 0.];
    let width: f32 = 4.;
    for (i, point) in points.iter().enumerate() {
        let i_next: usize = if i + 1 == points.len() { 0 } else { i + 1 };
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
        indices.push(ind);
        indices.push(ind + 1);
        indices.push(ind + 2);
        indices.push(ind + 2);
        indices.push(ind + 1);
        indices.push(ind + 3);
        // indices.push(ind + 3);
        // indices.push(ind + 4);
        // indices.push(ind + 5);

        vertices.push(v1.into());
        vertices.push(v2.into());
        // vertices.push((*point_next + dv0).into());
        // vertices.push((*point_next + dv0).into());
        // vertices.push((*point + dv1).into());
        // vertices.push((*point_next + dv1).into());

        normals.push(normal.clone());
        normals.push(normal.clone());
    }
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(vertices),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::from(normals));
    mesh.set_indices(Some(Indices::U32(indices)));
    // mesh.compute_flat_normals();

    commands.spawn(PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgba(0.05, 0.05, 0.05, 0.5).into()),
        transform: Transform::from_xyz(0., 0.1, 0.),
        ..Default::default()
    });

    // TODO delete
    let geoms = models();
    for obj_path in geoms.into_iter() {
        let is_road = obj_path.contains("road.obj");
        let input = BufReader::new(File::open(&obj_path).unwrap());
        let model = raw::parse_obj(input).unwrap();
        let obj: Obj<TexturedVertex, u32> = Obj::new(model).unwrap();

        let positions: Vec<[f32; 3]> = obj
            .vertices
            .iter()
            .map(|v| {
                [
                    v.position[0],
                    match is_road {
                        true => 0., // fix small deviations from 0. after blender obj triangulation export
                        false => v.position[1],
                    },
                    v.position[2],
                ]
            })
            .collect();
        let normales: Vec<[f32; 3]> = obj.vertices.iter().map(|v| v.normal).collect();
        let uv_data: Vec<[f32; 2]> = obj
            .vertices
            .iter()
            .map(|v| [v.texture[0], 1.0 - v.texture[1]])
            .collect();

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions.clone());
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normales);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uv_data);
        mesh.set_indices(Some(Indices::U32(
            obj.indices.iter().map(|i| *i as u32).collect(),
        )));

        let vertices: Vec<Point3<Real>> = positions
            .iter()
            .map(|v| Point3::new(v[0], v[1], v[2]))
            .collect();

        let indices: Vec<_> = obj
            .indices
            .chunks(3)
            .map(|idx| [idx[0] as u32, idx[1] as u32, idx[2] as u32])
            .collect();

        let h = match is_road {
            true => 0.001,
            false => 0.,
        };
        let id = commands
            .spawn_empty()
            .insert(Name::new(obj_path))
            .insert(RigidBody::Fixed)
            .insert(PbrBundle {
                transform: Transform::from_translation(Vec3::new(0., h, 0.)),
                mesh: meshes.add(mesh),
                material: materials.add(match is_road {
                    true => Color::rgb(0.1, 0.1, 0.15).into(),
                    false => Color::rgb(0.2, 0.2, 0.2).into(),
                }),
                ..default()
            })
            .id();
        if !is_road {
            commands
                .entity(id)
                .insert(Collider::from(ColliderShape::trimesh(vertices, indices)))
                .insert(ColliderScale::Absolute(Vec3::ONE))
                .insert(CollisionGroups::new(STATIC_GROUP, Group::ALL))
                .insert(Friction {
                    combine_rule: CoefficientCombineRule::Average,
                    coefficient: 0.1,
                    ..default()
                })
                .insert(Restitution::coefficient(0.));
        }
    }
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

pub const ASSET_ROAD: &str = "assets/road.obj";

fn models() -> Vec<String> {
    vec![
        // ASSET_ROAD.to_string(),
        // "assets/border-left.obj".to_string(),
        // "assets/border-right.obj".to_string(),
    ]
}

// fn face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
//     let (a, b, c) = (Vec3::from(a), Vec3::from(b), Vec3::from(c));
//     (b - a).cross(c - a).normalize().into()
// }

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
