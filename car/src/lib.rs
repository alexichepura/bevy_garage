pub mod car;
pub mod config;
pub mod spawn;

use bevy::prelude::SystemSet;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum CarSet {
    Input,
    Brain,
    Esp,
}
