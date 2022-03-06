use std::sync::Arc;

use bevy::{
    math::{Quat, Vec3},
    pbr::{PbrBundle, StandardMaterial},
    prelude::{
        AssetServer, Assets, BuildChildren, Color, Commands, Component, Mesh, Res, ResMut,
        Transform,
    },
};
use bevy_rapier3d::{
    physics::{ColliderBundle, ColliderPositionSync, JointBuilderComponent, RigidBodyBundle},
    prelude::{
        ColliderMaterial, ColliderShape, Cylinder, Isometry, JointAxesMask, JointData,
        MassProperties, Point, Real, RigidBodyPosition, SharedShape, Vector,
    },
};

use crate::mesh::bevy_mesh;

#[derive(Component)]
pub struct Wheel;

#[derive(Component)]
pub struct FrontJoint;

#[derive(Component)]
pub struct FrontLeftJoint;

#[derive(Component)]
pub struct FrontRightJoint;

#[derive(Component)]
pub struct BackJoint;

#[derive(Component)]
pub struct Car;

pub fn car_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let car_graphics = "hatchbackSports.obj";
    let car_hw: f32 = 0.45;
    let car_hh: f32 = 0.5;
    let car_hl: f32 = 0.8;
    let wheel_r: f32 = 0.3;
    let wheel_hw: f32 = 0.125;
    let car_pos_transform = Vec3::new(0., 2.5, 0.);
    let qvec = Vec3::new(0., 0., 0.);
    let car_quat = Quat::from_axis_angle(qvec, 0.);
    let car_isometry: Isometry<Real> = Isometry::new(car_pos_transform.into(), qvec.into());

    let car_entity = commands
        .spawn_bundle(RigidBodyBundle {
            position: RigidBodyPosition {
                position: car_isometry,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(car_hl, car_hh, car_hw).into(),
            mass_properties: MassProperties::new(
                Vec3::new(0.0, -0.4, 0.0).into(),
                1500.0,
                Vec3::new(100.0, 100.0, 100.0).into(),
            )
            .into(),
            material: ColliderMaterial {
                friction: 0.001,
                restitution: 0.1,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .with_children(|parent| {
            let mut tr: Transform = Transform {
                ..Default::default()
            };
            tr.translation = Vec3::new(0.0, -car_hh, 0.0);
            parent.spawn_bundle(PbrBundle {
                mesh: asset_server.load(car_graphics),
                material: materials.add(Color::rgb(0.3, 0.3, 0.8).into()),
                transform: tr,
                ..Default::default()
            });
        })
        .insert(Car)
        .insert(Transform::default())
        .insert(ColliderPositionSync::Discrete)
        .id();

    let shift = Vec3::new(car_hw + 0.1 + wheel_hw, -car_hh, car_hl);
    let car_anchors: [Vec3; 4] = [
        Vec3::new(shift.x, shift.y, shift.z),
        Vec3::new(-shift.x, shift.y, shift.z),
        Vec3::new(shift.x, shift.y, -shift.z),
        Vec3::new(-shift.x, shift.y, -shift.z),
    ];

    for i in 0..4 {
        let mask = JointAxesMask::X
            | JointAxesMask::Y
            | JointAxesMask::Z
            | JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z;

        let data = JointData::default()
            .lock_axes(mask)
            .local_axis1(Vector::x_axis())
            .local_axis2(Vector::y_axis())
            .local_anchor1(Point::from(car_anchors[i]))
            .local_anchor2(Vec3::new(0., 0., 0.).into());

        // joint.configure_motor_model(SpringModel::ForceBased);

        let wheel_transform = car_pos_transform + car_quat.mul_vec3(car_anchors[i]);
        let wheel_qvec = Vec3::new(0., 0., 0.);
        let wheel_isometry: Isometry<Real> =
            Isometry::new(wheel_transform.into(), wheel_qvec.into());

        let wheel_cylinder = Cylinder::new(wheel_hw, wheel_r);
        let wheel_mesh = bevy_mesh(wheel_cylinder.to_trimesh(100));
        let wheel_shape = SharedShape(Arc::new(wheel_cylinder));
        let wheel = commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(wheel_mesh),
                material: materials.add(Color::rgb(0.1, 0.1, 0.3).into()),
                ..Default::default()
            })
            .insert_bundle(RigidBodyBundle {
                position: wheel_isometry.into(),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                shape: wheel_shape.into(),
                mass_properties: MassProperties::new(
                    Vec3::new(0.0, 0.0, 0.0).into(),
                    15.0,
                    Vec3::new(1.0, 1.0, 1.0).into(),
                )
                .into(),
                material: ColliderMaterial {
                    friction: 1.0,
                    restitution: 0.1,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            })
            .insert(Wheel)
            .insert(Transform::default())
            .insert(ColliderPositionSync::Discrete)
            .id();
        if i == 0 {
            commands
                .spawn_bundle((JointBuilderComponent::new(data, car_entity, wheel),))
                .insert(FrontRightJoint)
                .insert(FrontJoint);
        } else if i == 1 {
            commands
                .spawn_bundle((JointBuilderComponent::new(data, car_entity, wheel),))
                .insert(FrontLeftJoint)
                .insert(FrontJoint);
        } else if i == 2 {
            commands
                .spawn_bundle((JointBuilderComponent::new(data, car_entity, wheel),))
                .insert(BackJoint);
        } else if i == 3 {
            commands
                .spawn_bundle((JointBuilderComponent::new(data, car_entity, wheel),))
                .insert(BackJoint);
        }
    }
}
