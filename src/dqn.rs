use crate::{car::Car, progress::CarProgress};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use dfdx::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::time::Instant;

const SIZE: usize = 64;
const BUFFER_SIZE: usize = 50_000;
const SENSORS_SIZE: usize = 7;
const STATE_SIZE: usize = SENSORS_SIZE + 2;
const ACTION_SIZE: usize = 4;
type QNetwork = (
    (Linear<STATE_SIZE, 32>, ReLU),
    (Linear<32, 32>, ReLU),
    Linear<32, ACTION_SIZE>,
);
type Action1D = Tensor1D<ACTION_SIZE>;
type Observation = [f32; STATE_SIZE];

pub struct ReplayBuffer {
    pub state: [Observation; BUFFER_SIZE],
    pub action: [usize; BUFFER_SIZE],
    pub reward: [f32; BUFFER_SIZE],
    pub next_state: [Observation; BUFFER_SIZE],
    pub i: usize,
}
impl ReplayBuffer {
    pub fn new() -> Self {
        Self {
            state: [[0.; STATE_SIZE]; BUFFER_SIZE],
            action: [0; BUFFER_SIZE],
            reward: [0.; BUFFER_SIZE],
            next_state: [[0.; STATE_SIZE]; BUFFER_SIZE],
            i: 0,
        }
    }
    pub fn store(
        &mut self,
        state: Observation,
        action: usize,
        reward: f32,
        next_state: Observation,
    ) {
        let i = self.i % SIZE;
        self.state[i] = state;
        self.action[i] = action;
        self.reward[i] = reward;
        self.next_state[i] = next_state;
        self.i += 1;
    }
}
pub struct DqnResource {
    pub seconds: i32,
    pub qn: QNetwork,
    pub tqn: QNetwork,
    pub rpl: ReplayBuffer,
    pub epsilon: f32,
    pub max_epsilon: f32,
    pub min_epsilon: f32,
    pub decay: f32,
    pub done: f32,
}
impl DqnResource {
    pub fn default() -> Self {
        let mut rng = StdRng::seed_from_u64(0);
        let mut qn: QNetwork = Default::default();
        qn.reset_params(&mut rng);
        Self {
            seconds: 0,
            qn: qn.clone(),
            tqn: qn.clone(),
            rpl: ReplayBuffer::new(),
            epsilon: 1.,
            max_epsilon: 1.,
            min_epsilon: 0.01,
            decay: 0.01,
            done: 0.,
        }
    }
}
pub struct SgdResource {
    pub sgd: Sgd<QNetwork>,
}
pub fn dqn_start_system(world: &mut World) {
    world.insert_non_send_resource(DqnResource::default());
    world.insert_non_send_resource(SgdResource {
        sgd: Sgd::new(SgdConfig {
            lr: 1e-1,
            momentum: Some(Momentum::Nesterov(0.9)),
        }),
    });
}

pub fn dqn_system(
    time: Res<Time>,
    mut dqn: NonSendMut<DqnResource>,
    mut sgd: NonSendMut<SgdResource>,
    q_car: Query<(&Car, &Velocity, &CarProgress), With<Car>>,
) {
    let (car, v, progress) = q_car.single();
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
    if random_number <= dqn.epsilon {
        action = rng.gen_range(0..3);
    } else {
        let start = Instant::now();
        let next_q_values: Action1D = dqn.tqn.forward(obs_state_tensor.clone());
        let max_next_q_value = *next_q_values.clone().max_last_dim().data();
        action = next_q_values
            .clone()
            .data()
            .iter()
            .position(|q| *q == max_next_q_value)
            .unwrap();
        // targ_q = R + discount * max(Q(S'))
        // curr_q = Q(S)[A]
        // loss = mse(curr_q, targ_q)
        let target_q = 0.99 * max_next_q_value + reward;
        let target_q_tensor = Tensor0D::new(target_q);
        let q_values = dqn.qn.forward(obs_state_tensor.trace());
        let curr_q = q_values.gather_last_dim(&action);
        let loss = mse_loss(curr_q, &target_q_tensor);
        let loss_v = *loss.data();

        let gradients = loss.backward();
        sgd.sgd.update(&mut dqn.qn, gradients);
        let seconds_round = seconds.round() as i32;
        if seconds_round > dqn.seconds {
            println!("q loss={:#.3} in {:?}", loss_v, start.elapsed());
            dbg!(dqn.rpl.action);
            dqn.seconds = seconds_round + 1;
        }
    }
    let prev_state = obs; // TODO !!!!!!!!!!!
    dqn.rpl.store(prev_state, action, reward, obs);
    dqn.epsilon =
        dqn.min_epsilon + (dqn.max_epsilon - dqn.min_epsilon) * (-dqn.decay * seconds as f32);
}
