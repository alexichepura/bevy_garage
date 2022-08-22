use dfdx::tensor::{HasArrayData, Tensor1D, Tensor2D, TensorCreator};

use crate::dqn::{Observation, STATE_SIZE};

// https://machinelearningmastery.com/how-to-control-the-speed-and-stability-of-training-neural-networks-with-gradient-descent-batch-size/
pub const BATCH_SIZE: usize = 32;
const BUFFER_SIZE: usize = 10_000_000;

type StateTuple = (Observation, usize, f32, Observation);
type StateTensorsTuple = (
    Tensor2D<BATCH_SIZE, STATE_SIZE>, // s
    [usize; BATCH_SIZE],              // a
    Tensor1D<BATCH_SIZE>,             // r
    Tensor2D<BATCH_SIZE, STATE_SIZE>, // sn
    Tensor1D<BATCH_SIZE>,             // done
);

pub struct ReplayBuffer {
    pub state: Vec<Observation>,
    pub action: Vec<usize>,
    pub reward: Vec<f32>,
    pub next_state: Vec<Observation>,
    pub i: usize,
}

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
    pub fn get_batch_tensors(&self, sample_indexes: [usize; BATCH_SIZE]) -> StateTensorsTuple {
        let batch: [StateTuple; BATCH_SIZE] = self.get_batch(sample_indexes);
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
        (states, actions, rewards, next_states, done)
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
