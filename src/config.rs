use bevy::prelude::*;
use rapier3d::prelude::Polyline;
use std::f32::consts::PI;

pub struct Config {
    pub translation: Vec3,
    pub quat: Quat,
    pub cars_count: u16,
    pub use_brain: bool,
    pub friction: f32,
    pub restitution: f32,
    pub max_torque: f32,
    pub max_toi: f32,
    pub hid_car: Option<Entity>,
    pub camera_follow: Option<Entity>,
    pub polyline: Option<Polyline>,
    pub segment_i: u32,
    pub segment_m: f32,
    pub meters: Vec<f32>,
    pub meters_shift: f32,
    pub meters_total: f32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cars_count: 1,
            use_brain: false,
            max_torque: 1000.,
            max_toi: 50.,
            translation: Vec3::new(0., 0.9, 0.),
            quat: Quat::from_rotation_y(-PI * 0.225),
            restitution: 0.0,
            friction: 100.,
            hid_car: None,
            camera_follow: None,
            polyline: None,
            segment_i: 0,
            segment_m: 0.,
            meters: vec![],
            meters_shift: 0.,
            meters_total: 0.,
        }
    }
}
