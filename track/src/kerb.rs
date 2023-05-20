use super::track::Track;
use crate::material::MaterialHandle;
use bevy::{pbr::NotShadowCaster, prelude::*, render::mesh::*};
use bevy_garage_car::STATIC_GROUP;
use bevy_rapier3d::{na::Point3, prelude::*, rapier::prelude::ColliderShape};
use std::ops::Sub;

pub fn spawn_kerb(
    cmd: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    handled_materials: &Res<MaterialHandle>,
    track: &Track,
) {
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

    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: handled_materials.kerb.clone(),
            transform: Transform::from_xyz(0., kerb_height, 0.),
            ..Default::default()
        },
        Collider::from(ColliderShape::trimesh(
            vertices
                .iter()
                .map(|v| Point3::new(v[0], v[1], v[2]))
                .collect(),
            track.collider_indices.clone(),
        )),
        Friction {
            combine_rule: CoefficientCombineRule::Average,
            coefficient: 3.,
            ..default()
        },
        NotShadowCaster,
        ColliderScale::Absolute(Vec3::ONE),
        CollisionGroups::new(STATIC_GROUP, Group::ALL),
        Restitution::coefficient(0.),
    ));

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

    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: handled_materials.kerb.clone(),
            transform: Transform::from_xyz(0., kerb_height, 0.),
            ..Default::default()
        },
        Collider::from(ColliderShape::trimesh(
            vertices
                .iter()
                .map(|v| Point3::new(v[0], v[1], v[2]))
                .collect(),
            track.collider_indices.clone(),
        )),
        Friction {
            combine_rule: CoefficientCombineRule::Average,
            coefficient: 3.,
            ..default()
        },
        NotShadowCaster,
        ColliderScale::Absolute(Vec3::ONE),
        CollisionGroups::new(STATIC_GROUP, Group::ALL),
        Restitution::coefficient(0.),
    ));
}
