use super::params::*;
use crate::{
    camera::CameraConfig,
    car::*,
    config::*,
    nn::{dqn_bevy::*, util::*},
    track::*,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use dfdx::prelude::*;
use rand::Rng;
use std::time::Instant;

pub type QNetwork = (
    (Linear<STATE_SIZE, HIDDEN_SIZE>, ReLU),
    (Linear<HIDDEN_SIZE, HIDDEN_SIZE>, ReLU),
    Linear<HIDDEN_SIZE, ACTIONS>,
);
pub type Observation = [f32; STATE_SIZE];

pub fn dqn_system(
    time: Res<Time>,
    mut dqn: NonSendMut<DqnResource>,
    mut cars_dqn: NonSendMut<CarDqnResources>,
    q_name: Query<&Name>,
    mut q_car: Query<(
        &mut Car,
        &Velocity,
        &Transform,
        &Children,
        Entity,
        Option<&HID>,
    )>,
    q_colliding_entities: Query<&CollidingEntities, With<CollidingEntities>>,
    config: Res<Config>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
    mut camera_config: ResMut<CameraConfig>,
) {
    let car_gl: Handle<Scene> = asset_server.load("car-race.glb#Scene0");
    let seconds = time.seconds_since_startup();
    let should_act: bool = seconds > dqn.seconds;
    if should_act {
        dqn.seconds = seconds + STEP_DURATION;
        dqn.step += 1;
    }

    for (mut car, v, tr, children, e, hid) in q_car.iter_mut() {
        let car_dqn = cars_dqn.cars.get_mut(&e).unwrap();
        let mut crash: bool = car_dqn.crash;
        if !crash {
            for &child in children.iter() {
                let colliding_entities = q_colliding_entities.get(child);
                if let Ok(colliding_entities) = colliding_entities {
                    for e in colliding_entities.iter() {
                        let colliding_entity = q_name.get(e).unwrap();
                        if !colliding_entity.contains(ASSET_ROAD) {
                            crash = true;
                        }
                    }
                }
            }
        }
        if crash {
            car_dqn.crash = true;
        }
        if !should_act {
            return;
        }
        let is_hid = hid.is_some();
        let mut vel_angle = car.line_dir.angle_between(v.linvel);
        if vel_angle.is_nan() {
            vel_angle = 0.;
        }
        let pos_dir = tr.rotation.mul_vec3(Vec3::Z);
        let pos_angle = car.line_dir.angle_between(pos_dir);
        let shape_reward = || -> f32 {
            // let (_p, colliding_entities) = q_colliding_entities.single();
            if crash {
                return -1.;
            }
            // https://team.inria.fr/rits/files/2018/02/ICRA18_EndToEndDriving_CameraReady.pdf
            // In [13] the reward is computed as a function of the difference of angle α between the road and car’s heading and the speed v.
            // R = v(cos α − d) // TODO d
            let mut reward = v.linvel.length() / SPEED_LIMIT_MPS * vel_angle.cos();
            if vel_angle.cos().is_sign_positive() && pos_angle.cos().is_sign_negative() {
                reward = -reward;
            }
            if reward.is_nan() {
                return 0.;
            }
            // if reward.is_sign_negative() {
            //     reward *= 2.; // TODO test negative reward multiplication
            // }
            return reward;
        };
        let reward = shape_reward();
        let mps = v.linvel.length();
        let kmh = mps / 1000. * 3600.;
        let mut obs: Observation = [0.; STATE_SIZE];
        for i in 0..obs.len() {
            obs[i] = match i {
                0 => kmh / 100.,
                1 => vel_angle.cos(),
                2 => pos_angle.cos(),
                _ => car.sensor_inputs[i - STATE_SIZE_BASE],
            };
        }

        if !config.use_brain {
            // println!(
            //     "dqn {reward:.2} {:.?}",
            //     obs.map(|o| { (o * 10.).round() / 10. })
            // );
            return;
        }

        let car_dqn = cars_dqn.cars.get(&e).unwrap();
        let (s, a, r, sn) = (car_dqn.prev_obs, car_dqn.prev_action, reward, obs);
        dqn.rb.store(s, a, r, sn, crash); // crash is done
                                          // cars_dqn.qn.0 .0 .0.bias;
        if crash {
            println!(
                "crash!!! e_{e:?} i_{:?} r_{reward:.2} m_{:.2}",
                car.index, car.meters
            );
            dqn.crashes += 1;
            commands.entity(e).despawn_recursive();
            cars_dqn.del_car(&e);
            car.despawn_wheels(&mut commands);
            let (transform, init_meters) = config.get_transform_random();
            let new_car_id = spawn_car(
                &mut commands,
                &mut meshes,
                &mut materials,
                &car_gl,
                is_hid,
                transform,
                car.index,
                init_meters,
                config.max_toi,
                config.max_torque,
            );
            cars_dqn.add_car(new_car_id);
            if camera_config.mode.not_none() && is_hid {
                camera_config.camera_follow = Some(new_car_id);
            }
            return;
        }

        let obs_state_tensor = Tensor1D::new(obs);
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0.0..1.0);
        let action: usize;
        let exploration = random_number < dqn.eps;
        if exploration {
            action = rng.gen_range(0..ACTIONS - 1);
        } else {
            // let car_dqn = cars_dqn.cars.get(&e).unwrap();
            let q_values = cars_dqn.qn.forward(obs_state_tensor.clone());
            let max_q_value = *q_values.clone().max_axis::<-1>().data();
            let some_action = q_values
                .clone()
                .data()
                .iter()
                .position(|q| *q >= max_q_value);
            if None == some_action {
                dbg!(q_values);
                panic!();
            } else {
                action = some_action.unwrap();
            }
        }

        if let Some(_hid) = hid {
            if dqn.rb.len() < BATCH_SIZE {
                log_action_reward(car_dqn.prev_action, reward);
            } else {
                let start = Instant::now();
                let batch_indexes = [(); BATCH_SIZE].map(|_| rng.gen_range(0..dqn.rb.len()));
                let (s, a, r, sn, done) = dqn.rb.get_batch_tensors(batch_indexes);
                let mut loss_string: String = String::from("");
                for _i_epoch in 0..EPOCHS {
                    let next_q_values: Tensor2D<BATCH_SIZE, ACTIONS> =
                        cars_dqn.tqn.forward(sn.clone());
                    let max_next_q: Tensor1D<BATCH_SIZE> = next_q_values.max_axis::<-1>();
                    let target_q = 0.99 * mul(max_next_q, &(1.0 - done.clone())) + &r;
                    // forward through model, computing gradients
                    let q_values: Tensor2D<BATCH_SIZE, ACTIONS, OwnedTape> =
                        cars_dqn.qn.forward(s.trace());
                    let action_qs: Tensor1D<BATCH_SIZE, OwnedTape> = q_values.select(&a);
                    let loss = huber_loss(action_qs, &target_q, 1.);
                    let loss_v = *loss.data();
                    // run backprop
                    let gradients = loss.backward();
                    dqn.sgd
                        .update(&mut cars_dqn.qn, gradients)
                        .expect("Unused params");
                    if _i_epoch % 5 == 0 {
                        loss_string.push_str(format!("{:.2} ", loss_v).as_str());
                    }
                }
                log_training(exploration, action, reward, &loss_string, start);
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

        let mut car_dqn = cars_dqn.cars.get_mut(&e).unwrap();
        car_dqn.prev_obs = obs;
        car_dqn.prev_action = action;
        car_dqn.prev_reward = reward;

        let (gas, brake, left, right) = map_action_to_car(action);
        car.gas = gas;
        car.brake = brake;
        car.steering = -left + right;
    }
}
