use bevy::prelude::*;
use dfdx::prelude::*;
use std::time::Instant;

const SENSORS_SIZE: usize = 7;
const STATE_SIZE: usize = SENSORS_SIZE + 2;
const ACTION_SIZE: usize = 4;

type QNetwork = (
    (Linear<STATE_SIZE, 32>, ReLU),
    (Linear<32, 32>, ReLU),
    Linear<32, ACTION_SIZE>,
);

// #[derive(Default)]
pub struct Dqn {
    pub qn: QNetwork,
    pub state: Tensor2D<64, STATE_SIZE>,
    pub next_state: Tensor2D<64, STATE_SIZE>,
    pub action: [usize; 64],
    pub reward: Tensor1D<64>,
    pub done: Tensor1D<64>,
    pub sgd: Sgd<QNetwork>,
}

impl Dqn {
    pub fn default() -> Self {
        Self {
            qn: Default::default(),
            state: Tensor2D::zeros(),
            next_state: Tensor2D::zeros(),
            action: [(); 64].map(|_| 0),
            reward: Tensor1D::zeros(),
            done: Tensor1D::zeros(),
            sgd: Sgd::new(SgdConfig {
                lr: 1e-1,
                momentum: Some(Momentum::Nesterov(0.9)),
            }),
        }
    }
}

pub fn dqn_start_system(world: &mut World) {
    let dqn = Dqn::default();
    world.insert_non_send_resource(dqn);
}
pub fn dfdx_system(dqn: NonSend<Dqn>) {
    let start = Instant::now();
    // targ_q = R + discount * max(Q(S'))
    // curr_q = Q(S)[A]
    // loss = mse(curr_q, targ_q)
    // let next_q_values = target_q_net.forward(next_state.clone());
    // let max_next_q = next_q_values.max_last_dim();
    // let target_q = 0.99 * mul(max_next_q, &(1.0 - done.clone())) + &reward;
    // let q_values = q_net.forward(state.trace());
    // let loss = mse_loss(q_values.gather_last_dim(&action), &target_q);
    // let loss_v = *loss.data();
    // let gradients = loss.backward();
    // sgd.update(&mut q_net, gradients);
    // println!("q loss={:#.3} in {:?}", loss_v, start.elapsed());
}
