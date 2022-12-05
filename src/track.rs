use crate::config::Config;
use bevy::prelude::*;
use bevy::render::mesh::*;
use bevy::render::render_resource::*;
use bevy::render::texture::ImageSampler;
use bevy_rapier3d::na::Point3;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::prelude::ColliderShape;
use wgpu::{Extent3d, TextureDimension, TextureFormat};

use obj::*;
use std::f32::consts::{FRAC_PI_2, PI};
use std::fs::File;
use std::io::BufReader;
use std::ops::Mul;
use std::ops::Sub;

// https://google.github.io/filament/Filament.html#materialsystem/parameterization/
// https://google.github.io/filament/Material%20Properties.pdf

pub const STATIC_GROUP: Group = Group::GROUP_1;
#[derive(Component, Debug)]
pub struct TrackRoad;

#[derive(Component, Debug)]
pub struct Track {
    width: f32,
    points: Vec<Vec3>,
    indices: Vec<u32>,
    collider_indices: Vec<[u32; 3]>,
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

    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut len: f32 = 0.;
    for (i, p) in track.points.iter().enumerate() {
        let last: bool = i + 1 == track.points.len();
        let i_next: usize = if last { 0 } else { i + 1 };
        let next = track.points.get(i_next).unwrap();
        let diff = next.sub(*p).length();
        uvs.push([len / 10., 0.]);
        uvs.push([len / 10., 1.]);
        len += diff;
    }
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            // material: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::rgb(0.05, 0.05, 0.05),
                perceptual_roughness: 0.7,
                ..default()
            }),
            transform: Transform::from_xyz(0., 0.001, 0.),
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
            coefficient: 3.,
            ..default()
        })
        .insert(Restitution::coefficient(0.));
}

pub fn spawn_kerb(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    track: &Track,
) {
    let material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(kerb_texture())),
        perceptual_roughness: 0.7,
        ..default()
    });
    let kerb_length: f32 = 10.;
    let from_center: f32 = 5.;
    let top_norm = Vec3::Y;

    let normals_side = &track.left_norm;
    let mut vertices: Vec<[f32; 3]> = vec![];
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut len: f32 = 0.;
    for (i, p) in track.points.iter().enumerate() {
        let last: bool = i + 1 == track.points.len();
        let i_next: usize = if last { 0 } else { i + 1 };
        let point: Vec3 = *p + normals_side[i] * from_center;
        let point_next: Vec3 = track.points[i_next] + normals_side[i_next] * from_center;
        vertices.push((point + normals_side[i]).into());
        vertices.push(point.into());
        let diff = point_next.sub(point).length();
        uvs.push([len / kerb_length, 0.]);
        uvs.push([len / kerb_length, 1.]);
        normals.push(top_norm.to_array());
        normals.push(top_norm.to_array());
        len += diff;
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(vertices.clone()),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::from(normals));
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::from(uvs));
    mesh.set_indices(Some(Indices::U32(track.indices.clone())));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: material.clone(),
            transform: Transform::from_xyz(0., 0.01, 0.),
            ..Default::default()
        })
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
            coefficient: 3.,
            ..default()
        })
        .insert(Restitution::coefficient(0.));

    let normals_side = &track.right_norm;
    let mut vertices: Vec<[f32; 3]> = vec![];
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();
    let mut len: f32 = 0.;
    for (i, p) in track.points.iter().enumerate() {
        let last: bool = i + 1 == track.points.len();
        let i_next: usize = if last { 0 } else { i + 1 };
        let point: Vec3 = *p + normals_side[i] * from_center;
        let point_next: Vec3 = track.points[i_next] + normals_side[i_next] * from_center;
        vertices.push(point.into());
        vertices.push((point + normals_side[i]).into());
        let diff = point_next.sub(point).length();
        uvs.push([len / kerb_length, 0.]);
        uvs.push([len / kerb_length, 1.]);
        normals.push(top_norm.to_array());
        normals.push(top_norm.to_array());
        len += diff;
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(vertices.clone()),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::from(normals));
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::from(uvs));
    mesh.set_indices(Some(Indices::U32(track.indices.clone())));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: material.clone(),
            transform: Transform::from_xyz(0., 0.01, 0.),
            ..Default::default()
        })
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
            coefficient: 3.,
            ..default()
        })
        .insert(Restitution::coefficient(0.));
}

pub fn spawn_walls(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    images: &mut ResMut<Assets<Image>>,
    indices_input: &Vec<u32>,
    points: &Vec<Vec3>,
    normals_input: &Vec<Vec3>,
) {
    let points_len = points.len() as u32;
    let wall_material = materials.add(StandardMaterial {
        base_color_texture: Some(images.add(wall_texture())),
        perceptual_roughness: 0.7,
        ..default()
    });
    let material_lengh = 20.;
    let width: f32 = 1.;
    let height: f32 = 1.;
    let heightv: Vec3 = Vec3::Y * height;
    let hw = width / 2.;

    let mut vertices: Vec<[f32; 3]> = vec![];
    let mut normals: Vec<[f32; 3]> = Vec::new();
    let mut uvs: Vec<[f32; 2]> = Vec::new();

    let mut len: f32 = 0.;
    for (i, p) in points.iter().enumerate() {
        let last: bool = i + 1 == points.len();
        let i_next: usize = if last { 0 } else { i + 1 };
        let normal = normals_input[i];
        let point: Vec3 = *p + normal * hw;
        let point_next: Vec3 = points[i_next] + normal * hw;
        vertices.push((point + heightv).into());
        vertices.push(point.into());

        let diff = point_next.sub(point).length();
        uvs.push([len / material_lengh, 0.]);
        uvs.push([len / material_lengh, 1.]);
        normals.push(normal.to_array());
        normals.push(normal.to_array());
        len += diff;
    }

    let mut len: f32 = 0.;
    for (i, p) in points.iter().enumerate() {
        let last: bool = i + 1 == points.len();
        let i_next: usize = if last { 0 } else { i + 1 };
        let normal = normals_input[i];
        let point: Vec3 = *p + heightv;
        let point_next: Vec3 = points[i_next] + heightv;
        vertices.push((point - normal * hw).into());
        vertices.push((point + normal * hw).into());

        let diff = point_next.sub(point).length();
        uvs.push([len / material_lengh, 0.]);
        uvs.push([len / material_lengh, 1.]);
        normals.push(Vec3::Y.to_array());
        normals.push(Vec3::Y.to_array());
        len += diff;
    }

    let mut len: f32 = 0.;
    for (i, p) in points.iter().enumerate() {
        let last: bool = i + 1 == points.len();
        let i_next: usize = if last { 0 } else { i + 1 };
        let normal = normals_input[i];
        let point: Vec3 = *p - normal * hw;
        let point_next: Vec3 = points[i_next] - normal * hw;
        vertices.push((point).into());
        vertices.push((point + heightv).into());

        let diff = point_next.sub(point).length();
        uvs.push([len / material_lengh, 0.]);
        uvs.push([len / material_lengh, 1.]);
        normals.push(normal.mul(-1.).to_array());
        normals.push(normal.mul(-1.).to_array());
        len += diff;
    }

    let mut indices: Vec<u32> = vec![];
    indices.extend(indices_input.clone());
    indices.extend(indices_input.iter().map(|ind| ind + points_len * 2));
    indices.extend(indices_input.iter().map(|ind| ind + points_len * 4));

    let collider_vertices: Vec<Point3<Real>> = vertices
        .iter()
        .map(|v| Point3::new(v[0], v[1], v[2]))
        .collect();

    let collider_indices: Vec<[u32; 3]> = indices.chunks(3).map(|i| [i[0], i[1], i[2]]).collect();

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(vertices.clone()),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::from(normals));
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, VertexAttributeValues::from(uvs));
    mesh.set_indices(Some(Indices::U32(indices)));

    commands
        .spawn(PbrBundle {
            mesh: meshes.add(mesh),
            material: wall_material.clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        })
        .insert(Collider::from(ColliderShape::trimesh(
            collider_vertices,
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
            // material: materials.add(Color::rgb(0.2, 0.35, 0.2).into()),
            material: materials.add(StandardMaterial {
                base_color: Color::hex("7b824e").unwrap(),
                perceptual_roughness: 0.3,
                ..default()
            }),
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
    mut images: ResMut<Assets<Image>>,
) {
    let track = Track::new();
    spawn_ground(&mut commands, &mut meshes, &mut materials);
    spawn_road(&mut commands, &mut meshes, &mut materials, &track);
    spawn_kerb(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut images,
        &track,
    );
    let mut left_wall_points: Vec<Vec3> = vec![];
    let mut right_wall_points: Vec<Vec3> = vec![];
    for (i, p) in track.points.iter().enumerate() {
        left_wall_points.push(*p + track.right_norm[i] * 7.);
        right_wall_points.push(*p + track.right_norm[i] * -7.);
    }
    spawn_walls(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut images,
        &track.indices,
        &left_wall_points,
        &track.right_norm,
    );
    spawn_walls(
        &mut commands,
        &mut meshes,
        &mut materials,
        &mut images,
        &track.indices,
        &right_wall_points,
        &track.right_norm,
    );
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
            .with_translation(Vec3::new(1.65, 0., 1.65))
            .with_rotation(config.quat.mul_quat(Quat::from_rotation_y(PI))),
        ..default()
    });
}

fn wall_texture() -> Image {
    let mut image = Image::new_fill(
        Extent3d {
            width: 2,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[
            8, 8, 8, 255, // darker
            128, 128, 128, 255, // dark
        ],
        TextureFormat::Rgba8UnormSrgb,
    );

    image.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
        address_mode_u: AddressMode::Repeat,
        address_mode_v: AddressMode::Repeat,
        address_mode_w: AddressMode::Repeat,
        ..Default::default()
    });

    image
}
fn kerb_texture() -> Image {
    let mut image = Image::new_fill(
        Extent3d {
            width: 2,
            height: 1,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &[
            255, 0, 0, 255, // red
            255, 255, 255, 255, // white
        ],
        TextureFormat::Rgba8UnormSrgb,
    );

    image.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
        address_mode_u: AddressMode::Repeat,
        address_mode_v: AddressMode::Repeat,
        address_mode_w: AddressMode::Repeat,
        ..Default::default()
    });

    image
}

// fn uv_debug_texture() -> Image {
//     const TEXTURE_SIZE: usize = 8;
//     let mut palette: [u8; 32] = [
//         255, 102, 159, 255, 255, 159, 102, 255, 236, 255, 102, 255, 121, 255, 102, 255, 102, 255,
//         198, 255, 102, 198, 255, 255, 121, 102, 255, 255, 236, 102, 255, 255,
//     ];
//     let mut texture_data = [0; TEXTURE_SIZE * TEXTURE_SIZE * 4];
//     for y in 0..TEXTURE_SIZE {
//         let offset = TEXTURE_SIZE * y * 4;
//         texture_data[offset..(offset + TEXTURE_SIZE * 4)].copy_from_slice(&palette);
//         palette.rotate_right(4);
//     }
//     let mut image = Image::new_fill(
//         Extent3d {
//             width: TEXTURE_SIZE as u32,
//             height: TEXTURE_SIZE as u32,
//             depth_or_array_layers: 1,
//         },
//         TextureDimension::D2,
//         &texture_data,
//         TextureFormat::Rgba8UnormSrgb,
//     );
//     image.sampler_descriptor = ImageSampler::Descriptor(SamplerDescriptor {
//         address_mode_u: AddressMode::Repeat,
//         address_mode_v: AddressMode::Repeat,
//         address_mode_w: AddressMode::Repeat,
//         ..Default::default()
//     });
//     image
// }

// fn face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
//     let (a, b, c) = (Vec3::from(a), Vec3::from(b), Vec3::from(c));
//     (b - a).cross(c - a).normalize().into()
// }
