use bevy::prelude::*;
use dfdx::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::time::Instant;

// https://github.com/coreylowman/dfdx/blob/main/examples/dqn.rs
// https://github.com/mswang12/minDQN/blob/main/minDQN.py
// https://iq.opengenus.org/deep-q-learning/
// https://towardsdatascience.com/deep-q-learning-tutorial-mindqn-2a4c855abffc

const SENSORS_SIZE: usize = 7;
const STATE_SIZE: usize = SENSORS_SIZE + 2;
const ACTION_SIZE: usize = 4;
type QNetwork = (
    (Linear<STATE_SIZE, 32>, ReLU),
    (Linear<32, 32>, ReLU),
    Linear<32, ACTION_SIZE>,
);
type TState = Tensor2D<64, STATE_SIZE>;
pub struct ReplayMemory {
    pub state: TState,
    pub next_state: TState,
    pub action: [usize; 64],
    pub reward: Tensor1D<64>,
    pub done: Tensor1D<64>,
}
impl ReplayMemory {
    pub fn default() -> Self {
        let mut rng = StdRng::seed_from_u64(0);
        Self {
            // state: Tensor2D::zeros(),
            state: Tensor2D::randn(&mut rng),
            // next_state: Tensor2D::zeros(),
            next_state: Tensor2D::randn(&mut rng),
            // action: [(); 64].map(|_| 0),
            action: [(); 64].map(|_| rng.gen_range(0..ACTION_SIZE)),
            // reward: Tensor1D::zeros(),
            reward: Tensor1D::randn(&mut rng),
            // done: Tensor1D::zeros(),
            done: Tensor1D::zeros(),
        }
    }
}
pub struct DqnResource {
    pub seconds: i32,
    pub qn: QNetwork,
    pub tqn: QNetwork,
    pub replay: ReplayMemory,
    // pub state: TState,
    // pub next_state: TState,
    // pub action: [usize; 64],
    // pub reward: Tensor1D<64>,
    // pub done: Tensor1D<64>,
    pub epsilon: f32,
    pub max_epsilon: f32,
    pub min_epsilon: f32,
    pub decay: f32,
}
impl DqnResource {
    pub fn default() -> Self {
        let qn: QNetwork = Default::default();
        Self {
            seconds: 0,
            qn: qn.clone(),
            tqn: qn.clone(),
            replay: ReplayMemory::default(),
            epsilon: 1.,
            max_epsilon: 1.,
            min_epsilon: 0.01,
            decay: 0.01,
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
) {
    let seconds = time.seconds_since_startup();
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0.0..1.0);
    if random_number <= dqn.epsilon {
        // action = env.action_space.sample()
        println!("TODO random action");
    } else {
        let start = Instant::now();
        // targ_q = R + discount * max(Q(S'))
        // curr_q = Q(S)[A]
        // loss = mse(curr_q, targ_q)
        let next_q_values = dqn.tqn.forward(dqn.replay.next_state.clone());
        let max_next_q = next_q_values.max_last_dim();
        let discount = 0.99;
        let target_q =
            discount * mul(max_next_q, &(1.0 - dqn.replay.done.clone())) + &dqn.replay.reward;
        let q_values = dqn.qn.forward(dqn.replay.state.trace());
        let loss = mse_loss(q_values.gather_last_dim(&dqn.replay.action), &target_q);
        let loss_v = *loss.data();
        let gradients = loss.backward();
        sgd.sgd.update(&mut dqn.qn, gradients);
        let seconds_round = seconds.round() as i32;
        if seconds_round > dqn.seconds {
            println!("q loss={:#.3} in {:?}", loss_v, start.elapsed());
            dbg!(dqn.replay.action);
            dqn.seconds = seconds_round + 1;
        }
    }
    dqn.epsilon =
        dqn.min_epsilon + (dqn.max_epsilon - dqn.min_epsilon) * (-dqn.decay * seconds as f32);
}
