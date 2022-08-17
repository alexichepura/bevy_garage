use crate::config::Config;
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy_rapier3d::na::Point3;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::prelude::ColliderShape;

use std::f32::consts::PI;
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
        let is_road = obj_path.contains("road.obj");
        let input = BufReader::new(File::open(&obj_path).unwrap());
        let model = obj::raw::parse_obj(input).unwrap();
        let obj: obj::Obj<obj::TexturedVertex, u32> = obj::Obj::new(model).unwrap();

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

        let collider = Collider::from(ColliderShape::trimesh(vertices, indices));

        let pbr = PbrBundle {
            mesh: meshes.add(mesh),
            material: materials.add(Color::rgb(0.1, 0.1, 0.15).into()),
            ..default()
        };

        let h = match is_road {
            true => 0.01,
            false => 0.,
        };
        let restitution = match is_road {
            true => 0.1,
            false => 0.1,
        };
        let friction = match is_road {
            true => 1.,
            false => 0.01,
        };
        commands
            .spawn()
            .insert_bundle(pbr)
            .insert(Name::new(obj_path))
            .insert(collider)
            .insert(ColliderScale::Absolute(Vec3::ONE))
            .insert(CollisionGroups::new(STATIC_GROUP, u32::MAX))
            .insert(RigidBody::Fixed)
            .insert(Friction::coefficient(friction))
            .insert(Restitution::coefficient(restitution))
            .insert_bundle(TransformBundle::from_transform(Transform {
                translation: Vec3::new(0., h, 0.),
                ..default()
            }));
    }
}

pub const ASSET_ROAD: &str = "assets/road.obj";

fn models() -> Vec<String> {
    vec![
        ASSET_ROAD.to_string(),
        "assets/border-left.obj".to_string(),
        "assets/border-right.obj".to_string(),
    ]
}

pub fn track_decorations_start_system(
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    config: Res<Config>,
) {
    let gl_object = asset_server.load("overheadLights.glb#Scene0");
    commands.spawn_bundle(SceneBundle {
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
