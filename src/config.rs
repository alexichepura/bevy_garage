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
    pub polyline: Option<Polyline>,
    pub segment_i: u32,
    pub segment_m: f32,
    pub meters: Vec<f32>,
    pub meters_shift: f32,
    pub track_length: f32,
}
impl Config {
    pub fn get_transform_by_index(&self, i: usize) -> (Vec3, Quat) {
        return self.get_transform_by_meter(i as f32 * 250.);
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
                println!("{shift:.1} {new_seg_meters:.1}");
                seg_meters = new_seg_meters;
            } else {
                let a: Vec3 = segment.a.into();
                let dir: Vec3 = segment.direction().unwrap().into();
                let mut pos: Vec3 = a + dir * (shift - seg_meters);
                pos.y = self.translation.y;

                return (
                    self.translation + pos,
                    Quat::from_rotation_arc(Vec3::Z, dir),
                );
            }
        }
        panic!();
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            cars_count: 20,
            use_brain: false,
            show_rays: true,
            max_torque: 800.,
            max_toi: 50.,
            translation: Vec3::new(0., 0.8, 0.),
            quat: Quat::from_rotation_y(-PI * 0.225),
            hid_car: None,
            polyline: None,
            segment_i: 0,
            segment_m: 0.,
            meters: vec![],
            meters_shift: 0.,
            track_length: 0.,
        }
    }
}
