use super::{api_client::ReplayBufferRecord, dqn::*, gradient::AutoDevice, params::*};
use dfdx::prelude::*;
use std::ops::RangeFrom;

pub type Tensor1DUsize<const M: usize, Tape = NoneTape> = Tensor<Rank1<M>, usize, Cpu, Tape>;

type StateTuple = (Observation, usize, f32, Observation, f32);
type StateTensorsTuple = (
    Tensor2D<BATCH_SIZE, STATE_SIZE>, // s
    // [usize; BATCH_SIZE],              // a
    Tensor1DUsize<BATCH_SIZE>,        // a
    Tensor1D<BATCH_SIZE>,             // r
    Tensor2D<BATCH_SIZE, STATE_SIZE>, // sn
    Tensor1D<BATCH_SIZE>,             // done
);

const PERSIST_BATCH_SIZE: usize = 100;

pub struct ReplayBuffer {
    pub state: Vec<Observation>,
    pub action: Vec<usize>,
    pub reward: Vec<f32>,
    pub next_state: Vec<Observation>,
    pub done: Vec<f32>,
    pub i: usize,
}

impl ReplayBuffer {
    pub fn new() -> Self {
        Self {
            state: Vec::new(),
            action: Vec::new(),
            reward: Vec::new(),
            next_state: Vec::new(),
            done: Vec::new(),
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
                self.done[i],
            )
        })
    }
    pub fn get_batch_tensors(
        &self,
        sample_indexes: [usize; BATCH_SIZE],
        device: AutoDevice,
    ) -> StateTensorsTuple {
        let dev = AutoDevice::default();

        let batch: [StateTuple; BATCH_SIZE] = self.get_batch(sample_indexes);
        let mut states: Tensor2D<BATCH_SIZE, STATE_SIZE> = Tensor2D::zeros();
        let mut actions: [usize; BATCH_SIZE] = [0; BATCH_SIZE];
        let mut rewards: [f32; BATCH_SIZE] = [0.; BATCH_SIZE];
        let mut next_states: Tensor2D<BATCH_SIZE, STATE_SIZE> = Tensor2D::zeros();
        let mut done: [f32; BATCH_SIZE] = [0.; BATCH_SIZE];
        for (i, (s, a, r, s_n, d)) in batch.iter().enumerate() {
            states.mut_data()[i] = *s;
            actions[i] = 1 * a;
            rewards[i] = *r;
            next_states.mut_data()[i] = *s_n;
            done[i] = *d;
        }
        let actions = device.tensor_from_vec(actions.to_vec(), (Const::<BATCH_SIZE>,));
        let rewards = device.tensor_from_vec(rewards.to_vec(), (Const::<BATCH_SIZE>,));
        let done = device.tensor_from_vec(done.to_vec(), (Const::<BATCH_SIZE>,));
        (states, actions, rewards, next_states, done)
    }
    pub fn store(&mut self, s: Observation, a: usize, r: f32, sn: Observation, done: bool) {
        let done_float = if done { 1. } else { 0. };
        let i = self.i % BUFFER_SIZE;
        if self.len() < BUFFER_SIZE {
            self.state.push(s);
            self.action.push(a);
            self.reward.push(r);
            self.next_state.push(sn);
            self.done.push(done_float);
        } else {
            self.state[i] = s;
            self.action[i] = a;
            self.reward[i] = r;
            self.next_state[i] = sn;
            self.done[i] = done_float;
        }
        self.i += 1;
    }
    pub fn should_persist(&self) -> bool {
        return self.i % PERSIST_BATCH_SIZE == 0;
    }

    pub fn get_replay_buffer_to_persist(&self) -> Vec<ReplayBufferRecord> {
        let save_start_index = self.state.len() - PERSIST_BATCH_SIZE;
        let r: RangeFrom<usize> = save_start_index..;

        let records: Vec<ReplayBufferRecord> = self.state.as_slice()[r]
            .iter()
            .enumerate()
            .map(|t| {
                let i = save_start_index + t.0;
                return ReplayBufferRecord {
                    state: self.state[i].to_vec(),
                    action: self.action[i] as i32,
                    reward: self.reward[i] as f64,
                    next_state: self.next_state[i].to_vec(),
                    done: self.done[i] == 1.,
                };
            })
            .collect();
        return records;
    }
}
