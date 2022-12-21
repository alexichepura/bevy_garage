use super::{dqn::*, params::*};
use crate::api_client::ReplayBufferRecord;
use std::ops::RangeFrom;

const PERSIST_BATCH_SIZE: usize = 500;

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
