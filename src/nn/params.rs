use crate::car::SENSOR_COUNT;

#[cfg(target_arch = "wasm32")]
pub const BATCH_SIZE: usize = 128;
#[cfg(not(target_arch = "wasm32"))]
pub const BATCH_SIZE: usize = 512;

#[cfg(target_arch = "wasm32")]
pub const EPOCHS: usize = 32;
#[cfg(not(target_arch = "wasm32"))]
pub const EPOCHS: usize = 500;

#[cfg(target_arch = "wasm32")]
pub const STEP_DURATION: f64 = 1. / 10.;
#[cfg(not(target_arch = "wasm32"))]
pub const STEP_DURATION: f64 = 1. / 20.;

pub const DECAY: f32 = 0.001;
pub const LEARNING_RATE: f32 = 0.001;
pub const SYNC_INTERVAL_STEPS: usize = 100;
pub const BUFFER_SIZE: usize = 10_000_000;
pub const HIDDEN_SIZE: usize = 16;

pub const STATE_SIZE_BASE: usize = 3;
pub const STATE_SIZE: usize = STATE_SIZE_BASE + SENSOR_COUNT;
pub const ACTIONS: usize = 8;
