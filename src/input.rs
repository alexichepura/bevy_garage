use crate::{
    car::*,
    config::*,
    db_client::DbClientResource,
    nn::{dqn::OBSERVATION_ZERO, dqn_bevy::DqnResource},
};
use bevy::prelude::*;
use bevy_rapier3d::render::DebugRenderContext;

#[tokio::main]
pub async fn keyboard_input_system(
    input: Res<Input<KeyCode>>,
    mut config: ResMut<Config>,
    mut cars: Query<(&mut Car, &Transform, With<HID>)>,
    mut commands: Commands,
    q_car: Query<Entity, With<Car>>,
    q_wheel: Query<Entity, With<Wheel>>,
    mut debug_ctx: ResMut<DebugRenderContext>,
    dbres: Res<DbClientResource>,
    mut dqn: ResMut<DqnResource>,
) {
    if input.just_pressed(KeyCode::B) {
        let rb = dbres.client.rb().find_many(vec![]).exec().await;
        match rb {
            Ok(rb) => {
                for r in rb.iter() {
                    let mut state = OBSERVATION_ZERO;
                    let mut next_state = OBSERVATION_ZERO;
                    for (i, s) in r.state.iter().enumerate() {
                        state[i] = *s as f32;
                    }
                    for (i, s) in r.next_state.iter().enumerate() {
                        next_state[i] = *s as f32;
                    }
                    dqn.rb.store(
                        state,
                        r.action as usize,
                        r.reward as f32,
                        next_state,
                        r.done,
                    );
                }
            }
            Err(err) => {
                dbg!(err);
            }
        };
    }
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
    if input.just_pressed(KeyCode::R) {
        debug_ctx.enabled = !debug_ctx.enabled;
    }
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
