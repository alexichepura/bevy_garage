use bevy::prelude::{Component, Vec3};
use std::f32::consts::FRAC_PI_4;

#[derive(Debug, Clone)]
pub struct CarSize {
    pub hw: f32,
    pub hh: f32,
    pub hl: f32,
}

#[derive(Component, Debug)]
pub struct CarSpec {
    // geometry
    pub size: CarSize,
    pub anchors: [Vec3; 4],
    pub wheel_radius: f32,
    pub wheel_half_width: f32,
    // drive
    pub wheel_max_torque: f32,
    pub wheel_max_angle: f32,
    pub speed_limit: f32,
    pub steering_speed_limit: f32,
}

impl Default for CarSpec {
    fn default() -> Self {
        const SPEED_LIMIT_KMH: f32 = 300.;
        const SPEED_LIMIT_MPS: f32 = SPEED_LIMIT_KMH * 1000. / 3600.;
        const STEERING_SPEEDLIMIT_KMH: f32 = 270.;
        const STEERING_SPEEDLIMIT_MPS: f32 = STEERING_SPEEDLIMIT_KMH * 1000. / 3600.;

        let ride_height = 0.06;
        let wheel_radius: f32 = 0.35;
        let wheel_half_width: f32 = 0.17;

        let size = CarSize {
            hw: 1.,
            hh: 0.35,
            hl: 2.2,
        };

        let shift = Vec3::new(
            size.hw - wheel_half_width - 0.1,
            -size.hh + wheel_radius - ride_height,
            size.hl - wheel_radius - 0.5,
        );

        let anchors: [Vec3; 4] = [
            Vec3::new(shift.x, shift.y, shift.z),
            Vec3::new(-shift.x, shift.y, shift.z),
            Vec3::new(shift.x, shift.y, -shift.z),
            Vec3::new(-shift.x, shift.y, -shift.z),
        ];

        Self {
            size,
            speed_limit: SPEED_LIMIT_MPS,
            steering_speed_limit: STEERING_SPEEDLIMIT_MPS,
            wheel_max_torque: 1200.,
            wheel_max_angle: FRAC_PI_4,
            anchors,
            wheel_radius,
            wheel_half_width,
        }
    }
}
