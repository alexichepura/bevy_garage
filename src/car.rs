use bevy::prelude::*;
use bevy_rapier3d::{parry::shape::Cylinder, prelude::*};
use rapier3d::prelude::{Isometry, JointAxesMask, RigidBodyPosition, SharedShape};
use std::sync::Arc;

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
    let car_entity = commands
        .spawn()
        .insert(RigidBody::Dynamic)
        .insert_bundle(TransformBundle::from(
            Transform::from_translation(car_pos_transform).with_rotation(car_quat),
        ))
        .insert(Velocity::zero())
        .insert(Collider::cuboid(car_hl, car_hh, car_hw))
        .insert(ColliderMassProperties::Density(1.0))
        // .insert(MassProperties {
        //     local_center_of_mass: Vec3::new(0.0, -0.4, 0.0),
        //     mass: 1500.0,
        //     principal_inertia: Vec3::new(100.0, 100.0, 100.0),
        //     ..Default::default()
        // })
        // material: ColliderMaterial {
        //     friction: 0.001,
        //     restitution: 0.1,
        //     ..Default::default()
        // }
        // .with_children(|parent| {
        //     let mut tr: Transform = Transform {
        //         ..Default::default()
        //     };
        //     tr.translation = Vec3::new(0.0, -car_hh, 0.0);
        //     parent.spawn_bundle(PbrBundle {
        //         mesh: asset_server.load(car_graphics),
        //         material: materials.add(Color::rgb(0.3, 0.3, 0.8).into()),
        //         transform: tr,
        //         ..Default::default()
        //     });
        // })
        .insert(Car)
        .insert(Transform::default())
        .id();

    let shift = Vec3::new(car_hw + 0.1 + wheel_hw, -car_hh, car_hl);
    let car_anchors: [Vec3; 4] = [
        Vec3::new(shift.x, shift.y, shift.z),
        Vec3::new(-shift.x, shift.y, shift.z),
        Vec3::new(shift.x, shift.y, -shift.z),
        Vec3::new(-shift.x, shift.y, -shift.z),
    ];

    for i in 0..1 {
        let mask = JointAxesMask::X
            | JointAxesMask::Y
            | JointAxesMask::Z
            | JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z;

        let data = GenericJointBuilder::new(mask)
            .local_axis1(Vec3::X)
            .local_axis2(Vec3::Y)
            .local_anchor1(car_anchors[i])
            .local_anchor2(Vec3::new(0., 0., 0.).into());

        let wheel_transform = car_pos_transform + car_quat.mul_vec3(car_anchors[i]);
        let wheel_cylinder = Cylinder::new(wheel_hw, wheel_r);
        let wheel_mesh = bevy_mesh(wheel_cylinder.to_trimesh(100));
        let wheel_shape = SharedShape(Arc::new(wheel_cylinder));

        let wheel = commands
            .spawn()
            // .spawn_bundle(PbrBundle {
            //     mesh: meshes.add(wheel_mesh),
            //     material: materials.add(Color::rgb(0.1, 0.1, 0.3).into()),
            //     ..Default::default()
            // })
            .insert(RigidBody::Dynamic)
            .insert_bundle(TransformBundle::from(
                Transform::from_translation(wheel_transform), // .with_rotation(wheel_qvec) // TODO
            ))
            // position: wheel_isometry.into(),
            .insert(Velocity::zero())
            .insert(Collider::from(wheel_shape))
            .insert(ColliderMassProperties::Density(1.0))
            // .insert(MassProperties {
            //     local_center_of_mass: Vec3::new(0.0, 0.0, 0.0),
            //     mass: 15.0,
            //     principal_inertia: Vec3::new(1.0, 1.0, 1.0),
            //     ..Default::default()
            // })
            // material: ColliderMaterial {
            //     friction: 1.0,
            //     restitution: 0.1,
            //     ..Default::default()
            // }
            .insert(Wheel)
            .insert(Transform::default())
            .id();
        if i == 0 {
            commands
                .spawn()
                .insert(ImpulseJoint::new(car_entity, data))
                .insert(FrontRightJoint)
                .insert(FrontJoint);
        } else if i == 1 {
            commands
                .spawn()
                .insert(ImpulseJoint::new(car_entity, data))
                .insert(FrontLeftJoint)
                .insert(FrontJoint);
        } else if i == 2 {
            commands
                .spawn()
                .insert(ImpulseJoint::new(car_entity, data))
                .insert(BackJoint);
        } else if i == 3 {
            commands
                .spawn()
                .insert(ImpulseJoint::new(car_entity, data))
                .insert(BackJoint);
        }
    }
}
