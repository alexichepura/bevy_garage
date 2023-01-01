use crate::{car::*, config::*};
use bevy::prelude::*;
// use bevy_rapier3d::render::DebugRenderContext;

pub fn keyboard_input_system(
    input: Res<Input<KeyCode>>,
    mut config: ResMut<Config>,
    mut cars: Query<(&mut Car, &Transform, With<HID>)>,
    mut commands: Commands,
    q_car: Query<Entity, With<Car>>,
    q_wheel: Query<Entity, With<Wheel>>,
    // mut debug_ctx: ResMut<DebugRenderContext>,
) {
    if input.just_pressed(KeyCode::N) {
        config.use_brain = !config.use_brain;
    }
    if input.just_pressed(KeyCode::Space) {
        for e in q_wheel.iter() {
            commands.entity(e).despawn_recursive();
        }
        for e in q_car.iter() {
            commands.entity(e).despawn_recursive();
        }
    }
    // if input.just_pressed(KeyCode::R) {
    //     debug_ctx.enabled = !debug_ctx.enabled;
    // }
    for (mut car, _transform, _car) in cars.iter_mut() {
        if input.pressed(KeyCode::Up) {
            car.gas = 1.;
        }
        if input.just_released(KeyCode::Up) {
            car.gas = 0.;
        }

        if input.pressed(KeyCode::Down) {
            car.brake = 1.;
        }
        if input.just_released(KeyCode::Down) {
            car.brake = 0.;
        }

        if input.just_pressed(KeyCode::Left) {
            car.steering = -1.;
        }
        if input.just_pressed(KeyCode::Right) {
            car.steering = 1.;
        }
        if input.just_released(KeyCode::Left) {
            car.steering = 0.;
        }
        if input.just_released(KeyCode::Right) {
            car.steering = 0.;
        }
    }
}
