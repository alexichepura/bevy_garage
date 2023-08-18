use bevy::prelude::*;
use bevy_garage_camera::CameraConfig;
use bevy_garage_car::{Car, CarRes, CarWheels, Player};
use bevy_garage_track::SpawnCarOnTrackEvent;

pub fn input_system(
    input: Res<Input<KeyCode>>,
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    gamepads: Res<Gamepads>,
    mut camera_config: ResMut<CameraConfig>,
    mut cars: Query<(&mut Car, &mut CarWheels, Entity, &Transform, With<Player>)>,
    mut cmd: Commands,
    mut car_spawn_events: EventWriter<SpawnCarOnTrackEvent>,
    mut debug_ctx: ResMut<bevy_rapier3d::render::DebugRenderContext>,
    mut car_res: ResMut<CarRes>,
    #[cfg(feature = "nn")] mut dqn: ResMut<bevy_garage_nn::DqnResource>,
) {
    #[cfg(feature = "nn")]
    if input.just_pressed(KeyCode::N) {
        dqn.use_nn = !dqn.use_nn;
    }
    if input.just_pressed(KeyCode::R) {
        debug_ctx.enabled = !debug_ctx.enabled;
        car_res.show_rays = debug_ctx.enabled;
    }
    for (mut car, mut wheels, e, _transform, _hid) in cars.iter_mut() {
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
            if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::LeftTrigger)) {
                debug_ctx.enabled = !debug_ctx.enabled;
                car_res.show_rays = debug_ctx.enabled;
            }
            if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::RightTrigger)) {
                camera_config.next_view();
            }
        }

        if input.just_pressed(KeyCode::Space) && input.pressed(KeyCode::ShiftLeft) {
            cmd.entity(e).despawn_recursive();
            wheels.despawn(&mut cmd);

            car_spawn_events.send(SpawnCarOnTrackEvent {
                player: true,
                index: 0,
                position: None,
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
