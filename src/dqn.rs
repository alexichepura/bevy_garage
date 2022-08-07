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
pub struct SgdResource {
    pub sgd: Sgd<QNetwork>,
}
pub struct DqnResource {
    pub qn: QNetwork,
    pub tqn: QNetwork,
    pub state: Tensor2D<64, STATE_SIZE>,
    pub next_state: Tensor2D<64, STATE_SIZE>,
    pub action: [usize; 64],
    pub reward: Tensor1D<64>,
    pub done: Tensor1D<64>,
}
impl DqnResource {
    pub fn default() -> Self {
        let qn: QNetwork = Default::default();
        Self {
            qn: qn.clone(),
            tqn: qn.clone(),
            state: Tensor2D::zeros(),
            next_state: Tensor2D::zeros(),
            action: [(); 64].map(|_| 0),
            reward: Tensor1D::zeros(),
            done: Tensor1D::zeros(),
        }
    }
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
pub fn dqn_system(mut dqn: NonSendMut<DqnResource>, mut sgd: NonSendMut<SgdResource>) {
    let start = Instant::now();
    // targ_q = R + discount * max(Q(S'))
    // curr_q = Q(S)[A]
    // loss = mse(curr_q, targ_q)
    let next_q_values = dqn.tqn.forward(dqn.next_state.clone());
    let max_next_q = next_q_values.max_last_dim();
    let target_q = 0.99 * mul(max_next_q, &(1.0 - dqn.done.clone())) + &dqn.reward;
    let q_values = dqn.qn.forward(dqn.state.trace());
    let loss = mse_loss(q_values.gather_last_dim(&dqn.action), &target_q);
    let loss_v = *loss.data();
    let gradients = loss.backward();
    sgd.sgd.update(&mut dqn.qn, gradients);
    println!("q loss={:#.3} in {:?}", loss_v, start.elapsed());
}
