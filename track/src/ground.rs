use super::{GroundPbr, MaterialHandle};
use crate::mesh::QuadPlane;
use bevy::{math::Vec3Swizzles, pbr::NotShadowCaster, prelude::*, render::primitives::Aabb};
use bevy_garage_car::STATIC_GROUP;
use bevy_rapier3d::prelude::*;

#[derive(Component, Debug)]
pub struct GroundCell {
    pub is_color: bool,
}

pub fn spawn_ground_heightfield(
    cmd: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    handled_materials: &Res<MaterialHandle>,
    aabb: &Aabb,
    padding: f32,
) {
    let aabb_center: Vec3 = aabb.center.into();
    let size: Vec2 = 2. * (aabb.half_extents.xz() + padding * Vec2::ONE);
    let (cols, rows) = (10, 10);

    let meshes_n_half = 10;
    let size_s = size / (2 * meshes_n_half) as f32;
    let mut mesh = Mesh::from(QuadPlane::new(size_s));
    mesh.generate_tangents().unwrap();

    let mesh_handle = meshes.add(mesh.clone());
    for x in -meshes_n_half..meshes_n_half {
        for z in -meshes_n_half..meshes_n_half {
            cmd.spawn((
                GroundPbr {
                    mesh: mesh_handle.clone(),
                    material: handled_materials.ground.clone(),
                    transform: Transform::from_translation(Vec3::new(
                        aabb_center.x + x as f32 * size_s.x + size_s.x / 2.,
                        0.,
                        aabb_center.z + z as f32 * size_s.y + size_s.y / 2.,
                    )),
                    ..default()
                },
                NotShadowCaster,
                GroundCell {
                    // mesh_handle,
                    is_color: false,
                },
            ));
        }
    }

    cmd.spawn((
        Name::new("ground-heightfield"),
        RigidBody::Fixed,
        ColliderScale::Absolute(Vec3::ONE),
        CollisionGroups::new(STATIC_GROUP, Group::ALL),
        Friction::coefficient(3.),
        Restitution::coefficient(0.),
        Collider::heightfield(
            vec![0.; rows * cols],
            rows,
            cols,
            Vec3::new(size.x, 0., size.y),
        ),
        TransformBundle::from_transform(Transform::from_translation(aabb_center)),
    ));
}
