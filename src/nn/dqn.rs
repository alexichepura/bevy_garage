use super::{gradient::get_sgd, params::*};
use crate::{
    car::*,
    config::*,
    nn::{dqn_bevy::*, util::*},
    track::*,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use dfdx::prelude::*;
use rand::Rng;

#[cfg(target_arch = "wasm32")]
pub type QNetwork = (
    (Linear<STATE_SIZE, HIDDEN_SIZE>, ReLU),
    (Linear<HIDDEN_SIZE, HIDDEN_SIZE>, ReLU),
    Linear<HIDDEN_SIZE, ACTIONS>,
);
#[cfg(not(target_arch = "wasm32"))]
pub type QNetwork = (
    (Linear<STATE_SIZE, HIDDEN_SIZE>, ReLU),
    (Linear<HIDDEN_SIZE, HIDDEN_SIZE>, ReLU),
    (Linear<HIDDEN_SIZE, HIDDEN_SIZE>, ReLU),
    Linear<HIDDEN_SIZE, ACTIONS>,
);
pub type QNetworkBuilt = <QNetwork as BuildOnDevice<AutoDevice, f32>>::Built;
pub type Observation = [f32; STATE_SIZE];

pub fn dqn_system(
    time: Res<Time>,
    mut dqn: ResMut<DqnResource>,
    mut cars_dqn: NonSendMut<CarsDqnResource>,
    dqn_tx: Res<DqnTx>,
    q_road: Query<&TrackRoad>,
    mut q_car: Query<(
        &mut Car,
        &Velocity,
        &Transform,
        Entity,
        Option<&HID>,
        &mut CarDqn,
    )>,
    q_colliding_entities: Query<&CollidingEntities, With<CollidingEntities>>,
    mut config: ResMut<Config>,
    mut commands: Commands,
    #[cfg(feature = "brain_api")] api: Res<super::api_client::ApiClient>,
) {
    let seconds = time.elapsed_seconds_f64();
    if dqn.respawn_in > 0. && seconds > dqn.respawn_in {
        let (transform, init_meters) = config.get_transform_random();
        spawn_car(
            &mut commands,
            &config.car_scene.as_ref().unwrap(),
            &config.wheel_scene.as_ref().unwrap(),
            dqn.respawn_is_hid,
            transform,
            dqn.respawn_index,
            init_meters,
            config.max_torque,
        );
        dqn.respawn_in = 0.;
        dqn.respawn_is_hid = false;
        dqn.respawn_index = 0;
        config.use_brain = true;
        return;
    };
    let should_act: bool = seconds > dqn.seconds;
    if should_act && config.use_brain {
        dqn.seconds = seconds + STEP_DURATION;
        dqn.step += 1;
    }

    for (mut car, v, tr, e, hid, mut car_dqn) in q_car.iter_mut() {
        let is_hid = hid.is_some();
        let mut crash: bool = false;

        let colliding_entities = q_colliding_entities.get(e);
        if let Ok(colliding_entities) = colliding_entities {
            for e in colliding_entities.iter() {
                let colliding_road = q_road.get(e);
                if !colliding_road.is_ok() {
                    crash = true;
                }
            }
        }

        let mut vel_angle = car.line_dir.angle_between(v.linvel);
        if vel_angle.is_nan() {
            vel_angle = 0.;
        }
        let pos_dir = tr.rotation.mul_vec3(Vec3::Z);
        let mut pos_angle = car.line_dir.angle_between(pos_dir);
        if pos_angle.is_nan() {
            pos_angle = 0.;
        }
        let vel_cos = vel_angle.cos();
        let pos_cos = pos_angle.cos();
        let mut d_from_center = car.line_pos - tr.translation;
        d_from_center.y = 0.;
        let d = d_from_center.length();
        let d_norm = d / 4.;

        let velocity = v.linvel.length();
        let velocity_norm = velocity / car_dqn.speed_limit;
        let shape_reward = || -> f32 {
            if crash {
                return -1.;
            }
            // https://team.inria.fr/rits/files/2018/02/ICRA18_EndToEndDriving_CameraReady.pdf
            // In [13] the reward is computed as a function of the difference of angle α between the road and car’s heading and the speed v.
            // R = v(cos α − d)
            let mut reward = velocity_norm * (vel_cos - d_norm);
            if vel_cos.is_sign_positive() && pos_cos.is_sign_negative() {
                reward = -reward;
            }
            if reward.is_nan() {
                return 0.;
            }
            return reward;
        };
        let reward = shape_reward();
        let mut obs: Observation = [0.; STATE_SIZE];
        for i in 0..STATE_SIZE {
            obs[i] = match i {
                0 => velocity_norm,
                1 => v.angvel.y,
                2 => d_norm,
                3 => vel_cos,
                4 => pos_cos,
                STATE_SIZE_BASE..=STATE_SIZE => car.sensor_inputs[i - STATE_SIZE_BASE],
                _ => panic!("unknown observation record"),
            };
        }

        let (prev_action, prev_obs) = (car_dqn.prev_action, car_dqn.prev_obs);
        if config.use_brain && (should_act || crash) && !prev_obs.iter().all(|&x| x == 0.) {
            dqn.rb.store(prev_obs, prev_action, reward, obs, crash);
            #[cfg(feature = "brain_api")]
            if dqn.rb.i % super::api_client::PERSIST_BATCH_SIZE == 0 {
                api.save_replay_buffer(super::api_client::get_replay_buffer_to_persist(&dqn.rb));
            }
        }

        let (action, _) = cars_dqn.act(obs, dqn.eps);
        if should_act && !crash {
            car_dqn.prev_obs = obs;
            car_dqn.prev_action = action;
            car_dqn.prev_reward = reward;
        }
        if !config.use_brain {
            return;
        }
        if crash {
            dqn.crashes += 1;
            dqn.respawn_in = seconds;
            dqn.respawn_is_hid = is_hid;
            dqn.respawn_index = car.index;
            commands.entity(e).despawn_recursive();
            car.despawn_wheels(&mut commands);
            config.use_brain = false;
        }
        if !config.use_brain || !should_act || crash {
            return;
        }

        if let Some(_hid) = hid {
            let rb_len = dqn.rb.len();
            if rb_len < BATCH_SIZE {
                log_action_reward(car_dqn.prev_action, reward);
            } else if !cars_dqn.processing {
                cars_dqn.processing = true;
                let mut rng = rand::thread_rng();
                let batch_indexes = [(); BATCH_SIZE].map(|_| rng.gen_range(0..dqn.rb.len()));
                let (s, a, r, sn, done) = dqn
                    .rb
                    .get_batch_tensors(batch_indexes, cars_dqn.device.clone());

                let tqn = cars_dqn.tqn.clone();
                let mut qn = cars_dqn.qn.clone();
                let gradients = cars_dqn.gradients.clone();
                let dqn_tx = dqn_tx.clone();

                #[cfg(target_arch = "wasm32")]
                {
                    let mut loss_string: String = String::from("");
                    let mut sgd = get_sgd(&qn);
                    for _i_epoch in 0..EPOCHS {
                        let next_q_values: Tensor2D<BATCH_SIZE, ACTIONS> = tqn.forward(sn.clone());
                        let max_next_q: Tensor1D<BATCH_SIZE> = next_q_values.max();
                        let target_q = (max_next_q * (-done.clone() + 1.0)) * 0.99 + r.clone();

                        // forward through model, computing gradients
                        let q_values = qn.forward(s.trace(gradients.clone()));
                        let action_qs = q_values.select(a.clone());

                        let loss = huber_loss(action_qs, target_q, 1.);
                        let loss_v = loss.array();
                        // run backprop
                        let gradients = loss.backward();
                        sgd.update(&mut qn, &gradients).expect("Unused params");
                        if _i_epoch % 10 == 0 {
                            loss_string.push_str(format!("{:.2} ", loss_v).as_str());
                        }
                    }
                    dqn_tx
                        .send(DqnX {
                            loss_string,
                            qn,
                            duration_string: "-".to_string(),
                        })
                        .unwrap();
                }

                #[cfg(not(target_arch = "wasm32"))]
                {
                    std::thread::spawn(move || {
                        let start = std::time::Instant::now();
                        let mut loss_string: String = String::from("");
                        let mut sgd = get_sgd(&qn);
                        for _i_epoch in 0..EPOCHS {
                            let next_q_values: Tensor2D<BATCH_SIZE, ACTIONS> =
                                tqn.forward(sn.clone());
                            let max_next_q: Tensor1D<BATCH_SIZE> = next_q_values.max();
                            let target_q = (max_next_q * (-done.clone() + 1.0)) * 0.99 + r.clone();

                            // forward through model, computing gradients
                            let q_values = qn.forward(s.trace(gradients.clone()));
                            let action_qs = q_values.select(a.clone());

                            let loss = huber_loss(action_qs, target_q, 1.);
                            let loss_v = loss.array();
                            // run backprop
                            let gradients = loss.backward();
                            sgd.update(&mut qn, &gradients).expect("Unused params");
                            if _i_epoch % 10 == 0 {
                                loss_string.push_str(format!("{:.2} ", loss_v).as_str());
                            }
                        }
                        let duration_string = start.elapsed().as_millis().to_string() + "ms";
                        dqn_tx
                            .send(DqnX {
                                loss_string,
                                qn,
                                duration_string,
                            })
                            .unwrap();
                    });
                }

                if dqn.step % SYNC_INTERVAL_STEPS == 0 && dqn.rb.len() > BATCH_SIZE * 2 {
                    dbg!("networks sync");
                    cars_dqn.tqn = cars_dqn.qn.clone();
                }
                dqn.eps = if dqn.eps <= dqn.min_eps {
                    dqn.min_eps
                } else {
                    dqn.eps - DECAY
                };
            }
        }

        let (gas, brake, left, right) = map_action_to_car(action);
        car.gas = gas;
        car.brake = brake;
        car.steering = -left + right;
    }
}
