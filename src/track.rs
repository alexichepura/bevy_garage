use crate::{
    config::Config, ear_clipping::triangulate_ear_clipping, mesh::QuadPlane, shader::GroundMaterial,
};
use bevy::{
    pbr::NotShadowCaster,
    prelude::*,
    render::{mesh::*, render_resource::*, texture::ImageSampler},
};
use bevy_rapier3d::{na::Point3, prelude::*, rapier::prelude::ColliderShape};
use nalgebra::Point2;
use parry3d::{math::Point, shape::TriMesh};
// use parry3d::transformation::triangulate_ear_clipping;
use wgpu::{Extent3d, TextureDimension, TextureFormat};

// use obj::*;
use std::f32::consts::{FRAC_PI_2, PI};
// use std::fs::File;
// use std::io::BufReader;
// use std::io::Write;
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
            left: Vec::new(),
            right: Vec::new(),
            left_norm: Vec::new(),
            right_norm: Vec::new(),
        }
    }
    pub fn new() -> Self {
        // let polyline_buf = BufReader::new(File::open("assets/track-polyline.obj").unwrap());
        // let model = raw::parse_obj(polyline_buf).unwrap();
        // let pretty_config = ron::ser::PrettyConfig::default()
        //     .indentor("  ".to_string())
        //     .new_line("\n".to_string());
        // let pos_ron = ron::ser::to_string_pretty(&model.positions, pretty_config).unwrap();
        // File::create(format!("assets/track-positions.ron"))
        //     .and_then(|mut file| file.write(pos_ron.as_bytes()))
        //     .expect("Error while writing scene to file");
        // let positions = model.positions;

        let positions = TRACK_POSITIONS;
        let mut track = Track::empty();
        let mut points: Vec<Vec3> = vec![];
        points.extend(
            positions
                .iter()
                .map(|pos| Vec3::new(pos.0, pos.1, pos.2))
                .collect::<Vec<Vec3>>(),
        );
        track.points = points;
        for (i, point) in track.points.iter().enumerate() {
            let last: bool = i + 1 == track.points.len();
            let ix2: u32 = i as u32 * 2;
            if last {
                let inx = if last { 0 } else { i + 1 };
                track.left_norm.push(track.left_norm[inx]);
                track.right_norm.push(track.right_norm[inx]);
                track.left.push(track.left[inx]);
                track.right.push(track.right[inx]);
            } else {
                let (i1, i2) = ([ix2, ix2 + 1, ix2 + 2], [ix2 + 2, ix2 + 1, ix2 + 3]);
                track.indices.extend(i1);
                track.indices.extend(i2);
                track.collider_indices.push(i1);
                track.collider_indices.push(i2);

                let point_prev = if i == 0 {
                    track.points[track.points.len() - 1]
                } else {
                    track.points[i - 1]
                };
                let point_next = track.points[i + 1];
                let vec_len = (point_prev - *point).length();
                let angle = if vec_len == 0. {
                    0.
                } else {
                    (*point - point_next).angle_between(point_prev - *point) / 2.
                };
                let dir: Vec3 = (point_next - *point).normalize();
                let left_norm = Quat::from_rotation_y(FRAC_PI_2 + angle).mul_vec3(dir);
                let right_norm = -left_norm;
                track.left_norm.push(left_norm);
                track.right_norm.push(right_norm);
                track.left.push(*point + left_norm * track.width);
                track.right.push(*point + right_norm * track.width);
            }
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
    asset_server: &Res<AssetServer>,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    track: &Track,
) {
    let texture_handle = asset_server.load("8k_asphalt.jpg");
    let texture_normals_handle = asset_server.load("8k_asphalt_normals.jpg");
    let texture_metallic_roughness_handle = asset_server.load("8k_asphalt_metallic_roughness.jpg");
    let material_handle = materials.add(StandardMaterial {
        base_color_texture: Some(texture_handle.clone()),
        normal_map_texture: Some(texture_normals_handle.clone()),
        metallic_roughness_texture: Some(texture_metallic_roughness_handle.clone()),
        perceptual_roughness: 1.,
        ..default()
    });

    let (vertices, normals) = track.road();

    // let mut ps: Vec<Point<f32>> = track
    //     .left
    //     .iter()
    //     .map(|v| Point3::new(v[0], v[1], v[2]))
    //     .collect();

    // let y = -100.;
    // ps.push(Point3::new(2000., y, 2000.));
    // ps.push(Point3::new(2000., y, -2000.));
    // ps.push(Point3::new(-2000., y, -2000.));
    // ps.push(Point3::new(-2000., y, 2000.));
    // let shape = parry3d::shape::SharedShape::convex_hull(&ps).unwrap();
    // commands.spawn_empty().insert(Collider::from(shape));

    let mut right3d: Vec<Vec3> = track.right.clone();
    right3d.pop();
    let mut right3dnorm: Vec<[f32; 3]> = vec![];
    for (_i, _) in right3d.iter().enumerate() {
        right3dnorm.push(Vec3::Y.into());
    }

    let right2d: Vec<Point2<f32>> = right3d.iter().map(|v| Point2::new(v[0], v[2])).collect();
    let ind = triangulate_ear_clipping(&right2d).unwrap();
    // commands
    //     .spawn_empty()
    //     .insert(Collider::trimesh(right3d.clone(), ind.clone()))
    //     .insert(ColliderScale::Absolute(Vec3::ONE))
    //     .insert(CollisionGroups::new(STATIC_GROUP, Group::ALL))
    //     .insert(Friction {
    //         combine_rule: CoefficientCombineRule::Average,
    //         coefficient: 3.,
    //         ..default()
    //     })
    //     .insert(Restitution::coefficient(0.));

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(right3d.clone()),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::from(right3dnorm),
    );
    let ind_rev: Vec<[u32; 3]> = ind.iter().map(|indx| [indx[2], indx[1], indx[0]]).collect();
    mesh.set_indices(Some(Indices::U32(ind_rev.flatten().to_vec())));
    commands.spawn((PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.05, 0.85, 0.05).into()),
        // transform: Transform::from_xyz(0., 0.001, 0.),
        ..Default::default()
    },));

    let mut uvs: Vec<[f32; 2]> = Vec::new();
    for (i, _p) in track.points.iter().enumerate() {
        let left = track.left.get(i).unwrap();
        let right = track.right.get(i).unwrap();
        let x = 50.;
        uvs.push([left.x / x, left.z / x]);
        uvs.push([right.x / x, right.z / x]);
    }
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(vertices.clone()),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, VertexAttributeValues::from(normals));
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(track.indices.clone())));
    let generate_tangents = mesh.generate_tangents();
    if generate_tangents.is_ok() {
        println!("Generated tangents");
    }

    commands
        .spawn((
            PbrBundle {
                mesh: meshes.add(mesh),
                material: material_handle,
                // material: materials.add(Color::rgb(0.05, 0.05, 0.05).into()),
                // material: materials.add(StandardMaterial {
                //     base_color: Color::rgb(0.05, 0.05, 0.05),
                //     perceptual_roughness: 0.7,
                //     ..default()
                // }),
                transform: Transform::from_xyz(0., 0.001, 0.),
                ..Default::default()
            },
            NotShadowCaster,
        ))
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
    let kerb_height: f32 = 0.002;
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
        let (v1, v2) = (point + normals_side[i], point);
        vertices.push(v1.into());
        vertices.push(v2.into());
        let diff = point_next.sub(point).length();
        let uv = len / kerb_length;
        uvs.push([uv, 0.]);
        uvs.push([uv, 1.]);
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
        .spawn((
            PbrBundle {
                mesh: meshes.add(mesh),
                material: material.clone(),
                transform: Transform::from_xyz(0., kerb_height, 0.),
                ..Default::default()
            },
            NotShadowCaster,
        ))
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
        let (v1, v2) = (point, point + normals_side[i]);
        vertices.push(v1.into());
        vertices.push(v2.into());
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
        .spawn((
            PbrBundle {
                mesh: meshes.add(mesh),
                material: material.clone(),
                transform: Transform::from_xyz(0., kerb_height, 0.),
                ..Default::default()
            },
            NotShadowCaster,
        ))
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
    let height: f32 = 0.6;
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
    custom_materials: &mut ResMut<Assets<GroundMaterial>>,
) {
    let multiplier: usize = 2;
    let scale = 280. / multiplier as f32;
    let (cols, rows): (usize, usize) = (2 * multiplier, 3 * multiplier);
    let size: Vec2 = 2. * Vec2::new(cols as f32 * scale, rows as f32 * scale);
    let color = Color::hex("7b824e").unwrap();
    commands
        .spawn((
            Name::new("road-heightfield"),
            MaterialMeshBundle::<GroundMaterial> {
                mesh: meshes.add(Mesh::from(QuadPlane::new(size))),
                material: custom_materials.add(GroundMaterial { color }),
                ..default()
            },
            NotShadowCaster,
            RigidBody::Fixed,
            ColliderScale::Absolute(Vec3::ONE),
            CollisionGroups::new(STATIC_GROUP, Group::ALL),
            Friction::coefficient(1.),
            Restitution::coefficient(0.),
            Collider::heightfield(
                vec![0.; rows * cols],
                rows,
                cols,
                Vec3::new(size.x, 0., size.y),
            ),
        ))
        .insert(TransformBundle::from_transform(Transform::from_xyz(
            -350., 0., 570.,
        )));
}

pub fn track_start_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut custom_materials: ResMut<Assets<GroundMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    let track = Track::new();
    // spawn_ground(&mut commands, &mut meshes, &mut custom_materials);
    spawn_road(
        &asset_server,
        &mut commands,
        &mut meshes,
        &mut materials,
        &track,
    );
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
        left_wall_points.push(*p + track.right_norm[i] * 8.);
        right_wall_points.push(*p + track.right_norm[i] * -8.);
    }
    // spawn_walls(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     &mut images,
    //     &track.indices,
    //     &left_wall_points,
    //     &track.right_norm,
    // );
    // spawn_walls(
    //     &mut commands,
    //     &mut meshes,
    //     &mut materials,
    //     &mut images,
    //     &track.indices,
    //     &right_wall_points,
    //     &track.right_norm,
    // );
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
            210, 20, 20, 255, // red
            210, 210, 210, 255, // white
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

pub const _TRACK_POSITIONS: [(f32, f32, f32, f32); 6] = [
    (0., 0.0, 0., 1.0),
    (100., 0.0, 0., 1.0),
    (100., 0.0, 100., 1.0),
    (-100., 0.0, 100., 1.0),
    (-100., 0.0, 0., 1.0),
    (0., 0.0, 0., 1.0),
];

pub const TRACK_POSITIONS: [(f32, f32, f32, f32); 365] = [
    (-65.2042, 0.0, 80.13815, 1.0),
    (-115.01793, 0.0, 143.06631, 1.0),
    (-166.25946, 0.0, 207.94211, 1.0),
    (-217.50098, 0.0, 272.81802, 1.0),
    (-265.31787, 0.0, 333.1974, 1.0),
    (-278.88818, 0.0, 348.6136, 1.0),
    (-289.03387, 0.0, 359.53333, 1.0),
    (-299.1795, 0.0, 370.4531, 1.0),
    (-309.71115, 0.0, 381.52063, 1.0),
    (-325.6153, 0.0, 395.35187, 1.0),
    (-357.13297, 0.0, 422.21912, 1.0),
    (-389.7332, 0.0, 449.55347, 1.0),
    (-408.00998, 0.0, 463.88077, 1.0),
    (-414.2032, 0.0, 467.27847, 1.0),
    (-419.8355, 0.0, 469.87085, 1.0),
    (-424.9733, 0.0, 471.5963, 1.0),
    (-428.5472, 0.0, 471.43433, 1.0),
    (-430.55695, 0.0, 469.78018, 1.0),
    (-431.86078, 0.0, 467.55853, 1.0),
    (-432.82025, 0.0, 464.62982, 1.0),
    (-433.69626, 0.0, 460.75327, 1.0),
    (-434.31042, 0.0, 455.8055, 1.0),
    (-434.37216, 0.0, 449.98566, 1.0),
    (-433.95425, 0.0, 443.58792, 1.0),
    (-433.15848, 0.0, 436.92114, 1.0),
    (-431.98438, 0.0, 430.315, 1.0),
    (-430.27872, 0.0, 423.49313, 1.0),
    (-427.88867, 0.0, 415.7542, 1.0),
    (-424.6275, 0.0, 406.36566, 1.0),
    (-420.02145, 0.0, 394.77567, 1.0),
    (-414.31296, 0.0, 381.75626, 1.0),
    (-408.50928, 0.0, 368.77945, 1.0),
    (-403.61978, 0.0, 357.3127, 1.0),
    (-399.92276, 0.0, 348.08023, 1.0),
    (-397.03543, 0.0, 340.52264, 1.0),
    (-394.86655, 0.0, 333.9693, 1.0),
    (-393.35565, 0.0, 327.80286, 1.0),
    (-392.39914, 0.0, 321.95062, 1.0),
    (-391.93256, 0.0, 316.53302, 1.0),
    (-391.9601, 0.0, 311.25452, 1.0),
    (-392.49002, 0.0, 305.79987, 1.0),
    (-393.51434, 0.0, 300.11154, 1.0),
    (-395.09988, 0.0, 294.37674, 1.0),
    (-397.3798, 0.0, 288.68988, 1.0),
    (-400.48355, 0.0, 283.13736, 1.0),
    (-404.43283, 0.0, 277.85242, 1.0),
    (-409.10828, 0.0, 273.01215, 1.0),
    (-414.40457, 0.0, 268.77515, 1.0),
    (-420.1761, 0.0, 265.2932, 1.0),
    (-425.88422, 0.0, 262.65134, 1.0),
    (-431.39847, 0.0, 260.7731, 1.0),
    (-437.2533, 0.0, 259.54214, 1.0),
    (-443.98825, 0.0, 258.85468, 1.0),
    (-451.57062, 0.0, 258.7662, 1.0),
    (-460.00357, 0.0, 259.4374, 1.0),
    (-469.8863, 0.0, 260.9392, 1.0),
    (-481.91312, 0.0, 263.39816, 1.0),
    (-496.97186, 0.0, 267.5356, 1.0),
    (-514.17163, 0.0, 273.38562, 1.0),
    (-531.24316, 0.0, 279.93073, 1.0),
    (-545.9178, 0.0, 286.0777, 1.0),
    (-557.31055, 0.0, 291.0736, 1.0),
    (-566.29144, 0.0, 295.29834, 1.0),
    (-573.51685, 0.0, 299.5471, 1.0),
    (-579.5647, 0.0, 304.574, 1.0),
    (-584.5794, 0.0, 310.3618, 1.0),
    (-588.42865, 0.0, 316.41418, 1.0),
    (-591.2291, 0.0, 322.6873, 1.0),
    (-593.12213, 0.0, 329.17633, 1.0),
    (-594.20825, 0.0, 335.76184, 1.0),
    (-594.4774, 0.0, 342.1986, 1.0),
    (-593.88605, 0.0, 348.27322, 1.0),
    (-592.4117, 0.0, 353.7739, 1.0),
    (-590.2653, 0.0, 358.48477, 1.0),
    (-587.6792, 0.0, 362.38956, 1.0),
    (-584.6661, 0.0, 365.61047, 1.0),
    (-581.2511, 0.0, 368.2584, 1.0),
    (-577.7637, 0.0, 370.22656, 1.0),
    (-574.08264, 0.0, 371.52832, 1.0),
    (-569.4827, 0.0, 372.47562, 1.0),
    (-563.0689, 0.0, 373.35736, 1.0),
    (-553.0936, 0.0, 373.9443, 1.0),
    (-540.82477, 0.0, 374.0006, 1.0),
    (-530.3922, 0.0, 373.80402, 1.0),
    (-525.5116, 0.0, 373.67676, 1.0),
    (-523.04724, 0.0, 373.86285, 1.0),
    (-518.47345, 0.0, 374.49692, 1.0),
    (-513.189, 0.0, 375.72046, 1.0),
    (-509.06354, 0.0, 377.64725, 1.0),
    (-506.4927, 0.0, 380.10782, 1.0),
    (-504.57764, 0.0, 382.95535, 1.0),
    (-503.0304, 0.0, 386.3425, 1.0),
    (-501.70184, 0.0, 390.71503, 1.0),
    (-501.07376, 0.0, 398.86667, 1.0),
    (-501.36905, 0.0, 410.53516, 1.0),
    (-502.0071, 0.0, 421.0746, 1.0),
    (-502.56863, 0.0, 427.98648, 1.0),
    (-504.91052, 0.0, 452.9993, 1.0),
    (-509.68292, 0.0, 503.84277, 1.0),
    (-514.4554, 0.0, 554.6865, 1.0),
    (-516.7904, 0.0, 579.5851, 1.0),
    (-517.29285, 0.0, 585.1606, 1.0),
    (-518.0862, 0.0, 593.61584, 1.0),
    (-519.2647, 0.0, 604.97363, 1.0),
    (-520.74713, 0.0, 617.04456, 1.0),
    (-522.8299, 0.0, 629.1848, 1.0),
    (-525.3593, 0.0, 640.69836, 1.0),
    (-527.50366, 0.0, 649.3096, 1.0),
    (-528.9951, 0.0, 655.11456, 1.0),
    (-535.4711, 0.0, 681.8989, 1.0),
    (-548.6861, 0.0, 736.62506, 1.0),
    (-561.9011, 0.0, 791.35114, 1.0),
    (-568.3536, 0.0, 818.3286, 1.0),
    (-569.53, 0.0, 826.4492, 1.0),
    (-570.70795, 0.0, 839.3703, 1.0),
    (-571.44257, 0.0, 853.9521, 1.0),
    (-570.78217, 0.0, 864.7146, 1.0),
    (-568.8492, 0.0, 871.31995, 1.0),
    (-566.3807, 0.0, 877.0248, 1.0),
    (-563.4499, 0.0, 882.33984, 1.0),
    (-559.89966, 0.0, 887.50604, 1.0),
    (-554.165, 0.0, 893.0686, 1.0),
    (-546.4851, 0.0, 898.6526, 1.0),
    (-539.71094, 0.0, 902.9658, 1.0),
    (-535.73755, 0.0, 905.2305, 1.0),
    (-524.98145, 0.0, 910.3941, 1.0),
    (-503.28568, 0.0, 920.7534, 1.0),
    (-481.59003, 0.0, 931.1128, 1.0),
    (-470.85144, 0.0, 936.24805, 1.0),
    (-467.09808, 0.0, 938.1513, 1.0),
    (-460.84183, 0.0, 941.55023, 1.0),
    (-453.8461, 0.0, 945.77936, 1.0),
    (-448.8296, 0.0, 949.66534, 1.0),
    (-446.10443, 0.0, 952.7011, 1.0),
    (-444.12866, 0.0, 955.3935, 1.0),
    (-442.5293, 0.0, 958.25934, 1.0),
    (-441.08817, 0.0, 961.7788, 1.0),
    (-439.8083, 0.0, 966.1127, 1.0),
    (-438.90985, 0.0, 970.9869, 1.0),
    (-438.5368, 0.0, 976.15826, 1.0),
    (-438.81223, 0.0, 981.3528, 1.0),
    (-439.745, 0.0, 986.0166, 1.0),
    (-441.30478, 0.0, 990.5505, 1.0),
    (-443.37558, 0.0, 994.8909, 1.0),
    (-446.07837, 0.0, 999.1318, 1.0),
    (-451.66498, 0.0, 1005.1821, 1.0),
    (-459.28683, 0.0, 1011.8669, 1.0),
    (-466.0256, 0.0, 1017.2575, 1.0),
    (-470.2183, 0.0, 1020.36884, 1.0),
    (-483.80643, 0.0, 1029.5912, 1.0),
    (-511.35403, 0.0, 1048.2463, 1.0),
    (-538.90155, 0.0, 1066.9016, 1.0),
    (-552.80426, 0.0, 1076.3949, 1.0),
    (-560.91974, 0.0, 1082.88, 1.0),
    (-576.75183, 0.0, 1096.0057, 1.0),
    (-596.0649, 0.0, 1112.4362, 1.0),
    (-613.2298, 0.0, 1127.83, 1.0),
    (-627.2941, 0.0, 1141.3761, 1.0),
    (-640.83124, 0.0, 1155.0972, 1.0),
    (-654.0887, 0.0, 1169.3721, 1.0),
    (-667.04944, 0.0, 1184.3892, 1.0),
    (-679.70856, 0.0, 1200.3035, 1.0),
    (-691.9722, 0.0, 1216.924, 1.0),
    (-703.6771, 0.0, 1233.86, 1.0),
    (-714.66284, 0.0, 1250.7272, 1.0),
    (-724.8737, 0.0, 1267.3837, 1.0),
    (-734.3853, 0.0, 1283.8623, 1.0),
    (-743.25543, 0.0, 1300.0703, 1.0),
    (-751.5354, 0.0, 1315.9341, 1.0),
    (-759.2372, 0.0, 1331.6665, 1.0),
    (-766.3371, 0.0, 1346.9092, 1.0),
    (-772.8273, 0.0, 1360.6372, 1.0),
    (-778.65405, 0.0, 1371.7915, 1.0),
    (-783.33075, 0.0, 1379.703, 1.0),
    (-786.7993, 0.0, 1384.9735, 1.0),
    (-789.7208, 0.0, 1388.6624, 1.0),
    (-792.76086, 0.0, 1391.7944, 1.0),
    (-795.9293, 0.0, 1394.6609, 1.0),
    (-798.98456, 0.0, 1396.9653, 1.0),
    (-802.1735, 0.0, 1398.753, 1.0),
    (-805.784, 0.0, 1400.1136, 1.0),
    (-809.96893, 0.0, 1401.1653, 1.0),
    (-814.5014, 0.0, 1401.8734, 1.0),
    (-819.037, 0.0, 1402.0732, 1.0),
    (-823.2341, 0.0, 1401.6229, 1.0),
    (-826.8905, 0.0, 1400.7245, 1.0),
    (-830.12506, 0.0, 1399.5732, 1.0),
    (-833.1312, 0.0, 1398.0159, 1.0),
    (-836.0986, 0.0, 1395.8983, 1.0),
    (-839.118, 0.0, 1393.4063, 1.0),
    (-842.0764, 0.0, 1390.6963, 1.0),
    (-844.8236, 0.0, 1387.5676, 1.0),
    (-847.2217, 0.0, 1383.788, 1.0),
    (-849.27783, 0.0, 1379.2133, 1.0),
    (-851.028, 0.0, 1374.1077, 1.0),
    (-852.3827, 0.0, 1368.92, 1.0),
    (-853.2362, 0.0, 1364.0802, 1.0),
    (-853.46545, 0.0, 1359.6475, 1.0),
    (-853.1126, 0.0, 1355.3767, 1.0),
    (-852.34705, 0.0, 1351.1876, 1.0),
    (-851.3286, 0.0, 1347.0225, 1.0),
    (-850.00476, 0.0, 1342.8528, 1.0),
    (-848.2704, 0.0, 1338.7137, 1.0),
    (-846.19745, 0.0, 1334.6536, 1.0),
    (-843.8758, 0.0, 1330.7296, 1.0),
    (-841.37744, 0.0, 1327.0695, 1.0),
    (-838.62317, 0.0, 1323.5676, 1.0),
    (-835.451, 0.0, 1319.8938, 1.0),
    (-831.61707, 0.0, 1315.6149, 1.0),
    (-826.18005, 0.0, 1309.543, 1.0),
    (-818.7661, 0.0, 1301.2446, 1.0),
    (-810.07666, 0.0, 1291.5431, 1.0),
    (-799.6235, 0.0, 1279.7172, 1.0),
    (-775.042, 0.0, 1249.7529, 1.0),
    (-734.94806, 0.0, 1199.6663, 1.0),
    (-697.1531, 0.0, 1152.1194, 1.0),
    (-679.0134, 0.0, 1129.2075, 1.0),
    (-674.2861, 0.0, 1122.9059, 1.0),
    (-667.31836, 0.0, 1113.1593, 1.0),
    (-659.7833, 0.0, 1102.012, 1.0),
    (-654.88367, 0.0, 1093.467, 1.0),
    (-652.69336, 0.0, 1087.6216, 1.0),
    (-651.18933, 0.0, 1081.9497, 1.0),
    (-650.0792, 0.0, 1076.083, 1.0),
    (-649.2716, 0.0, 1069.916, 1.0),
    (-648.7646, 0.0, 1063.5455, 1.0),
    (-648.66394, 0.0, 1056.8918, 1.0),
    (-649.05786, 0.0, 1049.5547, 1.0),
    (-650.05646, 0.0, 1041.1195, 1.0),
    (-651.9869, 0.0, 1031.388, 1.0),
    (-654.8272, 0.0, 1020.7129, 1.0),
    (-658.1062, 0.0, 1009.59924, 1.0),
    (-661.3561, 0.0, 998.52045, 1.0),
    (-664.5649, 0.0, 987.541, 1.0),
    (-667.7342, 0.0, 976.7097, 1.0),
    (-670.41895, 0.0, 966.4733, 1.0),
    (-672.1706, 0.0, 957.3218, 1.0),
    (-672.98773, 0.0, 949.6955, 1.0),
    (-673.22833, 0.0, 943.28, 1.0),
    (-673.0429, 0.0, 937.306, 1.0),
    (-672.43585, 0.0, 930.53937, 1.0),
    (-670.29205, 0.0, 917.975, 1.0),
    (-666.54596, 0.0, 899.75165, 1.0),
    (-662.9507, 0.0, 883.20966, 1.0),
    (-660.2756, 0.0, 871.6004, 1.0),
    (-648.39417, 0.0, 823.2599, 1.0),
    (-624.14136, 0.0, 724.70264, 1.0),
    (-599.8886, 0.0, 626.1455, 1.0),
    (-588.01685, 0.0, 577.862, 1.0),
    (-585.4699, 0.0, 566.96313, 1.0),
    (-582.2554, 0.0, 552.0473, 1.0),
    (-579.19385, 0.0, 535.8664, 1.0),
    (-578.07904, 0.0, 525.20807, 1.0),
    (-578.8495, 0.0, 519.93085, 1.0),
    (-580.2771, 0.0, 515.5043, 1.0),
    (-582.21106, 0.0, 511.40204, 1.0),
    (-584.7895, 0.0, 507.4692, 1.0),
    (-589.679, 0.0, 502.93228, 1.0),
    (-596.7556, 0.0, 497.94452, 1.0),
    (-603.1727, 0.0, 493.8976, 1.0),
    (-607.4722, 0.0, 491.5452, 1.0),
    (-623.7511, 0.0, 484.48615, 1.0),
    (-656.87476, 0.0, 470.202, 1.0),
    (-689.99835, 0.0, 455.91797, 1.0),
    (-706.3221, 0.0, 448.8444, 1.0),
    (-711.1773, 0.0, 446.30872, 1.0),
    (-718.8409, 0.0, 441.82343, 1.0),
    (-727.42194, 0.0, 436.23907, 1.0),
    (-733.6259, 0.0, 431.07263, 1.0),
    (-737.3321, 0.0, 426.6945, 1.0),
    (-740.4598, 0.0, 422.38446, 1.0),
    (-743.11456, 0.0, 417.84235, 1.0),
    (-745.196, 0.0, 412.88284, 1.0),
    (-746.5815, 0.0, 407.92957, 1.0),
    (-747.42017, 0.0, 403.1585, 1.0),
    (-748.0648, 0.0, 397.97037, 1.0),
    (-748.52356, 0.0, 391.3672, 1.0),
    (-745.4203, 0.0, 379.53638, 1.0),
    (-737.8833, 0.0, 363.08496, 1.0),
    (-730.0958, 0.0, 348.38242, 1.0),
    (-723.9885, 0.0, 338.31982, 1.0),
    (-695.7259, 0.0, 297.90765, 1.0),
    (-637.99475, 0.0, 215.57397, 1.0),
    (-580.26355, 0.0, 133.23997, 1.0),
    (-551.943, 0.0, 92.91128, 1.0),
    (-545.06177, 0.0, 83.881294, 1.0),
    (-534.8899, 0.0, 71.46597, 1.0),
    (-522.9068, 0.0, 57.708076, 1.0),
    (-512.61237, 0.0, 47.89148, 1.0),
    (-500.06433, 0.0, 40.53226, 1.0),
    (-483.0107, 0.0, 32.302956, 1.0),
    (-467.7683, 0.0, 25.416548, 1.0),
    (-456.79745, 0.0, 20.844793, 1.0),
    (-408.33072, 0.0, 2.470601, 1.0),
    (-309.40173, 0.0, -34.967377, 1.0),
    (-210.47275, 0.0, -72.40536, 1.0),
    (-162.10571, 0.0, -90.71279, 1.0),
    (-152.39862, 0.0, -94.43996, 1.0),
    (-140.31851, 0.0, -99.21548, 1.0),
    (-127.875374, 0.0, -104.37261, 1.0),
    (-121.05636, 0.0, -107.71647, 1.0),
    (-119.434525, 0.0, -109.259224, 1.0),
    (-118.65479, 0.0, -110.55949, 1.0),
    (-118.15833, 0.0, -111.999504, 1.0),
    (-117.83322, 0.0, -113.86208, 1.0),
    (-117.85459, 0.0, -116.677376, 1.0),
    (-118.27408, 0.0, -120.11793, 1.0),
    (-118.77339, 0.0, -123.03669, 1.0),
    (-119.06937, 0.0, -124.4875, 1.0),
    (-119.57418, 0.0, -126.25783, 1.0),
    (-120.58323, 0.0, -130.02583, 1.0),
    (-121.619156, 0.0, -134.66245, 1.0),
    (-122.172325, 0.0, -138.8275, 1.0),
    (-122.19422, 0.0, -141.96877, 1.0),
    (-121.85974, 0.0, -144.6154, 1.0),
    (-121.03129, 0.0, -147.23032, 1.0),
    (-119.55593, 0.0, -150.18484, 1.0),
    (-117.444885, 0.0, -153.11598, 1.0),
    (-114.55189, 0.0, -155.78104, 1.0),
    (-110.46152, 0.0, -158.75069, 1.0),
    (-104.697464, 0.0, -162.64195, 1.0),
    (-96.547935, 0.0, -167.65044, 1.0),
    (-86.56804, 0.0, -173.1839, 1.0),
    (-76.39321, 0.0, -178.54279, 1.0),
    (-67.69057, 0.0, -183.05646, 1.0),
    (-61.315372, 0.0, -186.43925, 1.0),
    (-56.309635, 0.0, -189.15627, 1.0),
    (-51.31945, 0.0, -191.78577, 1.0),
    (-44.80492, 0.0, -194.95728, 1.0),
    (-33.965153, 0.0, -199.6233, 1.0),
    (-19.940641, 0.0, -205.2524, 1.0),
    (-7.759984, 0.0, -210.00179, 1.0),
    (-1.124318, 0.0, -212.48625, 1.0),
    (12.489059, 0.0, -216.89825, 1.0),
    (39.678364, 0.0, -225.65486, 1.0),
    (66.867455, 0.0, -234.41145, 1.0),
    (80.31314, 0.0, -238.69998, 1.0),
    (84.88849, 0.0, -239.63683, 1.0),
    (92.63419, 0.0, -240.71968, 1.0),
    (101.754, 0.0, -241.44905, 1.0),
    (109.249596, 0.0, -240.98724, 1.0),
    (114.79173, 0.0, -239.38567, 1.0),
    (119.82243, 0.0, -237.22177, 1.0),
    (124.29548, 0.0, -234.53355, 1.0),
    (128.02698, 0.0, -231.32362, 1.0),
    (131.12634, 0.0, -227.82646, 1.0),
    (133.8187, 0.0, -224.23863, 1.0),
    (136.1124, 0.0, -220.4993, 1.0),
    (137.99931, 0.0, -216.53261, 1.0),
    (139.53043, 0.0, -212.36807, 1.0),
    (140.75928, 0.0, -208.00507, 1.0),
    (141.68025, 0.0, -203.3181, 1.0),
    (142.27525, 0.0, -198.20486, 1.0),
    (142.45554, 0.0, -192.92119, 1.0),
    (142.18015, 0.0, -187.79083, 1.0),
    (141.51033, 0.0, -182.82297, 1.0),
    (140.27577, 0.0, -177.71089, 1.0),
    (136.10605, 0.0, -169.56802, 1.0),
    (128.7902, 0.0, -158.28867, 1.0),
    (121.75907, 0.0, -148.20049, 1.0),
    (116.03888, 0.0, -140.65422, 1.0),
    (86.867874, 0.0, -105.11299, 1.0),
    (27.652275, 0.0, -33.059837, 1.0),
    (-27.773415, 0.0, 34.381996, 1.0),
    (-65.2042, 0.0, 80.13815, 1.0),
];
