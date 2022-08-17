use super::replay::ReplayBuffer;
use crate::dqn::{Observation, QNetwork, STATE_SIZE};
use bevy::prelude::*;
use dfdx::prelude::*;
use rand::{rngs::StdRng, SeedableRng};

const LEARNING_RATE: f32 = 0.01;

pub struct DqnResource {
    pub seconds: f64,
    pub step: i32,
    pub qn: QNetwork,
    pub tqn: QNetwork,
    pub rb: ReplayBuffer,
    pub eps: f32,
    pub max_eps: f32,
    pub min_eps: f32,
    pub done: f32,
    pub sgd: Sgd<QNetwork>,
}
impl DqnResource {
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(0);
        let mut qn = QNetwork::default();
        qn.reset_params(&mut rng);
        Self {
            seconds: 0.,
            step: 0,
            qn: qn.clone(),
            tqn: qn.clone(),
            rb: ReplayBuffer::new(),
            eps: 1.,
            max_eps: 1.,
            min_eps: 0.01,
            done: 0.,
            sgd: Sgd::new(SgdConfig {
                lr: LEARNING_RATE,
                momentum: Some(Momentum::Nesterov(0.9)),
            }),
        }
    }
    pub fn sgd_update(&mut self, gradients: Gradients) {
        self.sgd.update(&mut self.qn, gradients);
    }
}

pub fn dqn_start_system(world: &mut World) {
    world.insert_non_send_resource(DqnResource::new());
}

#[derive(Component, Debug)]
pub struct CarDqn {
    pub prev_obs: Observation,
    pub prev_action: usize,
    pub prev_reward: f32,
    pub prev_progress: f32,
}
impl CarDqn {
    pub fn new() -> Self {
        Self {
            prev_obs: [0.; STATE_SIZE],
            prev_action: 0,
            prev_reward: 0.,
            prev_progress: 0.,
        }
    }
}