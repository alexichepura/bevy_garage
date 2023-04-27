use dfdx::prelude::*;
use std::time::Instant;
// use db_client::db::{rb, PrismaClient};
// use crate::replay::ReplayBuffer;

pub const ACTIONS: usize = 8;
pub const SENSOR_COUNT: usize = 31;
pub const STATE_SIZE_BASE: usize = 3;
pub const STATE_SIZE: usize = STATE_SIZE_BASE + SENSOR_COUNT;
pub type Observation = [f32; STATE_SIZE];
pub const OBSERVATION_ZERO: Observation = [0.; STATE_SIZE];

pub const BATCH_SIZE: usize = 256;
pub const EPOCHS: usize = 50;
pub const LEARNING_RATE: f32 = 0.0025;
// pub const DECAY: f32 = 0.001;

pub const HIDDEN_SIZE: usize = 16;
pub type QNetwork = (
    (Linear<STATE_SIZE, HIDDEN_SIZE>, ReLU),
    (Linear<HIDDEN_SIZE, HIDDEN_SIZE>, ReLU),
    Linear<HIDDEN_SIZE, ACTIONS>,
);

pub type QNetworkBuilt = <QNetwork as BuildOnDevice<AutoDevice, f32>>::Built;

pub struct Dqn {
    pub qn: QNetworkBuilt,
    pub tqn: QNetworkBuilt,
    pub gradients: Gradients<f32, Cpu>,
}
impl Dqn {
    pub fn new(qn: &QNetworkBuilt) -> Self {
        let gradients = qn.alloc_grads();
        Self {
            qn: qn.clone(),
            tqn: qn.clone(),
            gradients,
        }
    }
}

// pub fn create_training_route() -> Router {
//     Router::new().route("/train", get(start_training))
// }
// pub async fn start_training(db: PrismaClient) {
//     println!("Training started");

//     let mut sgd = Sgd::new(SgdConfig {
//         lr: LEARNING_RATE,
//         momentum: Some(Momentum::Nesterov(0.9)),
//         weight_decay: None,
//     });
//     let rb_data: Vec<rb::Data> = db.rb().find_many(vec![]).exec().await.unwrap();
//     let mut rb = ReplayBuffer::new();
//     let mut dqn = Dqn::new();

//     for r in rb_data.iter() {
//         let mut state = OBSERVATION_ZERO;
//         let mut next_state = OBSERVATION_ZERO;
//         for (i, state_item) in r
//             .state
//             .split(",")
//             .map(|x| x.parse::<f32>().unwrap())
//             .enumerate()
//         {
//             state[i] = state_item;
//         }
//         for (i, next_state_item) in r
//             .state
//             .split(",")
//             .map(|x| x.parse::<f32>().unwrap())
//             .enumerate()
//         {
//             next_state[i] = next_state_item;
//         }
//         rb.store(
//             state,
//             r.action as usize,
//             r.reward as f32,
//             next_state,
//             r.done,
//         );

//         let start = Instant::now();
//         let mut rng = rand::thread_rng();
//         let batch_indexes = [(); BATCH_SIZE].map(|_| rng.gen_range(0..rb.len()));
//         let (s, a, r, sn, done) = rb.get_batch_tensors(batch_indexes);
//         let mut loss_string: String = String::from("");
//         for _i_epoch in 0..EPOCHS {
//             let next_q_values: Tensor2D<BATCH_SIZE, ACTIONS> = dqn.tqn.forward(sn.clone());
//             let max_next_q: Tensor1D<BATCH_SIZE> = next_q_values.max();
//             let target_q = 0.99 * mul(max_next_q, 1.0 - done.clone()) + r.clone();
//             // forward through model, computing gradients
//             let q_values: Tensor2D<BATCH_SIZE, ACTIONS, OwnedTape> = dqn.qn.forward(s.trace());
//             let action_qs: Tensor1D<BATCH_SIZE, OwnedTape> = q_values.select(&a);
//             let loss = huber_loss(action_qs, target_q, 1.);
//             let loss_v = *loss.data();
//             // run backprop
//             let gradients = loss.backward();
//             sgd.update(&mut dqn.qn, gradients).expect("Unused params");
//             if _i_epoch % 4 == 0 {
//                 loss_string.push_str(format!("{:.2} ", loss_v).as_str());
//             }
//         }
//         log_training(&loss_string, start);
//         //     if dqn.step % SYNC_INTERVAL_STEPS == 0 && dqn.rb.len() > BATCH_SIZE * 2 {
//         //         dbg!("networks sync");
//         //         cars_dqn.tqn = cars_dqn.qn.clone();
//         //     }
//         //     dqn.eps = if dqn.eps <= dqn.min_eps {
//         //         dqn.min_eps
//         //     } else {
//         //         dqn.eps - DECAY
//         //     };
//     }
// }

pub fn log_training(loss_string: &String, start: Instant) {
    let log = [
        start.elapsed().as_millis().to_string() + "ms",
        " ".to_string(),
        loss_string.to_string(),
    ]
    .join("");
    println!("{log:?}");
}
