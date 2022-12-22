use crate::{nn::*, replay::*};
use db_client::db::{rb, PrismaClient};
use dfdx::prelude::*;
use rand::Rng;
use std::time::Instant;
pub mod nn;
pub mod replay;

#[tokio::main]
async fn main() {
    println!("Training started");

    // let db_client = get_db_client();
    let db_client = PrismaClient::_builder().build().await.unwrap();
    #[cfg(debug)]
    db_client._db_push(false).await.unwrap();

    #[cfg(debug)]
    db_client._db_push(false).await.unwrap();

    let mut sgd = Sgd::new(SgdConfig {
        lr: LEARNING_RATE,
        momentum: Some(Momentum::Nesterov(0.9)),
        weight_decay: None,
    });
    let rb_data: Vec<rb::Data> = db_client.rb().find_many(vec![]).exec().await.unwrap();
    println!("rb_data len: {:?}", rb_data.len());
    let mut rb = ReplayBuffer::new();
    let mut dqn = Dqn::new();

    for r in rb_data.iter() {
        let mut state = OBSERVATION_ZERO;
        let mut next_state = OBSERVATION_ZERO;
        for (i, state_item) in r
            .state
            .split(",")
            .map(|x| x.parse::<f32>().unwrap())
            .enumerate()
        {
            state[i] = state_item;
        }
        for (i, next_state_item) in r
            .state
            .split(",")
            .map(|x| x.parse::<f32>().unwrap())
            .enumerate()
        {
            next_state[i] = next_state_item;
        }
        rb.store(
            state,
            r.action as usize,
            r.reward as f32,
            next_state,
            r.done,
        );

        let start = Instant::now();
        let mut rng = rand::thread_rng();
        let batch_indexes = [(); BATCH_SIZE].map(|_| rng.gen_range(0..rb.len()));
        let (s, a, r, sn, done) = rb.get_batch_tensors(batch_indexes);
        let mut loss_string: String = String::from("");
        for _i_epoch in 0..EPOCHS {
            let next_q_values: Tensor2D<BATCH_SIZE, ACTIONS> = dqn.tqn.forward(sn.clone());
            let max_next_q: Tensor1D<BATCH_SIZE> = next_q_values.max();
            let target_q = 0.99 * mul(max_next_q, 1.0 - done.clone()) + r.clone();
            // forward through model, computing gradients
            let q_values: Tensor2D<BATCH_SIZE, ACTIONS, OwnedTape> = dqn.qn.forward(s.trace());
            let action_qs: Tensor1D<BATCH_SIZE, OwnedTape> = q_values.select(&a);
            let loss = huber_loss(action_qs, target_q, 1.);
            let loss_v = *loss.data();
            // run backprop
            let gradients = loss.backward();
            sgd.update(&mut dqn.qn, gradients).expect("Unused params");
            if _i_epoch % 4 == 0 {
                loss_string.push_str(format!("{:.2} ", loss_v).as_str());
            }
        }
        log_training(&loss_string, start);
    }
    println!("Training ended");
}
