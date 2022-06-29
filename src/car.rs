use bevy::prelude::*;
use bevy_rapier3d::{parry::shape::Cylinder, prelude::*};
use rapier3d::prelude::{Isometry, JointAxesMask, SharedShape};
use std::{f32::consts::PI, sync::Arc};

use crate::mesh::bevy_mesh;

#[derive(Component)]
pub struct Wheel;
// #[derive(Component)]
// struct Wheel2 {
//     diameter: f32,
//     widrh: f32,
// }
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
    let car_transform = Vec3::new(0., 1.5, 0.);
    let qvec = Vec3::new(0., 1., 0.).normalize();
    let car_quat = Quat::from_axis_angle(qvec, 0.);

    let car = commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert(Velocity::zero())
        .insert(Collider::cuboid(car_hl, car_hh, car_hw))
        .insert(Friction::coefficient(0.001))
        .insert(Restitution::coefficient(0.1))
        .insert_bundle(TransformBundle::from(
            Transform::from_translation(car_transform).with_rotation(car_quat),
        ))
        .insert(AdditionalMassProperties(MassProperties {
            local_center_of_mass: Vec3::new(0.0, -0.4, 0.0),
            mass: 1500.0,
            principal_inertia: Vec3::new(100.0, 100.0, 100.0),
            ..Default::default()
        }))
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
        .id();
    let shift = Vec3::new(car_hw + 0.1 + wheel_hw, -car_hh, car_hl);
    let car_anchors: [Vec3; 4] = [
        Vec3::new(shift.x, shift.y, shift.z),
        Vec3::new(-shift.x, shift.y, shift.z),
        Vec3::new(shift.x, shift.y, -shift.z),
        Vec3::new(-shift.x, shift.y, -shift.z),
    ];

    for i in 0..4 {
        let joint_mask = JointAxesMask::X
            | JointAxesMask::Y
            | JointAxesMask::Z
            | JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z;

        // let joint = RevoluteJointBuilder::new(x)
        //     .local_anchor1(Vec3::new(0.0, 0.0, 1.0))
        //     .local_anchor2(Vec3::new(0.0, 0.0, -3.0));

        let joint_builder = GenericJointBuilder::new(joint_mask)
            .local_axis1(Vec3::X)
            .local_axis2(Vec3::Y)
            .local_anchor1(car_anchors[i])
            .local_anchor2(Vec3::new(0., 0., 0.).into())
            .build();

        let wheel_transform = car_transform + car_quat.mul_vec3(car_anchors[i]);
        let wheel_cylinder = Cylinder::new(wheel_hw, wheel_r);
        let wheel_mesh = bevy_mesh(wheel_cylinder.to_trimesh(100));
        let wheel_shape = SharedShape(Arc::new(wheel_cylinder));
        let wheel = commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(wheel_mesh),
                material: materials.add(Color::rgb(0.03, 0.01, 0.03).into()),
                ..Default::default()
            })
            .insert(RigidBody::Dynamic)
            .insert_bundle(TransformBundle::from(
                Transform::from_translation(wheel_transform)
                    .with_rotation(Quat::from_axis_angle(Vec3::new(0., 1., 0.).normalize(), 0.)),
            ))
            // position: wheel_isometry.into(),
            .insert(Velocity::zero())
            .insert(Collider::from(wheel_shape))
            .insert(Friction::coefficient(100.))
            .insert(Restitution::coefficient(0.1))
            // .insert(ColliderMassProperties::Density(1.0))
            .insert(AdditionalMassProperties(MassProperties {
                local_center_of_mass: Vec3::new(0.0, 0.0, 0.0),
                mass: 15.0,
                principal_inertia: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            }))
            .insert(Wheel)
            .insert(ImpulseJoint::new(car, joint_builder))
            .insert(ExternalForce::default())
            .id();
        if i == 0 {
            commands
                .entity(wheel)
                .insert(FrontRightJoint)
                .insert(FrontJoint);
        } else if i == 1 {
            commands
                .entity(wheel)
                .insert(FrontLeftJoint)
                .insert(FrontJoint);
        } else if i == 2 {
            commands.entity(wheel).insert(BackJoint);
        } else if i == 3 {
            commands.entity(wheel).insert(BackJoint);
        }
    }
}
