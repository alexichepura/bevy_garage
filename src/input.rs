use crate::{camera::CameraConfig, config::*};
use bevy::prelude::*;
use bevy_garage_car::{
    car::{Car, HID},
    spawn::SpawnCarEvent,
};

pub fn input_system(
    input: Res<Input<KeyCode>>,
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    gamepads: Res<Gamepads>,
    mut config: ResMut<Config>,
    mut camera_config: ResMut<CameraConfig>,
    mut cars: Query<(&mut Car, Entity, &Transform, With<HID>)>,
    mut commands: Commands,
    mut car_spawn_events: EventWriter<SpawnCarEvent>,
    #[cfg(feature = "debug_lines")] mut debug_ctx: ResMut<
        bevy_rapier3d::render::DebugRenderContext,
    >,
    #[cfg(feature = "brain")] mut dqn: ResMut<bevy_garage_dqn::dqn_bevy::DqnResource>,
) {
    #[cfg(feature = "brain")]
    if input.just_pressed(KeyCode::N) {
        dqn.use_brain = !dqn.use_brain;
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

            car_spawn_events.send(SpawnCarEvent {
                is_hid: true,
                index: 0,
                init_meters: None,
            });
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
