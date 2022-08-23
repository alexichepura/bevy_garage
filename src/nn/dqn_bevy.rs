use std::collections::HashMap;

use super::replay::ReplayBuffer;
use crate::{
    config::Config,
    dqn::{Observation, QNetwork, STATE_SIZE},
};
use bevy::prelude::*;
use dfdx::prelude::*;
use rand::{rngs::StdRng, SeedableRng};

pub struct CarDqnResource {
    pub prev_obs: Observation,
    pub prev_action: usize,
    pub prev_reward: f32,
    pub qn: QNetwork,
    pub tqn: QNetwork,
}

impl CarDqnResource {
    pub fn new() -> Self {
        let mut rng = StdRng::seed_from_u64(0);
        let mut qn = QNetwork::default();
        qn.reset_params(&mut rng);
        Self {
            prev_obs: [0.; STATE_SIZE],
            prev_action: 0,
            prev_reward: 0.,

            qn: qn.clone(),
            tqn: qn.clone(),
        }
    }
}

pub struct CarDqnResources {
    pub cars: HashMap<Entity, CarDqnResource>,
}
impl CarDqnResources {
    pub fn new() -> Self {
        Self {
            cars: HashMap::new(),
        }
    }
    pub fn add_car(&mut self, car_id: Entity) {
        self.cars.insert(car_id, CarDqnResource::new());
    }
}

pub struct DqnResource {
    pub seconds: f64,
    pub step: i32,
    pub rb: ReplayBuffer,
    pub eps: f32,
    pub max_eps: f32,
    pub min_eps: f32,
    pub done: f32,
    pub sgd: Sgd<QNetwork>,
}
impl DqnResource {
    pub fn new() -> Self {
        Self {
            seconds: 0.,
            step: 0,
            rb: ReplayBuffer::new(),
            eps: 1.,
            max_eps: 1.,
            min_eps: 0.01,
            done: 0.,
            sgd: Sgd::new(SgdConfig {
                lr: 0.01,
                momentum: Some(Momentum::Nesterov(0.9)),
            }),
        }
    }
}

pub fn dqn_start_system(world: &mut World) {
    world.insert_non_send_resource(DqnResource::new());
    world.insert_non_send_resource(CarDqnResources::new());
}
pub fn dqn_switch_system(mut config: ResMut<Config>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::N) {
        config.use_brain = !config.use_brain;
    }
}

use crate::{dash::*, dqn::SYNC_INTERVAL_STEPS};

pub fn dqn_dash_update_system(
    mut dash_set: ParamSet<(
        Query<&mut Text, With<TrainerRecordDistanceText>>,
        Query<&mut Text, With<TrainerGenerationText>>,
    )>,
    dqn: NonSend<DqnResource>,
) {
    let mut q_generation_text = dash_set.p1();
    let mut generation_text = q_generation_text.single_mut();
    generation_text.sections[0].value = format!(
        "rb {:?}, sync {:?}",
        dqn.rb.len(),
        (dqn.step / SYNC_INTERVAL_STEPS)
    );

    let mut q_timing_text = dash_set.p0();
    let mut timing_text = q_timing_text.single_mut();
    timing_text.sections[0].value = format!("epsilon {:.4}", dqn.eps);
}
