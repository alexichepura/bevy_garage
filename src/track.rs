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
#[derive(Component, Debug)]
pub struct TrackRoad;

#[derive(Component, Debug)]
pub struct Track {
    width: f32,
    points: Vec<Vec3>,
    indices: Vec<u32>,
    collider_indices: Vec<[u32; 3]>,
    dindices: Vec<[[u32; 3]; 2]>, // double triangle indices
    dirs: Vec<Vec3>,
    left: Vec<Vec3>,
    right: Vec<Vec3>,
    left_norm: Vec<Vec3>,
    right_norm: Vec<Vec3>,
}

impl Track {
    pub fn empty() -> Self {
        Track {
            width: 5.,
            points: Vec::new(),
            indices: Vec::new(),
            collider_indices: Vec::new(),
            dindices: Vec::new(),
            dirs: Vec::new(),
            left: Vec::new(),
            right: Vec::new(),
            left_norm: Vec::new(),
            right_norm: Vec::new(),
        }
    }
    pub fn new() -> Self {
        let polyline_buf = BufReader::new(File::open("assets/track-polyline.obj").unwrap());
        let model = raw::parse_obj(polyline_buf).unwrap();

        let mut track = Track::empty();
        track.points = model
            .positions
            .iter()
            .map(|pos| Vec3::new(pos.0, pos.1, pos.2))
            .collect();

        for (i, point) in track.points.iter().enumerate() {
            let last: bool = i + 1 == track.points.len();
            let i_next: usize = if last { 0 } else { i + 1 };
            let ind: u32 = i as u32 * 2;
            let dindices: [[u32; 3]; 2] = [
                [ind, ind + 1, if last { 0 } else { ind + 2 }],
                [
                    if last { 0 } else { ind + 2 },
                    ind + 1,
                    if last { 1 } else { ind + 3 },
                ],
            ];
            let point_next = track.points.get(i_next).unwrap();
            let dir: Vec3 = (*point_next - *point).normalize();
            track.dindices.push(dindices);
            track.indices.extend(dindices[0]);
            track.indices.extend(dindices[1]);
            track.collider_indices.push(dindices[0]);
            track.collider_indices.push(dindices[1]);
            track.dirs.push(dir);

            let left_norm = Quat::from_rotation_y(FRAC_PI_2).mul_vec3(dir);
            let right_norm = Quat::from_rotation_y(-FRAC_PI_2).mul_vec3(dir);
            track.left_norm.push(left_norm);
            track.right_norm.push(right_norm);
            track.left.push(*point + left_norm * track.width);
            track.right.push(*point + right_norm * track.width);
        }

        track
    }
    pub fn road(&self) -> (Vec<[f32; 3]>, Vec<[f32; 3]>) {
        let mut vertices: Vec<[f32; 3]> = vec![];
        let mut normals: Vec<[f32; 3]> = vec![];
        for (i, _) in self.points.iter().enumerate() {
            vertices.push(self.left[i].into());
            vertices.push(self.right[i].into());
            normals.push(Vec3::Y.into());
            normals.push(Vec3::Y.into());
        }
        return (vertices, normals);
    }
}

pub fn spawn_road(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    track: &Track,
) {
    let (vertices, normals) = track.road();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(vertices.clone()),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::from(normals));
    mesh.set_indices(Some(Indices::U32(track.indices.clone())));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
            transform: Transform::from_xyz(0., 0.1, 0.),
            ..Default::default()
        })
        .insert(TrackRoad)
        .insert(Collider::from(ColliderShape::trimesh(
            vertices
                .iter()
                .map(|v| Point3::new(v[0], v[1], v[2]))
                .collect(),
            track.collider_indices.clone(),
        )))
        .insert(ColliderScale::Absolute(Vec3::ONE))
        .insert(CollisionGroups::new(STATIC_GROUP, Group::ALL))
        .insert(Friction {
            combine_rule: CoefficientCombineRule::Average,
            coefficient: 0.1,
            ..default()
        })
        .insert(Restitution::coefficient(0.));
}

pub fn spawn_walls(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    track: &Track,
) {
    let mut left_wall_vertices: Vec<[f32; 3]> = vec![];
    let mut right_wall_vertices: Vec<[f32; 3]> = vec![];
    for (i, _) in track.points.iter().enumerate() {
        left_wall_vertices.push((track.left[i] + Vec3::Y).into());
        left_wall_vertices.push(track.left[i].into());
        right_wall_vertices.push(track.right[i].into());
        right_wall_vertices.push((track.right[i] + Vec3::Y).into());
    }

    let mut left_wall_mesh = Mesh::new(PrimitiveTopology::TriangleList);
    left_wall_mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(left_wall_vertices.clone()),
    );
    left_wall_mesh.set_indices(Some(Indices::U32(track.indices.clone())));
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(left_wall_mesh),
            material: materials.add(Color::rgb(0.05, 0.05, 0.35).into()),
            transform: Transform::from_xyz(0., 0.1, 0.),
            ..Default::default()
        })
        .insert(Collider::from(ColliderShape::trimesh(
            left_wall_vertices
                .iter()
                .map(|v| Point3::new(v[0], v[1], v[2]))
                .collect(),
            track.collider_indices.clone(),
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
    right_wall_mesh.set_indices(Some(Indices::U32(track.indices.clone())));
    commands
        .spawn(PbrBundle {
            mesh: meshes.add(right_wall_mesh),
            material: materials.add(Color::rgb(0.05, 0.05, 0.35).into()),
            transform: Transform::from_xyz(0., 0.1, 0.),
            ..Default::default()
        })
        .insert(Collider::from(ColliderShape::trimesh(
            right_wall_vertices
                .iter()
                .map(|v| Point3::new(v[0], v[1], v[2]))
                .collect(),
            track.collider_indices.clone(),
        )))
        .insert(ColliderScale::Absolute(Vec3::ONE))
        .insert(CollisionGroups::new(STATIC_GROUP, Group::ALL))
        .insert(Friction {
            combine_rule: CoefficientCombineRule::Average,
            coefficient: 0.1,
            ..default()
        })
        .insert(Restitution::coefficient(0.));
}

pub fn spawn_ground(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) {
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

pub fn track_start_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let track = Track::new();
    spawn_road(&mut commands, &mut meshes, &mut materials, &track);
    spawn_walls(&mut commands, &mut meshes, &mut materials, &track);
    spawn_ground(&mut commands, &mut meshes, &mut materials);
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
