use crate::car::FrontLeftJoint;
use crate::car::FrontRightJoint;
use crate::car::Wheel;
use crate::GamepadLobby;
use bevy::ecs::system::Query;
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy::input::gamepad::{GamepadAxis, GamepadAxisType, GamepadButton, GamepadButtonType};
use bevy::input::keyboard::KeyCode;
use bevy::input::Axis;
use bevy::input::Input;
use bevy::math::Vec3;
use bevy::prelude::With;
use bevy::transform::components::Transform;
use bevy_rapier3d::physics::JointHandleComponent;
use bevy_rapier3d::prelude::ImpulseJoint;
use bevy_rapier3d::prelude::RigidBodyForcesComponent;
use bevy_rapier3d::prelude::RigidBodyMassPropsComponent;
use bevy_rapier3d::prelude::RigidBodyVelocityComponent;
use bevy_rapier3d::prelude::{ImpulseJointSet, Real, Vector};
use nalgebra::Unit;

pub fn arrow_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut wheels: Query<(
        &mut RigidBodyVelocityComponent,
        &mut RigidBodyForcesComponent,
        &Transform,
        &RigidBodyMassPropsComponent,
        With<Wheel>,
    )>,
    mut front_right: Query<(&JointHandleComponent, With<FrontRightJoint>)>,
    mut front_left: Query<(&JointHandleComponent, With<FrontLeftJoint>)>,
    mut joints: ResMut<ImpulseJointSet>,
) {
    let torque: f32 = 1000.;
    if keyboard_input.pressed(KeyCode::Up) {
        for (mut _velocity, mut forces, transform, _mprops, _wheel) in wheels.iter_mut() {
            forces.torque = (transform.rotation.mul_vec3(Vec3::new(torque, 0., 0.))).into();
        }
    }
    if keyboard_input.pressed(KeyCode::Down) {
        for (mut _velocity, mut forces, transform, _mprops, _wheel) in wheels.iter_mut() {
            forces.torque = (transform.rotation.mul_vec3(Vec3::new(-torque, 0., 0.))).into();
        }
    }
    if keyboard_input.just_pressed(KeyCode::Left) {
        let wax_rot: Unit<Vector<Real>> = Unit::new_normalize(Vec3::new(1., 0., -0.3).into());
        for (jhc, _) in front_right.iter_mut() {
            joint_wax(joints.get_mut(jhc.handle()), wax_rot);
        }
        for (jhc, _) in front_left.iter_mut() {
            joint_wax(joints.get_mut(jhc.handle()), wax_rot);
        }
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        let wax_rot: Unit<Vector<Real>> = Unit::new_normalize(Vec3::new(1., 0., 0.3).into());
        for (jhc, _) in front_right.iter_mut() {
            joint_wax(joints.get_mut(jhc.handle()), wax_rot);
        }
        for (jhc, _) in front_left.iter_mut() {
            joint_wax(joints.get_mut(jhc.handle()), wax_rot);
        }
    }
    if keyboard_input.just_released(KeyCode::Left) {
        let wax_rot: Unit<Vector<Real>> = Unit::new_normalize(Vec3::new(1., 0., 0.).into());
        for (jhc, _) in front_right.iter_mut() {
            joint_wax(joints.get_mut(jhc.handle()), wax_rot);
        }
        for (jhc, _) in front_left.iter_mut() {
            joint_wax(joints.get_mut(jhc.handle()), wax_rot);
        }
    }
    if keyboard_input.just_released(KeyCode::Right) {
        let wax_rot: Unit<Vector<Real>> = Unit::new_normalize(Vec3::new(1., 0., 0.).into());
        for (jhc, _) in front_right.iter_mut() {
            joint_wax(joints.get_mut(jhc.handle()), wax_rot);
        }
        for (jhc, _) in front_left.iter_mut() {
            joint_wax(joints.get_mut(jhc.handle()), wax_rot);
        }
    }
}

pub fn joint_wax(joint: Option<&mut ImpulseJoint>, wax: Unit<Vector<Real>>) {
    if let Some(mut joint) = joint {
        joint.data = joint.data.local_axis1(wax);
        // let current_revolute_joint = joint.params.as_revolute_joint();
        // if let Some(j) = current_revolute_joint {
        // let joint_new =
        //     RevoluteJoint::new(j.local_anchor1, wax, j.local_anchor2, j.local_axis2);
        // joint.params = JointParams::RevoluteJoint(joint_new);
        // }
    }
}

pub fn gamepad_input_system(
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    lobby: Res<GamepadLobby>,
    mut wheels: Query<(
        &mut RigidBodyVelocityComponent,
        &mut RigidBodyForcesComponent,
        &Transform,
        &RigidBodyMassPropsComponent,
        With<Wheel>,
    )>,
    mut front_right_query: Query<(&JointHandleComponent, With<FrontRightJoint>)>,
    mut front_left: Query<(&JointHandleComponent, With<FrontLeftJoint>)>,
    mut joints: ResMut<ImpulseJointSet>,
) {
    for gamepad in lobby.gamepads.iter().cloned() {
        let axis_lx = GamepadAxis(gamepad, GamepadAxisType::LeftStickX);
        if let Some(x) = axes.get(axis_lx) {
            let wax_rot: Unit<Vector<Real>> =
                Unit::new_normalize(Vec3::new(-x / 2.0, 0.0, 1.0).into());
            for (jhc, _) in front_right_query.iter_mut() {
                joint_wax(joints.get_mut(jhc.handle()), wax_rot);
            }
            for (jhc, _) in front_left.iter_mut() {
                joint_wax(joints.get_mut(jhc.handle()), wax_rot);
            }
        }

        let north = GamepadButton(gamepad, GamepadButtonType::North);
        if buttons.pressed(north) {
            for (mut _velocity, mut forces, transform, _mprops, _wheel) in wheels.iter_mut() {
                forces.torque = (transform.rotation.mul_vec3(Vec3::new(0., 0., -10.))).into();
            }
        }
        let south = GamepadButton(gamepad, GamepadButtonType::South);
        if buttons.pressed(south) {
            for (mut _velocity, mut forces, transform, _mprops, _wheel) in wheels.iter_mut() {
                forces.torque = (transform.rotation.mul_vec3(Vec3::new(0., 0., 10.))).into();
            }
        }
    }
}
