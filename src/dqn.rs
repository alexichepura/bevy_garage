use crate::{car::Car, progress::CarProgress};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use dfdx::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};

const BATCH_SIZE: usize = 64;
// const MIN_REPLAY_SIZE: usize = 1000;
const BUFFER_SIZE: usize = 50_000;
const SENSORS_SIZE: usize = 7;
const STATE_SIZE: usize = SENSORS_SIZE + 2;
const ACTION_SIZE: usize = 4;
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
    pub seconds: i32,
    pub qn: QNetwork,
    pub tqn: QNetwork,
    pub rb: ReplayBuffer,
    pub eps: f32,
    pub max_eps: f32,
    pub min_eps: f32,
    pub decay: f32,
    pub done: f32,
}
impl DqnResource {
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(0);
        let mut qn = QNetwork::default();
        qn.reset_params(&mut rng);
        Self {
            seconds: 0,
            qn: qn.clone(),
            tqn: qn.clone(),
            rb: ReplayBuffer::new(),
            eps: 1.,
            max_eps: 1.,
            min_eps: 0.01,
            decay: 0.01,
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
}

pub fn dqn_system(
    time: Res<Time>,
    mut dqn: NonSendMut<DqnResource>,
    mut sgd: NonSendMut<SgdResource>,
    mut q_car: Query<(&mut Car, &Velocity, &CarProgress, &mut CarDqn), With<CarDqn>>,
) {
    let (mut car, v, progress, mut car_dqn) = q_car.single_mut();
    let obs: Observation = [
        car.sensor_inputs[0],
        car.sensor_inputs[1],
        car.sensor_inputs[2],
        car.sensor_inputs[3],
        car.sensor_inputs[4],
        car.sensor_inputs[5],
        car.sensor_inputs[6],
        v.linvel.length(),
        progress.meters,
    ];
    let obs_state_tensor = Tensor1D::new(obs);
    let seconds = time.seconds_since_startup();
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0.0..1.0);
    let action: usize;
    let reward = progress.meters;
    if random_number <= dqn.eps {
        action = rng.gen_range(0..3);
    } else {
        let q_values = dqn.qn.forward(obs_state_tensor.clone());
        let max_q_value = *q_values.clone().max_last_dim().data();
        action = q_values
            .clone()
            .data()
            .iter()
            .position(|q| *q == max_q_value)
            .unwrap();

        let batch_indexes = [(); BATCH_SIZE].map(|_| rng.gen_range(0..dqn.rb.len()));
        if dqn.rb.len() > BATCH_SIZE {
            let batch: [StateTuple; BATCH_SIZE] = dqn.rb.get_batch(batch_indexes);

            let mut states: Tensor2D<BATCH_SIZE, STATE_SIZE> = Tensor2D::zeros();
            let mut actions: [usize; BATCH_SIZE] = [0; BATCH_SIZE];
            let mut rewards: Tensor1D<BATCH_SIZE> = Tensor1D::zeros();
            let mut next_states: Tensor2D<BATCH_SIZE, STATE_SIZE> = Tensor2D::zeros();
            for (i, (state, action, reward, next_state)) in batch.iter().enumerate() {
                states.mut_data()[i] = *state;
                actions[i] = *action;
                rewards.mut_data()[i] = *reward;
                next_states.mut_data()[i] = *next_state;
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

            let seconds_round = seconds.round() as i32;
            if seconds_round > dqn.seconds {
                println!("obs={:?} loss={loss_v:#.3} rb_len={:?}", obs, dqn.rb.len());
                dqn.seconds = seconds_round + 1;
                dqn.tqn = dqn.qn.clone();
            }
        }
    }
    if car_dqn.prev_action != action {
        println!("action={action:?}");
    }
    dqn.rb.store(car_dqn.prev_obs, action, reward, obs);
    car_dqn.prev_obs = obs;
    car_dqn.prev_action = action;
    dqn.eps = dqn.min_eps + (dqn.max_eps - dqn.min_eps) * (-dqn.decay * seconds as f32);

    let gas = if action == 0 { 1. } else { 0. };
    let brake = if action == 1 { 1. } else { 0. };
    let left = if action == 2 { 1. } else { 0. };
    let right = if action == 3 { 1. } else { 0. };
    car.gas = gas;
    car.brake = brake;
    car.steering = -left + right;
}
