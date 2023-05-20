use super::{AsphaltPbr, MaterialHandle, Track, TrackRoad};
use bevy::{
    pbr::NotShadowCaster,
    prelude::*,
    render::{mesh::*, primitives::Aabb},
};
use bevy_garage_car::STATIC_GROUP;
use bevy_rapier3d::{na::Point3, prelude::*, rapier::prelude::ColliderShape};

#[derive(Component, Debug)]
pub struct AsphaltCell {
    pub is_color: bool,
}

#[derive(Debug)]
pub struct AsphaltBlock {
    vertices: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    indices: Vec<u32>,
    uvs: Vec<[f32; 2]>,
}

const BLOCK_SPAN: usize = 1;

pub fn spawn_road(
    handled_materials: &Res<MaterialHandle>,
    cmd: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    track: &Track,
) -> Aabb {
    let mut blocks_indexes: Vec<Vec<usize>> = vec![];
    for i in 0..track.points.len() {
        let block_i = i % BLOCK_SPAN;
        if block_i == 0 {
            if i + 1 < track.points.len() {
                let ilast = i + 1 + BLOCK_SPAN;
                blocks_indexes.push((i..ilast).collect());
            }
        }
    }
    // dbg!(&blocks_indexes);

    let mut blocks: Vec<AsphaltBlock> = Vec::new();
    for block_indexes in blocks_indexes {
        let mut vertices: Vec<[f32; 3]> = vec![];
        let mut normals: Vec<[f32; 3]> = vec![];
        let mut indices: Vec<u32> = vec![];
        let mut uvs: Vec<[f32; 2]> = vec![];

        for (block_i, track_i) in block_indexes.iter().enumerate() {
            let left = track.left[*track_i];
            let right = track.right[*track_i];
            vertices.push(left.to_array());
            vertices.push(right.to_array());
            normals.push(Vec3::Y.into());
            normals.push(Vec3::Y.into());
            let x = 50.;
            uvs.push([left.x / x, left.z / x]);
            uvs.push([right.x / x, right.z / x]);

            let last: bool = block_i == BLOCK_SPAN;
            if !last {
                let ix2: u32 = block_i as u32 * 2;
                let (i1, i2) = ([ix2, ix2 + 1, ix2 + 2], [ix2 + 2, ix2 + 1, ix2 + 3]);
                // 2---3
                // | \ |
                // 0---1
                // 1st triangle 0 1 2
                // 2nd triangle 2 1 3
                indices.extend(i1);
                indices.extend(i2);
            }
        }

        blocks.push(AsphaltBlock {
            vertices,
            normals,
            indices,
            uvs,
        });
    }
    for block in blocks.iter() {
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        let tr: Vec3 = block.vertices[0].into();
        let vertices: Vec<[f32; 3]> = block
            .vertices
            .iter()
            .map(|v| (Vec3::new(v[0], v[1], v[2]) - tr).to_array())
            .collect();
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            VertexAttributeValues::from(vertices),
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            VertexAttributeValues::from(block.normals.clone()),
        );
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, block.uvs.clone());
        mesh.set_indices(Some(Indices::U32(block.indices.clone())));
        mesh.generate_tangents().unwrap();

        cmd.spawn((
            AsphaltCell { is_color: false },
            AsphaltPbr {
                mesh: meshes.add(mesh.clone()),
                material: handled_materials.asphalt.clone(),
                transform: Transform::from_translation(tr),
                ..default()
            },
            NotShadowCaster,
        ));
    }

    let mut uvs: Vec<[f32; 2]> = Vec::new();
    for (i, _p) in track.points.iter().enumerate() {
        let left = track.left.get(i).unwrap();
        let right = track.right.get(i).unwrap();
        let x = 50.;
        uvs.push([left.x / x, left.z / x]);
        uvs.push([right.x / x, right.z / x]);
    }
    let (track_vertices, track_normals) = track.road();
    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        VertexAttributeValues::from(track_vertices.clone()),
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        VertexAttributeValues::from(track_normals),
    );
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.set_indices(Some(Indices::U32(track.indices.clone())));
    mesh.generate_tangents().unwrap();
    let aabb = mesh.compute_aabb().unwrap();

    cmd.spawn((
        TrackRoad,
        Collider::from(ColliderShape::trimesh(
            track_vertices
                .iter()
                .map(|v| Point3::new(v[0], v[1], v[2]))
                .collect(),
            track.collider_indices.clone(),
        )),
        ColliderScale::Absolute(Vec3::ONE),
        CollisionGroups::new(STATIC_GROUP, Group::ALL),
        Friction {
            combine_rule: CoefficientCombineRule::Average,
            coefficient: 5.,
            ..default()
        },
        Restitution::coefficient(0.),
    ));
    aabb
}
