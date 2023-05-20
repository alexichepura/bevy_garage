use crate::{
    dash::{TrainerEpsilonText, TrainerGenerationText},
    dqn::*,
    gradient::get_sgd,
    params::*,
    replay::ReplayBuffer,
};
use bevy::prelude::*;
use crossbeam_channel::{bounded, Receiver, Sender};
use dfdx::{optim::Sgd, prelude::*};
use rand::Rng;

#[derive(Component, Debug)]
pub struct CarDqn {
    pub max_speed: f32,
    pub prev_obs: Observation,
    pub prev_action: usize,
    pub prev_reward: f32,
}

impl CarDqn {
    pub fn new() -> Self {
        Self {
            max_speed: 100. * 1000. / 3600.,
            prev_obs: [0.; STATE_SIZE],
            prev_action: 0,
            prev_reward: 0.,
        }
    }
}

pub struct CarsDqnResource {
    pub processing: bool,
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
            processing: false,
            qn: qn.clone(),
            tqn: qn.clone(),
            device,
            gradients,
        }
    }
}

#[derive(Resource)]
pub struct DqnResource {
    pub use_nn: bool,
    pub seconds: f64,
    pub step: usize,
    pub crashes: usize,
    pub rb: ReplayBuffer,
    pub eps: f32,
    pub max_eps: f32,
    pub min_eps: f32,
    pub done: f32,

    pub respawn_in: f64,
    pub respawn_player: bool,
    pub respawn_index: usize,
}
impl DqnResource {
    pub fn default() -> Self {
        Self {
            use_nn: false,
            seconds: 0.,
            step: 0,
            crashes: 0,
            rb: ReplayBuffer::new(),
            eps: 1.,
            max_eps: 1.,
            min_eps: 0.01,
            done: 0.,

            respawn_in: 0.,
            respawn_player: false,
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

pub struct DqnX {
    pub loss_string: String,
    pub duration_string: String,
    pub qn: QNetworkBuilt,
}

#[derive(Resource, Deref)]
pub struct DqnRx(Receiver<DqnX>);
#[derive(Resource, Deref)]
pub struct DqnTx(Sender<DqnX>);
pub struct DqnEvent(DqnX);

pub fn dqn_start_system(world: &mut World) {
    let device = AutoDevice::default();
    let mut qn: QNetworkBuilt = device.build_module::<QNetwork, f32>();
    qn.reset_params();
    world.insert_non_send_resource(SgdResource::new(&qn));
    world.insert_non_send_resource(CarsDqnResource::new(&qn, device));
}
pub fn dqn_x_start_system(mut cmd: Commands) {
    let (tx, rx) = bounded::<DqnX>(10);
    cmd.insert_resource(DqnRx(rx));
    cmd.insert_resource(DqnTx(tx));
}
pub fn dqn_rx_to_bevy_event_system(receiver: Res<DqnRx>, mut events: EventWriter<DqnEvent>) {
    for from_stream in receiver.try_iter() {
        events.send(DqnEvent(from_stream));
    }
}
pub fn dqn_event_reader_system(
    mut reader: EventReader<DqnEvent>,
    mut cars_dqn: NonSendMut<CarsDqnResource>,
) {
    for event in reader.iter() {
        // dbg!((&event.0.duration_string, &event.0.loss_string));
        println!(
            "dqn_event:{}:{}",
            &event.0.duration_string, &event.0.loss_string
        );
        cars_dqn.qn = event.0.qn.clone();
        cars_dqn.processing = false;
    }
}

pub fn dqn_dash_update_system(
    mut dash_set: ParamSet<(
        Query<&mut Text, With<TrainerEpsilonText>>,
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
