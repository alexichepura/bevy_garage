pub mod car;
pub mod config;
pub mod joint;
pub mod sensor;
pub mod spawn;
pub mod wheel;

pub use car::*;
pub use config::CarConfig;
pub use joint::WheelJoint;
pub use wheel::Wheel;

use bevy::prelude::SystemSet;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum CarSet {
    Input,
    Brain,
    Esp,
}
