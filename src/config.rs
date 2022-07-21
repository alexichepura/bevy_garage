use bevy::prelude::*;
use std::f32::consts::PI;

pub struct Config {
    pub translation: Vec3,
    pub quat: Quat,
    pub hid_car: Option<Entity>,
    pub cars_count: i8,
    pub use_brain: bool,
    pub friction: f32,
    pub restitution: f32,
    pub max_torque: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cars_count: 100,
            translation: Vec3::new(0., 0.8, 0.),
            max_torque: 300.,
            quat: Quat::from_rotation_y(-PI * 0.2),
            use_brain: true,
            hid_car: None,
            restitution: 0.0001,
            friction: 1000.,
        }
    }
}
