use crate::material::MaterialHandle;
use bevy::{prelude::*, render::mesh::*};
use bevy_garage_car::STATIC_GROUP;
use bevy_rapier3d::{na::Point3, prelude::*, rapier::prelude::ColliderShape};
use std::ops::{Mul, Sub};

pub fn spawn_walls(
    cmd: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    handled_materials: &Res<MaterialHandle>,
    indices_input: &Vec<u32>,
    points: &Vec<Vec3>,
    normals_input: &Vec<Vec3>,
) {
    let points_len = points.len() as u32;
    let material_lengh = 20.;
    let width: f32 = 0.1;
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

    cmd.spawn((
        PbrBundle {
            mesh: meshes.add(mesh),
            material: handled_materials.wall.clone(),
            transform: Transform::from_xyz(0., 0., 0.),
            ..Default::default()
        },
        Friction {
            combine_rule: CoefficientCombineRule::Min,
            coefficient: 0.1,
            ..default()
        },
        Collider::from(ColliderShape::trimesh(collider_vertices, collider_indices)),
        ColliderScale::Absolute(Vec3::ONE),
        CollisionGroups::new(STATIC_GROUP, Group::ALL),
        Restitution::coefficient(0.),
    ));
}
