use crate::car::*;
use bevy::prelude::*;

pub fn arrow_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut cars: Query<(&mut Car, &Transform, With<HID>)>,
) {
    for (mut car, _transform, _car) in cars.iter_mut() {
        if keyboard_input.pressed(KeyCode::Up) {
            car.gas = 1.;
        }
        if keyboard_input.just_released(KeyCode::Up) {
            car.gas = 0.;
        }

        if keyboard_input.pressed(KeyCode::Down) {
            car.brake = 1.;
        }
        if keyboard_input.just_released(KeyCode::Down) {
            car.brake = 0.;
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
}
