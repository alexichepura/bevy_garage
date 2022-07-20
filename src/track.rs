use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy_rapier3d::prelude::*;
use nalgebra::point;
use rapier3d::prelude::ColliderShape;
use std::fs::File;
use std::io::BufReader;

pub const STATIC_GROUP: u32 = 0b010;

pub fn track_start_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let geoms = models();
    for obj_path in geoms.into_iter() {
        let input = BufReader::new(File::open(obj_path).unwrap());
        let model = obj::raw::parse_obj(input).unwrap();
        let obj: obj::Obj<obj::TexturedVertex, u32> = obj::Obj::new(model).unwrap();

        let positions: Vec<[f32; 3]> = obj.vertices.iter().map(|v| v.position).collect();
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

        let vertices: Vec<_> = positions.iter().map(|v| point![v[0], v[1], v[2]]).collect();

        let indices: Vec<_> = obj
            .indices
            .chunks(3)
            .map(|idx| [idx[0] as u32, idx[1] as u32, idx[2] as u32])
            .collect();

        let collider = Collider::from(ColliderShape::trimesh(vertices, indices));

        let pbr = PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.3, 0.1, 0.3).into()),
            ..default()
        };

        commands
            .spawn()
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(ContactForceEventThreshold(0.01))
            .insert_bundle(pbr)
            .insert(Name::new("Track"))
            .insert(collider)
            .insert(CollisionGroups::new(STATIC_GROUP, u32::MAX))
            .insert(RigidBody::Fixed)
            .insert(Velocity::zero())
            .insert(Friction::coefficient(1000.))
            .insert(Restitution::coefficient(0.00000001))
            .insert_bundle(TransformBundle::identity());
    }
}

fn models() -> Vec<String> {
    vec!["assets/track.obj".to_string()]
}
