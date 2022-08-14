use std::ops::Mul;

use crate::{
    car::{Car, SENSOR_COUNT},
    progress::CarProgress,
    track::ASSET_ROAD,
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use dfdx::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

const DECAY: f32 = 0.0001;
const SYNC_INTERVAL_STEPS: i32 = 200;
const STEP_DURATION: f64 = 0.1;
const BATCH_SIZE: usize = 256;
// const MIN_REPLAY_SIZE: usize = 1000;
const BUFFER_SIZE: usize = 50_000;
const STATE_SIZE: usize = SENSOR_COUNT + 2;
const ACTION_SIZE: usize = 8;
type QNetwork = (
    (Linear<STATE_SIZE, 32>, ReLU),
    (Linear<32, 32>, ReLU),
    Linear<32, ACTION_SIZE>,
);
type Observation = [f32; STATE_SIZE];

pub struct ReplayBuffer {
    pub state: Vec<Observation>,
    pub action: Vec<usize>,
    pub reward: Vec<f32>,
    pub next_state: Vec<Observation>,
    pub i: usize,
}
type StateTuple = (Observation, usize, f32, Observation);
impl ReplayBuffer {
    pub fn new() -> Self {
        Self {
            state: Vec::new(),
            action: Vec::new(),
            reward: Vec::new(),
            next_state: Vec::new(),
            i: 0,
        }
    }
    pub fn len(&self) -> usize {
        self.state.len()
    }
    pub fn get_batch(&self, sample_indexes: [usize; BATCH_SIZE]) -> [StateTuple; BATCH_SIZE] {
        sample_indexes.map(|i| {
            (
                self.state[i],
                self.action[i],
                self.reward[i],
                self.next_state[i],
            )
        })
    }
    pub fn store(
        &mut self,
        state: Observation,
        action: usize,
        reward: f32,
        next_state: Observation,
    ) {
        let i = self.i % BUFFER_SIZE;
        if self.len() < BUFFER_SIZE {
            self.state.push(state);
            self.action.push(action);
            self.reward.push(reward);
            self.next_state.push(next_state);
        } else {
            self.state[i] = state;
            self.action[i] = action;
            self.reward[i] = reward;
            self.next_state[i] = next_state;
        }
        self.i += 1;
    }
}
pub struct DqnResource {
    pub seconds: f64,
    pub step: i32,
    pub qn: QNetwork,
    pub tqn: QNetwork,
    pub rb: ReplayBuffer,
    pub eps: f32,
    pub max_eps: f32,
    pub min_eps: f32,
    pub done: f32,
}
impl DqnResource {
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(0);
        let mut qn = QNetwork::default();
        qn.reset_params(&mut rng);
        Self {
            seconds: 0.,
            step: 0,
            qn: qn.clone(),
            tqn: qn.clone(),
            rb: ReplayBuffer::new(),
            eps: 1.,
            max_eps: 1.,
            min_eps: 0.01,
            done: 0.,
        }
    }
}
pub struct SgdResource {
    pub sgd: Sgd<QNetwork>,
}
pub fn dqn_start_system(world: &mut World) {
    world.insert_non_send_resource(DqnResource::new());
    world.insert_non_send_resource(SgdResource {
        sgd: Sgd::new(SgdConfig {
            lr: 1e-1,
            momentum: Some(Momentum::Nesterov(0.9)),
        }),
    });
}

#[derive(Component, Debug, Default)]
pub struct CarDqn {
    pub prev_obs: Observation,
    pub prev_action: usize,
    pub prev_reward: f32,
    pub prev_progress: f32,
}

pub fn dqn_system(
    time: Res<Time>,
    mut dqn: NonSendMut<DqnResource>,
    mut sgd: NonSendMut<SgdResource>,
    q_name: Query<&Name>,
    mut q_car: Query<(&mut Car, &Velocity, &CarProgress, &mut CarDqn), With<CarDqn>>,
    mut q_colliding_entities: Query<(&Parent, &CollidingEntities), With<CollidingEntities>>,
) {
    let seconds = time.seconds_since_startup();
    if seconds > dqn.seconds {
        dqn.seconds = seconds + STEP_DURATION;
        if dqn.rb.len() > BATCH_SIZE {
            dqn.step += 1;
        }
    } else {
        return;
    }

    let (mut car, v, progress, mut car_dqn) = q_car.single_mut();
    let (_p, colliding_entities) = q_colliding_entities.single_mut();
    let mut crashed: bool = false;
    for e in colliding_entities.iter() {
        let colliding_entity = q_name.get(e).unwrap();
        if !colliding_entity.contains(ASSET_ROAD) {
            crashed = true;
        }
    }

    let obs: Observation = [
        // TODO map
        car.sensor_inputs[0],
        car.sensor_inputs[1],
        car.sensor_inputs[2],
        car.sensor_inputs[3],
        car.sensor_inputs[4],
        car.sensor_inputs[5],
        car.sensor_inputs[6],
        car.sensor_inputs[7],
        car.sensor_inputs[8],
        car.sensor_inputs[9],
        car.sensor_inputs[10],
        car.sensor_inputs[11],
        car.sensor_inputs[12],
        car.sensor_inputs[13],
        car.sensor_inputs[14],
        car.sensor_inputs[15],
        v.linvel.length(),
        progress.meters,
    ];
    let obs_state_tensor = Tensor1D::new(obs);
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0.0..1.0);
    let progress_delta = progress.meters - car_dqn.prev_progress;
    let mut reward: f32 = if crashed { -1. } else { progress_delta };
    if reward < -2. {
        // stabilise things, (issue: progress_delta is too big)
        reward = -2.
    }
    if reward > 2. {
        // stabilise things, (issue: progress_delta is too big)
        reward = 2.
    }
    let action: usize;
    let use_random = random_number < dqn.eps;
    if use_random {
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
            // TODO remove this random. why None == some_action?
            action = rng.gen_range(0..ACTION_SIZE - 1);
        } else {
            action = some_action.unwrap();
        }
    }
    if dqn.rb.len() > BATCH_SIZE + 1 {
        let batch_indexes = [(); BATCH_SIZE].map(|_| rng.gen_range(0..dqn.rb.len()));
        let batch: [StateTuple; BATCH_SIZE] = dqn.rb.get_batch(batch_indexes);

        let mut states: Tensor2D<BATCH_SIZE, STATE_SIZE> = Tensor2D::zeros();
        let mut actions: [usize; BATCH_SIZE] = [0; BATCH_SIZE];
        let mut rewards: Tensor1D<BATCH_SIZE> = Tensor1D::zeros();
        let mut next_states: Tensor2D<BATCH_SIZE, STATE_SIZE> = Tensor2D::zeros();
        for (i, (s, a, r, s_n)) in batch.iter().enumerate() {
            states.mut_data()[i] = *s;
            actions[i] = 1 * a;
            rewards.mut_data()[i] = *r;
            next_states.mut_data()[i] = *s_n;
        }
        let done: Tensor1D<BATCH_SIZE> = Tensor1D::zeros();
        let next_q_values: Tensor2D<BATCH_SIZE, ACTION_SIZE> = dqn.tqn.forward(next_states);
        let max_next_q: Tensor1D<BATCH_SIZE> = next_q_values.max_last_dim();
        let target_q = 0.99 * mul(max_next_q, &(1.0 - done.clone())) + &rewards;
        let q_values = dqn.qn.forward(states.trace());
        let loss = mse_loss(q_values.gather_last_dim(&actions), &target_q);
        let loss_v = *loss.data();
        let gradients = loss.backward();
        sgd.sgd.update(&mut dqn.qn, gradients);

        if dqn.step % SYNC_INTERVAL_STEPS as i32 == 0 {
            println!(
                "networks sync er={loss_v:#.3} rb={:?} obs={:?}",
                dqn.rb.len(),
                obs.map(|o| o.mul(10.).round() / 10.),
            );
            dqn.tqn = dqn.qn.clone();
        }
        println!(
            "{:?}_{:?}_{:.2}_{:?} d_{progress_delta:.01} r_{reward:.1} loss_{loss_v:.2}",
            dqn.step,
            action,
            dqn.eps,
            if use_random { 1 } else { 0 },
        );
        dqn.eps = if dqn.eps < dqn.min_eps {
            dqn.min_eps
        } else {
            dqn.max_eps - DECAY * dqn.step as f32
        };
    } else {
        println!("rb_{:?}", dqn.rb.len());
    }
    dqn.rb.store(
        car_dqn.prev_obs,
        car_dqn.prev_action,
        car_dqn.prev_reward,
        obs,
    );
    car_dqn.prev_obs = obs;
    car_dqn.prev_action = action;
    car_dqn.prev_reward = reward;
    car_dqn.prev_progress = progress.meters;
    let gas = if action == 0 || action == 4 || action == 5 {
        1.
    } else {
        0.
    };
    let brake = if action == 1 || action == 6 || action == 7 {
        1.
    } else {
        0.
    };
    let left = if action == 2 || action == 4 || action == 6 {
        1.
    } else {
        0.
    };
    let right = if action == 3 || action == 5 || action == 7 {
        1.
    } else {
        0.
    };
    car.gas = gas;
    car.brake = brake;
    car.steering = -left + right;
}
