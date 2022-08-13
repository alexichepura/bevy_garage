use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use dfdx::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::time::Instant;

use crate::{car::Car, progress::CarProgress};

// https://github.com/coreylowman/dfdx/blob/main/examples/dqn.rs
// https://alexandervandekleut.github.io/deep-q-learning/
// https://towardsdatascience.com/a-minimal-working-example-for-deep-q-learning-in-tensorflow-2-0-e0ca8a944d5e
// https://github.com/mswang12/minDQN/blob/main/minDQN.py
// https://iq.opengenus.org/deep-q-learning/
// https://towardsdatascience.com/deep-q-learning-tutorial-mindqn-2a4c855abffc

const SIZE: usize = 64;
const SENSORS_SIZE: usize = 7;
const STATE_SIZE: usize = SENSORS_SIZE + 2;
const ACTION_SIZE: usize = 4;
type QNetwork = (
    (Linear<STATE_SIZE, 32>, ReLU),
    (Linear<32, 32>, ReLU),
    Linear<32, ACTION_SIZE>,
);
type StateSizeTensor = Tensor1D<STATE_SIZE>;
type ActionSizeTensor = Tensor1D<ACTION_SIZE>;
type ObsState = [f32; STATE_SIZE];

pub struct ReplayMemory {
    pub state: StateSizeTensor,
    pub next_state: StateSizeTensor,
    pub action: [usize; SIZE],
    pub reward: Tensor1D<SIZE>,
    pub done: Tensor0D,
    pub i: usize,
}
impl ReplayMemory {
    pub fn default() -> Self {
        Self {
            state: Tensor1D::zeros(),
            next_state: Tensor1D::zeros(),
            action: [(); SIZE].map(|_| 0),
            reward: Tensor1D::zeros(),
            done: Tensor0D::zeros(),
            i: 0,
        }
    }
    pub fn random() -> Self {
        let mut rng = StdRng::seed_from_u64(0);
        Self {
            state: Tensor1D::randn(&mut rng),
            next_state: Tensor1D::randn(&mut rng),
            action: [(); SIZE].map(|_| rng.gen_range(0..ACTION_SIZE)),
            reward: Tensor1D::randn(&mut rng),
            done: Tensor0D::zeros(),
            i: 0,
        }
    }
    // pub fn store(
    //     &mut self,
    //     state: TSingleState,
    //     next_state: TSingleState,
    //     action: usize,
    //     reward: f32,
    // ) {
    //     let i = self.i % SIZE;
    //     self.state.mut_data()[i] = state;
    //     self.next_state.mut_data()[i] = next_state;
    //     self.action[i] = action;
    //     self.reward.mut_data()[i] = reward;
    //     self.i += 1;
    // }
}
pub struct DqnResource {
    pub seconds: i32,
    pub qn: QNetwork,
    pub tqn: QNetwork,
    pub rpl: ReplayMemory,
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
            rpl: ReplayMemory::random(),
            // rpl: ReplayMemory::default(),
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
    q_car: Query<(&Car, &Velocity, &CarProgress), With<Car>>,
) {
    let (car, v, progress) = q_car.single();
    let obs_state: ObsState = [
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
    let obs_state_tensor = Tensor1D::new(obs_state);

    let seconds = time.seconds_since_startup();
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(0.0..1.0);
    let mut action: usize = 0;

    if random_number <= dqn.epsilon {
        action = rng.gen_range(0..3);
    } else {
        let start = Instant::now();
        let q_values: ActionSizeTensor = dqn.tqn.forward(obs_state_tensor);
        dbg!(q_values.data());
        // targ_q = R + discount * max(Q(S'))
        let max_next_q_values = q_values.max_last_dim();
        // dbg!(max_next_q_values.data());

        // let max_next_q = mul(max_next_q_values, &(1.0 - dqn.rpl.done.clone()));
        // let discount = 0.99;
        // let reward = 1.; // &dqn.rpl.reward;
        // let target_q = discount * max_next_q + reward;
        // let max_target_q = target_q.clone().max_last_dim();

        // // curr_q = Q(S)[A]
        // let q_values = dqn.qn.forward(dqn.rpl.state.trace());
        // let curr_q = q_values.gather_last_dim(&dqn.rpl.action);

        // // loss = mse(curr_q, targ_q)
        // let loss = mse_loss(curr_q, &target_q);
        // let loss_v = *loss.data();

        // let gradients = loss.backward();
        // sgd.sgd.update(&mut dqn.qn, gradients);
        // let seconds_round = seconds.round() as i32;
        // if seconds_round > dqn.seconds {
        //     println!("q loss={:#.3} in {:?}", loss_v, start.elapsed());
        //     dbg!(dqn.rpl.action);
        //     dqn.seconds = seconds_round + 1;
        // }
    }
    // dqn.rpl.store(obs_state, obs_state, action, 0.);
    dqn.epsilon =
        dqn.min_epsilon + (dqn.max_epsilon - dqn.min_epsilon) * (-dqn.decay * seconds as f32);
}
