use bevy::prelude::*;
use parry3d::shape::Polyline;
use std::f32::consts::PI;

pub struct Config {
    pub translation: Vec3,
    pub quat: Quat,
    pub cars_count: usize,
    pub show_rays: bool,
    pub use_brain: bool,
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
            show_rays: true,
            max_torque: 400.,
            max_toi: 50.,
            translation: Vec3::new(0., 0.7, 0.),
            quat: Quat::from_rotation_y(-PI * 0.225),
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
