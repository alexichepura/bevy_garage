pub mod car;
pub mod config;
pub mod spawn;
pub mod wheel;

pub use car::{Car, JointType};
pub use wheel::Wheel;

use bevy::prelude::SystemSet;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum CarSet {
    Input,
    Brain,
    Esp,
}
