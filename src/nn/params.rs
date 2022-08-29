pub const BATCH_SIZE: usize = 128;
pub const DECAY: f32 = 0.0001;
pub const LEARNING_RATE: f32 = 0.002;
pub const EPOCHS: usize = 50;
pub const BUFFER_SIZE: usize = 10_000_000;
pub const STEP_DURATION: f64 = 1. / 2.;
pub const SYNC_INTERVAL_STEPS: usize = 100;

pub const SENSOR_COUNT: usize = 32;
pub const STATE_SIZE_BASE: usize = 3;
pub const STATE_SIZE: usize = STATE_SIZE_BASE + SENSOR_COUNT;
pub const HIDDEN_SIZE: usize = 32;
pub const ACTIONS: usize = 8;

pub const CARS_COUNT: usize = 1;
pub const SPEED_LIMIT_KMH: f32 = 100.;
pub const SPEED_LIMIT_MPS: f32 = SPEED_LIMIT_KMH * 1000. / 3600.;
pub const STEERING_SPEEDLIMIT_KMH: f32 = 250.;
pub const MAX_TORQUE: f32 = 2000.;
pub const MAX_TOI: f32 = 100.;
