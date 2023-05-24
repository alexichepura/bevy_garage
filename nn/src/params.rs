use bevy_garage_car::sensor::SENSOR_COUNT;

#[cfg(target_arch = "wasm32")]
pub const EPOCHS: usize = 32;
#[cfg(not(target_arch = "wasm32"))]
pub const EPOCHS: usize = 64;

#[cfg(target_arch = "wasm32")]
pub const BATCH_SIZE: usize = 32;
#[cfg(not(target_arch = "wasm32"))]
pub const BATCH_SIZE: usize = 256;

#[cfg(target_arch = "wasm32")]
pub const LEARNING_RATE: f32 = 0.01;
#[cfg(not(target_arch = "wasm32"))]
pub const LEARNING_RATE: f32 = 0.005;

#[cfg(target_arch = "wasm32")]
pub const STEP_DURATION: f64 = 1. / 10.;
#[cfg(not(target_arch = "wasm32"))]
pub const STEP_DURATION: f64 = 1. / 30.;

#[cfg(target_arch = "wasm32")]
pub const HIDDEN_SIZE: usize = 32;
#[cfg(not(target_arch = "wasm32"))]
pub const HIDDEN_SIZE: usize = 256;

pub const DECAY: f32 = 0.001;
pub const SYNC_INTERVAL_STEPS: usize = 300;
pub const BUFFER_SIZE: usize = 10_000_000;

pub const STATE_SIZE_BASE: usize = 5;
pub const STATE_SIZE: usize = STATE_SIZE_BASE + SENSOR_COUNT;
pub const ACTIONS: usize = 9; //
