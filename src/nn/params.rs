use crate::car::SENSOR_COUNT;

pub const BATCH_SIZE: usize = 64;
pub const BUFFER_SIZE: usize = 10_000_000;
pub const EPOCHS: usize = 100;
pub const LEARNING_RATE: f32 = 0.002;
pub const DECAY: f32 = 0.001;
pub const SYNC_INTERVAL_STEPS: i32 = 100;
pub const STEP_DURATION: f64 = 1. / 10.;
pub const STATE_SIZE_BASE: usize = 3;
pub const STATE_SIZE: usize = STATE_SIZE_BASE + SENSOR_COUNT;
pub const ACTIONS: usize = 8;
pub const HIDDEN_SIZE: usize = 32;

pub const MAX_TORQUE: f32 = 500.;
pub const CARS_COUNT: usize = 20;
pub const SPEED_LIMIT_KMH: f32 = 50.;
pub const SPEED_LIMIT_MPS: f32 = SPEED_LIMIT_KMH * 1000. / 3600.;
pub const STEERING_SPEEDLIMIT_KMH: f32 = 150.;
