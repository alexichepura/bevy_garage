use crate::car::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub fn car_change_detection(
    query: Query<(Entity, &Car), Changed<Car>>,
    mut wheels: Query<(&mut ExternalForce, &Transform, With<Wheel>)>,
    mut front: Query<(&mut MultibodyJoint, With<FrontJoint>)>,
) {
    for (_entity, car) in query.iter() {
        let torque: f32 = car.gas * 500.;
        for (mut forces, transform, _) in wheels.iter_mut() {
            forces.torque = (transform.rotation.mul_vec3(Vec3::new(0., torque, 0.))).into();
        }

        let axis = Vec3::new(1., 0., car.steering * 0.3);
        for (mut joint, _) in front.iter_mut() {
            joint.data.set_local_axis1(axis);
        }
    }
}

pub fn arrow_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut cars: Query<(&mut Car, &Transform, With<Car>)>,
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
        car.steering = -1.;
    }
    if keyboard_input.just_pressed(KeyCode::Right) {
        car.steering = 1.;
    }
    if keyboard_input.just_released(KeyCode::Left) {
        car.steering = 0.;
    }
    if keyboard_input.just_released(KeyCode::Right) {
        car.steering = 0.;
    }
}
