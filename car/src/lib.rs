pub mod car;
pub mod joint;
pub mod res;
pub mod sensor;
pub mod spawn;
pub mod wheel;

pub use car::*;
pub use joint::WheelJoint;
pub use res::CarRes;
pub use wheel::*;

use bevy::prelude::SystemSet;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum CarSet {
    Input,
    Brain,
    Esp,
}
