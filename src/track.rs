use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy_rapier3d::prelude::*;
use nalgebra::point;
use rapier3d::prelude::SharedShape;
use std::fs::File;
use std::io::BufReader;

pub fn track_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let track = "assets/nurburgring-gp-track.obj";
    // let track = "assets/ring.obj";
    let input = BufReader::new(File::open(track).unwrap());
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

    let decomposed_shape = SharedShape::convex_decomposition_with_params(
        &vertices,
        &indices,
        &VHACDParameters {
            // alpha: 0.01,
            // beta: 0.01,
            resolution: 1400,
            concavity: 0.0011,
            // max_convex_hulls: 2048,
            // plane_downsampling: 2,
            // convex_hull_downsampling: 2,
            // fill_mode: FillMode::FloodFill {
            //     detect_cavities: true,
            // },
            ..VHACDParameters::default()
        },
    );
    // let decomposed_shape = SharedShape::convex_decomposition(&vertices, &indices);

    let pbr = PbrBundle {
        mesh: meshes.add(mesh),
        material: materials.add(Color::rgb(0.3, 0.1, 0.3).into()),
        ..default()
    };

    commands
        .spawn()
        .insert_bundle(pbr)
        .insert(Name::new("Track"))
        .insert(Collider::from(decomposed_shape))
        .insert(RigidBody::Fixed)
        .insert(Velocity::zero())
        .insert(Friction::coefficient(100.))
        .insert(Restitution::coefficient(0.1))
        // .insert_bundle(TransformBundle::identity())
        .insert_bundle(TransformBundle::from_transform(Transform::from_xyz(
            0., 0.05, 0.,
        )));
}
