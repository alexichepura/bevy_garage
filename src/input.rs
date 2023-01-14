use crate::{car::*, config::*, gamepad::GamepadLobby};
use bevy::prelude::*;

pub fn keyboard_input_system(
    input: Res<Input<KeyCode>>,
    mut config: ResMut<Config>,
    mut cars: Query<(&mut Car, &Transform, With<HID>)>,
    mut commands: Commands,
    q_car: Query<Entity, With<Car>>,
    q_wheel: Query<Entity, With<Wheel>>,
    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "ios")))] mut debug_ctx: ResMut<
        bevy_rapier3d::render::DebugRenderContext,
    >,
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
    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "ios")))]
    if input.just_pressed(KeyCode::R) {
        debug_ctx.enabled = !debug_ctx.enabled;
        config.show_rays = debug_ctx.enabled;
    }
    for (mut car, _transform, _hid) in cars.iter_mut() {
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

pub fn gamepad_input_system(
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    lobby: Res<GamepadLobby>,
    mut cars: Query<(&mut Car, &Transform, With<HID>)>,
) {
    for (mut car, _transform, _hid) in cars.iter_mut() {
        for gamepad in lobby.gamepads.iter().cloned() {
            if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
                car.gas = 1.;
            } else if buttons.just_released(GamepadButton::new(gamepad, GamepadButtonType::South)) {
                car.gas = 0.;
            }
            if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::North)) {
                car.brake = 1.;
            } else if buttons.just_released(GamepadButton::new(gamepad, GamepadButtonType::North)) {
                car.brake = 0.;
            }
            let left_stick_x = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
                .unwrap();
            // dbg!(left_stick_x);
            car.steering = left_stick_x;
        }
    }
}
