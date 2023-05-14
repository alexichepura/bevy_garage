use bevy::prelude::*;

#[derive(Resource)]
pub struct Config {
    pub cars_count: usize,
}
impl Default for Config {
    fn default() -> Self {
        Self { cars_count: 1 }
    }
}
