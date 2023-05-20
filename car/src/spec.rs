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
    pub size: CarSize,
    pub wheel_radius: f32,
    pub wheel_width: f32,
    pub wheel_mount: [WheelMount; 4],

    pub wheel_max_torque: f32,
    pub wheel_max_angle: f32,
    pub max_speed: f32,
    pub max_steering_speed: f32,
}

impl Default for CarSpec {
    fn default() -> Self {
        let ride_height = 0.06;
        let wheel_radius: f32 = 0.35;
        let wheel_width: f32 = 0.34;

        let size = CarSize {
            hw: 1.,
            hh: 0.35,
            hl: 2.2,
        };

        let shift = Vec3::new(
            size.hw - wheel_width / 2. - 0.1,
            -size.hh + wheel_radius - ride_height,
            size.hl - wheel_radius - 0.5,
        );

        let anchors: [(Vec3, bool, bool); 4] = [
            (Vec3::new(shift.x, shift.y, shift.z), true, false), // front right
            (Vec3::new(-shift.x, shift.y, shift.z), true, true), // front left
            (Vec3::new(shift.x, shift.y, -shift.z), false, false), // rear right
            (Vec3::new(-shift.x, shift.y, -shift.z), false, true), // rear left
        ];

        Self {
            size,
            max_speed: 300. * 1000. / 3600.,
            max_steering_speed: 270. * 1000. / 3600.,
            wheel_max_torque: 1200.,
            wheel_max_angle: FRAC_PI_4,
            wheel_radius,
            wheel_width,
            wheel_mount: anchors.map(|a| WheelMount {
                anchor: a.0,
                front: a.1,
                left: a.2,
            }),
        }
    }
}

#[derive(Debug, Clone)]
pub struct WheelMount {
    pub anchor: Vec3,
    pub front: bool,
    pub left: bool,
}
