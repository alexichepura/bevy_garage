pub mod car;
pub mod esp;
pub mod joint;
pub mod res;
pub mod sensor;
pub mod spawn;
pub mod spec;
pub mod wheel;

pub use car::*;
pub use esp::*;
pub use res::CarRes;
pub use spec::*;
pub use wheel::*;

use bevy::prelude::SystemSet;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum CarSet {
    Input,
    NeuralNetwork,
    Esp,
}
