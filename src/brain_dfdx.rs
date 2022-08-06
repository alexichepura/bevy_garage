use std::time::Instant;

use dfdx::prelude::*;

const SENSORS_SIZE: usize = 7;
const STATE_SIZE: usize = SENSORS_SIZE + 2;
const ACTION_SIZE: usize = 4;

type QNetwork = (
    (Linear<STATE_SIZE, 32>, ReLU),
    (Linear<32, 32>, ReLU),
    Linear<32, ACTION_SIZE>,
);

pub fn dfdx_start_system() {
    let state: Tensor2D<64, STATE_SIZE> = Tensor2D::new([(); 64].map(|_| [0.; STATE_SIZE]));
    let action: [usize; 64] = [(); 64].map(|_| 0);
    let reward: Tensor1D<64> = Tensor1D::new([(); 64].map(|_| 0.));
    let done: Tensor1D<64> = Tensor1D::zeros();
    let next_state: Tensor2D<64, STATE_SIZE> = Tensor2D::new([(); 64].map(|_| [0.; STATE_SIZE]));

    let mut q_net: QNetwork = Default::default();
    let target_q_net: QNetwork = q_net.clone();

    let mut sgd = Sgd::new(SgdConfig {
        lr: 1e-1,
        momentum: Some(Momentum::Nesterov(0.9)),
    });

    let start = Instant::now();
    // targ_q = R + discount * max(Q(S'))
    // curr_q = Q(S)[A]
    // loss = mse(curr_q, targ_q)
    let next_q_values = target_q_net.forward(next_state.clone());
    let max_next_q = next_q_values.max_last_dim();
    let target_q = 0.99 * mul(max_next_q, &(1.0 - done.clone())) + &reward;

    // forward through model, computing gradients
    let q_values = q_net.forward(state.trace());

    let loss = mse_loss(q_values.gather_last_dim(&action), &target_q);
    let loss_v = *loss.data();

    // run backprop
    let gradients = loss.backward();

    // update weights with optimizer
    sgd.update(&mut q_net, gradients);

    println!("q loss={:#.3} in {:?}", loss_v, start.elapsed());
}
