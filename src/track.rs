use crate::mesh::*;
use bevy::prelude::*;
use bevy_rapier3d::parry::bounding_volume;
use bevy_rapier3d::prelude::*;
use nalgebra::{point, Isometry};
use obj::raw::object::Polygon;
use rapier3d::prelude::SharedShape;
use std::fs::File;
use std::io::BufReader;

pub fn track_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let input = BufReader::new(File::open("assets/nurburgring-gp-track.obj").unwrap());
    let scale = 1.;

    if let Ok(model) = obj::raw::parse_obj(input) {
        let vertices: Vec<_> = model
            .positions
            .iter()
            .map(|v| point![v.0 * scale, v.1 * scale, v.2 * scale])
            .collect();
        let indices: Vec<_> = model
            .polygons
            .into_iter()
            .flat_map(|p| match p {
                Polygon::P(idx) => idx.into_iter(),
                Polygon::PT(idx) => Vec::from_iter(idx.into_iter().map(|i| i.0)).into_iter(),
                Polygon::PN(idx) => Vec::from_iter(idx.into_iter().map(|i| i.0)).into_iter(),
                Polygon::PTN(idx) => Vec::from_iter(idx.into_iter().map(|i| i.0)).into_iter(),
            })
            .collect();

        let indices: Vec<_> = indices
            .chunks(3)
            .map(|idx| [idx[0] as u32, idx[1] as u32, idx[2] as u32])
            .collect();

        let decomposed_shape = SharedShape::convex_decomposition(&vertices, &indices);

        let pbr = PbrBundle {
            mesh: meshes.add(bevy_mesh((vertices, indices))),
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
                0., 0.1, 0.,
            )));
    }
}
