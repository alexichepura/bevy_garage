use crate::car::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn car_change_detection(
    query: Query<(Entity, &Car), Changed<Car>>,
    mut wheels: Query<(&mut ExternalForce, &Transform, With<Wheel>)>,
) {
    for (_entity, car) in query.iter() {
        let torque: f32 = car.gas * 500.;
        for (mut forces, transform, _) in wheels.iter_mut() {
            forces.torque = (transform.rotation.mul_vec3(Vec3::new(0., torque, 0.))).into();
        }
    }
}

pub fn arrow_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut cars: Query<(&mut Car, &Transform, With<Car>)>,
    mut front: Query<(&mut MultibodyJoint, With<FrontJoint>)>,
) {
    let (mut car, _transform, _car) = cars.single_mut();
    if keyboard_input.pressed(KeyCode::Up) {
        car.gas = 1.;
    }
    if keyboard_input.pressed(KeyCode::Down) {
        car.gas = -1.;
    }
    if keyboard_input.just_released(KeyCode::Up) {
        car.gas = 0.;
    }
    if keyboard_input.just_released(KeyCode::Down) {
        car.gas = 0.;
    }

    if keyboard_input.just_pressed(KeyCode::Left) {
        let axis = Vec3::new(1., 0., -0.3);
        for (mut joint, _) in front.iter_mut() {
            joint.data.set_local_axis1(axis);
        }
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        let axis = Vec3::new(1., 0., 0.3);
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
