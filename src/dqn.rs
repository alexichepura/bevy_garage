use crate::{
    car::*,
    config::Config,
    nn::{action::*, dqn_bevy::*, log::*, replay::*},
    progress::*,
    track::*,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use dfdx::prelude::*;
use rand::Rng;
use std::time::Instant;

const EPOCHS: usize = 30;
const DECAY: f32 = 0.0001;
pub const SYNC_INTERVAL_STEPS: i32 = 100;
const STEP_DURATION: f64 = 1. / 1.;

const STATE_SIZE_BASE: usize = 3;
pub const STATE_SIZE: usize = STATE_SIZE_BASE + SENSOR_COUNT;
const ACTION_SIZE: usize = 8;
const HIDDEN_SIZE: usize = 16;
pub type QNetwork = (
    (Linear<STATE_SIZE, HIDDEN_SIZE>, ReLU),
    (Linear<HIDDEN_SIZE, HIDDEN_SIZE>, ReLU),
    Linear<HIDDEN_SIZE, ACTION_SIZE>,
);
pub type Observation = [f32; STATE_SIZE];

pub fn dqn_system(
    time: Res<Time>,
    mut dqn: NonSendMut<DqnResource>,
    q_name: Query<&Name>,
    mut q_car: Query<(&mut Car, &Velocity, &CarProgress, &mut CarDqn, &Transform), With<CarDqn>>,
    q_colliding_entities: Query<(&Parent, &CollidingEntities), With<CollidingEntities>>,
    config: Res<Config>,
) {
    let seconds = time.seconds_since_startup();
    if seconds > dqn.seconds {
        dqn.seconds = seconds + STEP_DURATION;
        dqn.step += 1;
    } else {
        return;
    }

    let (mut car, v, progress, mut car_dqn, tr) = q_car.single_mut();
    let mut vel_angle = progress.line_dir.angle_between(v.linvel);
    if vel_angle.is_nan() {
        vel_angle = 0.;
    }
    let pos_dir = tr.rotation.mul_vec3(Vec3::Z);
    let pos_angle = progress.line_dir.angle_between(pos_dir);

    let shape_reward = || -> f32 {
        let (_p, colliding_entities) = q_colliding_entities.single();
        let mut crashed: bool = false;
        for e in colliding_entities.iter() {
            let colliding_entity = q_name.get(e).unwrap();
            if !colliding_entity.contains(ASSET_ROAD) {
                crashed = true;
            }
        }
        if crashed {
            return -1.;
        }
        // https://team.inria.fr/rits/files/2018/02/ICRA18_EndToEndDriving_CameraReady.pdf
        // In [13] the reward is computed as a function of the difference of angle α between the road and car’s heading and the speed v.
        // R = v(cos α − d) // TODO d
        let mut reward = 0.01 * v.linvel.length() * vel_angle.cos();
        if vel_angle.cos().is_sign_positive() && pos_angle.cos().is_sign_negative() {
            reward = -reward;
        }
        if reward.is_nan() {
            return 0.;
        }
        return reward;
    };
    let reward = shape_reward();

    let mps = v.linvel.length();
    let kmh = mps / 1000. * 3600.;
    let mut obs: Observation = [0.; STATE_SIZE];
    for i in 0..obs.len() {
        obs[i] = match i {
            0 => kmh / 200.,
            1 => vel_angle.cos(),
            2 => pos_angle.cos(),
            _ => car.sensor_inputs[i - STATE_SIZE_BASE],
        };
    }
    if !config.use_brain {
        println!(
            "dqn {reward:.2} {:.?}",
            obs.map(|o| { (o * 10.).round() / 10. })
        );
        return;
    }
    let obs_state_tensor = Tensor1D::new(obs);
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0.0..1.0);
    let action: usize;
    let exploration = random_number < dqn.eps;
    if exploration {
        action = rng.gen_range(0..ACTION_SIZE - 1);
    } else {
        let q_values = dqn.qn.forward(obs_state_tensor.clone());
        let max_q_value = *q_values.clone().max_last_dim().data();
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

    if dqn.rb.len() < BATCH_SIZE {
        log_action_reward(action, reward);
    } else {
        if dqn.rb.len() > BATCH_SIZE_2 {
            let start = Instant::now();
            let batch_indexes = [(); BATCH_SIZE_2].map(|_| rng.gen_range(0..dqn.rb.len()));
            let (s, a, r, sn, done) = dqn.rb.get_batch_2_tensors(batch_indexes);
            let mut loss_string: String = String::from("");
            for _i_epoch in 0..EPOCHS {
                let next_q_values: Tensor2D<BATCH_SIZE_2, ACTION_SIZE> =
                    dqn.tqn.forward(sn.clone());
                let max_next_q: Tensor1D<BATCH_SIZE_2> = next_q_values.max_last_dim();
                let target_q = 0.99 * mul(max_next_q, &(1.0 - done.clone())) + &r;
                let q_values = dqn.qn.forward(s.trace());
                let loss = mse_loss(q_values.gather_last_dim(&a), &target_q);
                let loss_v = *loss.data();
                let gradients = loss.backward();
                dqn.sgd_update(gradients);
                if _i_epoch % 5 == 0 {
                    loss_string.push_str(format!("{:.2} ", loss_v).as_str());
                }
            }
            log_training(exploration, action, reward, &loss_string, start);
        } else {
            let start = Instant::now();
            let batch_indexes = [(); BATCH_SIZE].map(|_| rng.gen_range(0..dqn.rb.len()));
            let (s, a, r, sn, done) = dqn.rb.get_batch_tensors(batch_indexes);
            let mut loss_string: String = String::from("");
            for _i_epoch in 0..EPOCHS {
                let next_q_values: Tensor2D<BATCH_SIZE, ACTION_SIZE> = dqn.tqn.forward(sn.clone());
                let max_next_q: Tensor1D<BATCH_SIZE> = next_q_values.max_last_dim();
                let target_q = 0.99 * mul(max_next_q, &(1.0 - done.clone())) + &r;
                let q_values = dqn.qn.forward(s.trace());
                let loss = mse_loss(q_values.gather_last_dim(&a), &target_q);
                let loss_v = *loss.data();
                let gradients = loss.backward();
                dqn.sgd_update(gradients);
                if _i_epoch % 5 == 0 {
                    loss_string.push_str(format!("{:.2} ", loss_v).as_str());
                }
            }
            log_training(exploration, action, reward, &loss_string, start);
        }
        if dqn.step % SYNC_INTERVAL_STEPS as i32 == 0 && dqn.rb.len() > BATCH_SIZE * 2 {
            dbg!("networks sync");
            dqn.tqn = dqn.qn.clone();
        }
        dqn.eps = if dqn.eps <= dqn.min_eps {
            dqn.min_eps
        } else {
            dqn.eps - DECAY
        };
    }
    dqn.rb
        .store(car_dqn.prev_obs, car_dqn.prev_action, reward, obs);
    car_dqn.prev_obs = obs;
    car_dqn.prev_action = action;
    car_dqn.prev_reward = reward;
    let (gas, brake, left, right) = map_action_to_car(action);
    car.gas = gas;
    car.brake = brake;
    car.steering = -left + right;
}
