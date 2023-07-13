use crate::{CarRes, CarSize};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_8, PI};

pub const FRAC_PI_16: f32 = FRAC_PI_8 / 2.;
pub const SENSOR_COUNT: usize = 31;

#[derive(Component, Debug)]
pub struct CarSensors {
    pub max_toi: f32,
    pub sensor_config: [(Vec3, Quat); SENSOR_COUNT],
    pub sensor_inputs: Vec<f32>,
}

impl CarSensors {
    pub fn new(car_size: &CarSize) -> Self {
        let (hw, hl) = (car_size.hw, car_size.hl);
        Self {
            max_toi: 100.,
            sensor_inputs: vec![0.; SENSOR_COUNT],
            sensor_config: [
                // front
                (hw, hl, 0.),
                (0., hl, 0.),
                (-hw, hl, 0.),
                (hw, hl, FRAC_PI_16 / 2.),
                (-hw, hl, -FRAC_PI_16 / 2.),
                (hw, hl, FRAC_PI_16),
                (-hw, hl, -FRAC_PI_16),
                (hw, hl, FRAC_PI_16 + FRAC_PI_16 / 2.),
                (-hw, hl, -FRAC_PI_16 - FRAC_PI_16 / 2.),
                (hw, hl, FRAC_PI_8),
                (-hw, hl, -FRAC_PI_8),
                (hw, hl, FRAC_PI_8 + FRAC_PI_16),
                (-hw, hl, -FRAC_PI_8 - FRAC_PI_16),
                (hw, hl, FRAC_PI_4),
                (-hw, hl, -FRAC_PI_4),
                // front > PI/4
                (hw, hl, FRAC_PI_4 + FRAC_PI_16),
                (-hw, hl, -FRAC_PI_4 - FRAC_PI_16),
                (hw, hl, FRAC_PI_4 + FRAC_PI_8),
                (-hw, hl, -FRAC_PI_4 - FRAC_PI_8),
                (hw, hl, FRAC_PI_4 + FRAC_PI_8 + FRAC_PI_16),
                (-hw, hl, -FRAC_PI_4 - FRAC_PI_8 - FRAC_PI_16),
                (hw, hl, FRAC_PI_2),
                (-hw, hl, -FRAC_PI_2),
                // side
                (hw, 0., FRAC_PI_2),
                (-hw, 0., -FRAC_PI_2),
                // back
                (hw, -hl, PI),
                (-hw, -hl, PI),
                (hw, -hl, PI - FRAC_PI_4),
                (-hw, -hl, PI + FRAC_PI_4),
                (hw, -hl, PI - FRAC_PI_2),
                (-hw, -hl, PI + FRAC_PI_2),
            ]
            .map(|(w, l, r)| (Vec3::new(w, -0.1, l), Quat::from_rotation_y(r))),
        }
    }
}

pub fn sensor_system(
    rapier_context: Res<RapierContext>,
    config: Res<CarRes>,
    mut q_car: Query<(&mut CarSensors, &Transform)>,
    mut gizmos: Gizmos,
) {
    let sensor_filter = QueryFilter::<'_>::exclude_dynamic().exclude_sensors();
    for (mut car, t) in q_car.iter_mut() {
        let dir = Vec3::Z * car.max_toi;
        let mut origins: Vec<Vec3> = Vec::new();
        let mut dirs: Vec<Vec3> = Vec::new();
        for a in 0..SENSOR_COUNT {
            let (pos, far_quat) = car.sensor_config[a];
            let origin = t.translation + t.rotation.mul_vec3(pos);
            origins.push(origin);
            let mut dir_vec = t.rotation.mul_vec3(far_quat.mul_vec3(dir));
            dir_vec.y = 0.;
            dirs.push(origin + dir_vec);
        }

        let mut inputs: Vec<f32> = vec![0.; SENSOR_COUNT];
        let mut hit_points: Vec<Vec3> = vec![Vec3::ZERO; SENSOR_COUNT];
        for (i, &ray_dir_pos) in dirs.iter().enumerate() {
            let ray_pos = origins[i];
            let ray_dir = (ray_dir_pos - ray_pos).normalize();

            if let Some((_e, toi)) =
                rapier_context.cast_ray(ray_pos, ray_dir, car.max_toi, false, sensor_filter)
            {
                hit_points[i] = ray_pos + ray_dir * toi;
                if toi > 0. {
                    inputs[i] = 1. - toi / car.max_toi;
                    if config.show_rays {
                        gizmos.line(ray_pos, hit_points[i], Color::rgba(0.5, 0.3, 0.3, 0.5));
                    }
                } else {
                    inputs[i] = 0.;
                }
            }
        }
        car.sensor_inputs = inputs;
        // println!("inputs {:#?}", car.sensor_inputs);
    }
}
