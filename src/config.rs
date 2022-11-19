use bevy::prelude::*;
use parry3d::shape::Polyline;
use rand::Rng;
use std::f32::consts::PI;

use crate::nn::params::{CARS_COUNT, MAX_TOI, MAX_TORQUE};

#[derive(Resource)]
pub struct Config {
    pub translation: Vec3,
    pub quat: Quat,
    pub cars_count: usize,
    pub show_rays: bool,
    pub use_brain: bool,
    pub max_torque: f32,
    pub max_toi: f32,
    pub hid_car: Option<Entity>,
    pub polyline: Option<Polyline>,
    pub segment_i: u32,
    pub segment_m: f32,
    pub meters: Vec<f32>,
    pub meters_shift: f32,
    pub track_length: f32,
    pub car_scene: Option<Handle<Scene>>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            cars_count: CARS_COUNT,
            use_brain: false,
            show_rays: true,
            max_torque: MAX_TORQUE,
            max_toi: MAX_TOI,
            translation: Vec3::new(0., 0.1, 0.),
            quat: Quat::from_rotation_y(-PI * 0.225),
            hid_car: None,
            polyline: None,
            segment_i: 0,
            segment_m: 0.,
            meters: vec![],
            meters_shift: 0.,
            track_length: 0.,
            car_scene: None,
        }
    }
}
impl Config {
    pub fn get_transform_by_index(&self, i: usize) -> (Transform, f32) {
        let meters = i as f32 * self.track_length / self.cars_count as f32;
        let (translate, quat) = self.get_transform_by_meter(meters);
        let transform = Transform::from_translation(translate).with_rotation(quat);
        return (transform, meters);
    }
    pub fn get_transform_random(&self) -> (Transform, f32) {
        let mut rng = rand::thread_rng();
        let meters = rng.gen_range(0.0..self.track_length);
        let (translate, quat) = self.get_transform_by_meter(meters);
        let transform = Transform::from_translation(translate).with_rotation(quat);
        return (transform, meters);
    }
    pub fn get_transform_by_meter(&self, meters: f32) -> (Vec3, Quat) {
        let polyline = self.polyline.as_ref().unwrap();
        let mut seg_meters = 0.;
        let mut shift = meters + self.meters_shift;
        if shift > self.track_length {
            shift = shift - self.track_length * (shift / self.track_length).floor();
        }

        for segment in polyline.segments() {
            let new_seg_meters: f32 = seg_meters + segment.length();
            if new_seg_meters < shift {
                seg_meters = new_seg_meters;
            } else {
                let a: Vec3 = segment.a.into();
                let dir: Vec3 = segment.direction().unwrap().into();
                let mut pos: Vec3 = a + dir * (shift - seg_meters);
                pos.y = 0.4;

                return (pos, Quat::from_rotation_arc(Vec3::Z, dir));
            }
        }
        panic!();
    }
}
