use super::params::LEARNING_RATE;
use dfdx::{
    optim::{Momentum, Sgd, SgdConfig},
    tensor::AutoDevice,
};

pub fn get_sgd<M>(m: &M) -> Sgd<M, f32, AutoDevice> {
    let sgd: Sgd<M, f32, AutoDevice> = Sgd::new(
        m,
        SgdConfig {
            lr: LEARNING_RATE,
            momentum: Some(Momentum::Nesterov(0.9)),
            weight_decay: None,
        },
    );
    sgd
}
