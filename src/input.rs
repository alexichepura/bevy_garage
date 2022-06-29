use crate::car::*;
use bevy::ecs::system::Query;
use bevy::ecs::system::Res;
use bevy::ecs::system::ResMut;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy::transform::components::Transform;
use bevy_rapier3d::prelude::*;

pub fn arrow_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut wheels: Query<(&mut ExternalForce, &Transform, With<Wheel>)>,
    mut front: Query<(&mut ImpulseJoint, With<FrontJoint>)>,
) {
    // TORQUE
    let torque: f32 = 100.;
    if keyboard_input.just_pressed(KeyCode::Up) {
        for (mut forces, transform, _) in wheels.iter_mut() {
            forces.torque = (transform.rotation.mul_vec3(Vec3::new(0., torque, 0.))).into();
        }
    }
    if keyboard_input.just_pressed(KeyCode::Down) {
        for (mut forces, transform, _) in wheels.iter_mut() {
            forces.torque = (transform.rotation.mul_vec3(Vec3::new(0., -torque, 0.))).into();
        }
    }
    if keyboard_input.just_released(KeyCode::Up) {
        for (mut forces, _, _) in wheels.iter_mut() {
            forces.torque = Vec3::ZERO;
        }
    }
    if keyboard_input.just_released(KeyCode::Down) {
        for (mut forces, _, _) in wheels.iter_mut() {
            forces.torque = Vec3::ZERO;
        }
    }

    // STEERING
    if keyboard_input.just_pressed(KeyCode::Left) {
        let axis = Vec3::new(1., 0., -0.3).normalize();
        for (mut joint, _) in front.iter_mut() {
            joint.data.set_local_axis1(axis);
        }
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        let axis = Vec3::new(1., 0., 0.3).normalize();
        for (mut joint, _) in front.iter_mut() {
            joint.data.set_local_axis1(axis);
        }
    }
    if keyboard_input.just_released(KeyCode::Left) {
        let axis = Vec3::new(1., 0., 0.0).normalize();
        for (mut joint, _) in front.iter_mut() {
            joint.data.set_local_axis1(axis);
        }
    }
    if keyboard_input.just_released(KeyCode::Right) {
        let axis = Vec3::new(1., 0., 0.0).normalize();
        for (mut joint, _) in front.iter_mut() {
            joint.data.set_local_axis1(axis);
        }
    }
}

// pub fn gamepad_input_system(
//     buttons: Res<Input<GamepadButton>>,
//     axes: Res<Axis<GamepadAxis>>,
//     lobby: Res<GamepadLobby>,
//     mut wheels: Query<(&mut ExternalForce, &Transform, With<Wheel>)>,
//     mut front_right: Query<(&RapierImpulseJointHandle, With<FrontRightJoint>)>,
//     mut front_left: Query<(&RapierImpulseJointHandle, With<FrontLeftJoint>)>,
//     mut joints: ResMut<ImpulseJointSet>,
// ) {
//     let (jh_front_left, _) = front_left.get_single_mut().unwrap();
//     let (jh_front_right, _) = front_right.get_single_mut().unwrap();
//     let mut joint_left = joints.get_mut(jh_front_left.0).unwrap().data;
//     let mut joint_right = joints.get_mut(jh_front_right.0).unwrap().data;

//     for gamepad in lobby.gamepads.iter().cloned() {
//         let axis_lx = GamepadAxis(gamepad, GamepadAxisType::LeftStickX);
//         if let Some(x) = axes.get(axis_lx) {
//             let wheel_axis: Unit<Vector<Real>> =
//                 Unit::new_normalize(Vec3::new(-x / 2.0, 0.0, 1.0).into());

//             joint_left.set_local_axis1(wheel_axis);
//             joint_right.set_local_axis1(wheel_axis);
//         }

//         let north = GamepadButton(gamepad, GamepadButtonType::North);
//         if buttons.pressed(north) {
//             for (mut forces, transform, _) in wheels.iter_mut() {
//                 forces.torque = (transform.rotation.mul_vec3(Vec3::new(0., 0., -10.))).into();
//             }
//         }
//         let south = GamepadButton(gamepad, GamepadButtonType::South);
//         if buttons.pressed(south) {
//             for (mut forces, transform, _) in wheels.iter_mut() {
//                 forces.torque = (transform.rotation.mul_vec3(Vec3::new(0., 0., 10.))).into();
//             }
//         }
//     }
// }
