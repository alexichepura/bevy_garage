use crate::{camera::CameraConfig, car::*, config::*};
use bevy::prelude::*;

pub fn input_system(
    input: Res<Input<KeyCode>>,
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    gamepads: Res<Gamepads>,
    mut config: ResMut<Config>,
    mut camera_config: ResMut<CameraConfig>,
    mut cars: Query<(&mut Car, Entity, &Transform, With<HID>)>,
    mut commands: Commands,
    #[cfg(feature = "debug_lines")] mut debug_ctx: ResMut<
        bevy_rapier3d::render::DebugRenderContext,
    >,
) {
    if input.just_pressed(KeyCode::N) {
        config.use_brain = !config.use_brain;
    }
    #[cfg(feature = "debug_lines")]
    if input.just_pressed(KeyCode::R) {
        debug_ctx.enabled = !debug_ctx.enabled;
        config.show_rays = debug_ctx.enabled;
    }
    for (mut car, e, _transform, _hid) in cars.iter_mut() {
        for gamepad in gamepads.iter() {
            let left_stick_x = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
                .unwrap();
            // dbg!(left_stick_x);
            car.steering = left_stick_x;

            let right_stick_y = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::RightStickY))
                .unwrap();
            // dbg!(right_stick_y);
            if right_stick_y < 0. {
                car.brake = -right_stick_y / 0.75;
                car.gas = 0.;
            } else {
                car.gas = right_stick_y / 0.75;
                car.brake = 0.;
            }

            if buttons.pressed(GamepadButton::new(gamepad, GamepadButtonType::North)) {
                car.gas = 1.;
            } else if buttons.just_released(GamepadButton::new(gamepad, GamepadButtonType::North)) {
                car.gas = 0.;
            }
            if buttons.pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
                car.brake = 1.;
            } else if buttons.just_released(GamepadButton::new(gamepad, GamepadButtonType::South)) {
                car.brake = 0.;
            }
            #[cfg(feature = "debug_lines")]
            if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger)) {
                debug_ctx.enabled = !debug_ctx.enabled;
                config.show_rays = debug_ctx.enabled;
            }
            if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::RightTrigger)) {
                camera_config.next_view();
            }
        }

        if input.just_pressed(KeyCode::Space) && input.pressed(KeyCode::LShift) {
            commands.entity(e).despawn_recursive();
            car.despawn_wheels(&mut commands);

            // let (transform, init_meters) = config.get_transform_random();
            let init_meters = 5088.;
            let (translate, quat) = config.get_transform_by_meter(init_meters);
            let transform = Transform::from_translation(translate).with_rotation(quat);
            spawn_car(
                &mut commands,
                &config.car_scene.as_ref().unwrap(),
                &config.wheel_scene.as_ref().unwrap(),
                true,
                transform,
                0,
                init_meters,
                config.max_torque,
            );
        }
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

        if input.pressed(KeyCode::Left) {
            car.steering = -1.;
        }
        if input.pressed(KeyCode::Right) {
            car.steering = 1.;
        }
        if input.just_released(KeyCode::Left) {
            car.steering = 0.;
        }
        if input.just_released(KeyCode::Right) {
            car.steering = 0.;
        }
        // if input.just_released(KeyCode::Space) {
        //     car.gas = 0.;
        //     car.brake = 0.;
        // }
        // if input.pressed(KeyCode::Space) {
        //     car.gas = 0.;
        //     car.brake = 1.;
        // }
    }
}
