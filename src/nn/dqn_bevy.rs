use super::{
    gradient::{get_sgd, AutoDevice},
    params::*,
    replay::ReplayBuffer,
};
use crate::{dash::*, nn::dqn::*};
use bevy::prelude::*;
use dfdx::{optim::Sgd, prelude::*};
use rand::Rng;

#[derive(Component, Debug)]
pub struct CarDqnPrev {
    pub prev_obs: Observation,
    pub prev_action: usize,
    pub prev_reward: f32,
}

impl CarDqnPrev {
    pub fn new() -> Self {
        Self {
            prev_obs: [0.; STATE_SIZE],
            prev_action: 0,
            prev_reward: 0.,
        }
    }
}

pub struct CarsDqnResource {
    pub qn: QNetworkBuilt,
    pub tqn: QNetworkBuilt,
    pub device: AutoDevice,
    pub gradients: Gradients<f32, Cpu>,
}
impl CarsDqnResource {
    pub fn act(&self, obs: Observation, epsilon: f32) -> (usize, bool) {
        let obs_state_tensor = self
            .device
            .tensor_from_vec(obs.to_vec(), (Const::<STATE_SIZE>,));
        let mut rng = rand::thread_rng();
        let random_number = rng.gen_range(0.0..1.0);
        let exploration = random_number < epsilon;

        let action: usize = if exploration {
            rng.gen_range(0..ACTIONS - 1)
        } else {
            let q_values = self.qn.forward(obs_state_tensor.clone());
            let max_q_value = q_values.clone().max::<Rank0, _>();
            let some_action = q_values
                .clone()
                .array()
                .iter()
                .position(|q| *q >= max_q_value.array());
            if None == some_action {
                dbg!(q_values);
                panic!();
            } else {
                some_action.unwrap()
            }
        };
        (action, exploration)
    }
    pub fn new(qn: &QNetworkBuilt, device: AutoDevice) -> Self {
        let gradients = qn.alloc_grads();
        Self {
            qn: qn.clone(),
            tqn: qn.clone(),
            device,
            gradients,
        }
    }
}

#[derive(Resource)]
pub struct DqnResource {
    pub seconds: f64,
    pub step: usize,
    pub crashes: usize,
    pub rb: ReplayBuffer,
    pub eps: f32,
    pub max_eps: f32,
    pub min_eps: f32,
    pub done: f32,

    pub respawn_at: f64,
    pub respawn_is_hid: bool,
    pub respawn_index: usize,
}
impl DqnResource {
    pub fn default() -> Self {
        Self {
            seconds: 0.,
            step: 0,
            crashes: 0,
            rb: ReplayBuffer::new(),
            eps: 1.,
            max_eps: 1.,
            min_eps: 0.01,
            done: 0.,

            respawn_at: 0.,
            respawn_is_hid: false,
            respawn_index: 0,
        }
    }
}

pub struct SgdResource {
    pub sgd: Sgd<QNetworkBuilt, f32, AutoDevice>,
}
impl SgdResource {
    pub fn new(qn: &QNetworkBuilt) -> Self {
        let sgd = get_sgd(qn);
        Self { sgd }
    }
}

pub fn dqn_exclusive_start_system(world: &mut World) {
    let device = AutoDevice::default();
    let mut qn: QNetworkBuilt = device.build_module::<QNetwork, f32>();
    qn.reset_params();
    world.insert_non_send_resource(SgdResource::new(&qn));
    world.insert_non_send_resource(CarsDqnResource::new(&qn, device));
}

pub fn dqn_dash_update_system(
    mut dash_set: ParamSet<(
        Query<&mut Text, With<TrainerRecordDistanceText>>,
        Query<&mut Text, With<TrainerGenerationText>>,
    )>,
    dqn: Res<DqnResource>,
) {
    let mut q_generation_text = dash_set.p1();
    let mut generation_text = q_generation_text.single_mut();
    generation_text.sections[0].value = format!(
        "rb {:?}, sync {:?}, crashes {:?}",
        dqn.rb.len(),
        (dqn.step / SYNC_INTERVAL_STEPS),
        dqn.crashes
    );

    let mut q_timing_text = dash_set.p0();
    let mut timing_text = q_timing_text.single_mut();
    timing_text.sections[0].value = format!("epsilon {:.4}", dqn.eps);
}
