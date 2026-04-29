use bevy::prelude::*;
use bevy_garage_camera::CameraConfig;
use bevy_garage_car::{Car, CarRes, CarWheels, Player};
use bevy_garage_track::SpawnCarOnTrackEvent;

pub fn input_system(
    input: Res<ButtonInput<KeyCode>>,
    mut camera_config: ResMut<CameraConfig>,
    mut cars: Query<(&mut Car, &mut CarWheels, Entity, &Transform), With<Player>>,
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
    if input.just_pressed(KeyCode::KeyR) {
        debug_ctx.enabled = !debug_ctx.enabled;
        car_res.show_rays = debug_ctx.enabled;
    }
    for (mut car, mut wheels, e, _transform) in cars.iter_mut() {
        if input.just_pressed(KeyCode::Space) && input.pressed(KeyCode::ShiftLeft) {
            cmd.entity(e).despawn();
            wheels.despawn(&mut cmd);

            car_spawn_events.write(SpawnCarOnTrackEvent {
                player: true,
                index: 0,
                position: None,
            });
        }
        if input.pressed(KeyCode::ArrowUp) {
            car.gas = 1.;
        }
        if input.just_released(KeyCode::ArrowUp) {
            car.gas = 0.;
        }

        if input.pressed(KeyCode::ArrowDown) {
            car.brake = 1.;
        }
        if input.just_released(KeyCode::ArrowDown) {
            car.brake = 0.;
        }

        if input.pressed(KeyCode::ArrowLeft) {
            car.steering = -1.;
        }
        if input.pressed(KeyCode::ArrowRight) {
            car.steering = 1.;
        }
        if input.just_released(KeyCode::ArrowLeft) {
            car.steering = 0.;
        }
        if input.just_released(KeyCode::ArrowRight) {
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
