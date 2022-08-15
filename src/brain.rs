use crate::car::*;
use crate::config::Config;
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier3d::prelude::*;

pub fn car_brain_system(
    rapier_context: Res<RapierContext>,
    config: Res<Config>,
    mut q_car: Query<(Entity, &mut Car, &Children, &Velocity), With<Car>>,
    q_near: Query<(&GlobalTransform, With<SensorNear>)>,
    q_far: Query<(&GlobalTransform, With<SensorFar>)>,
    mut ray_set: ParamSet<(
        Query<(&mut Transform, With<RayOrig>)>,
        Query<(&mut Transform, With<RayDir>)>,
        Query<(&mut Transform, With<RayHit>)>,
    )>,
    mut lines: ResMut<DebugLines>,
) {
    let sensor_filter = QueryFilter::new().exclude_dynamic().exclude_sensors();

    let e_hid_car = config.hid_car.unwrap();
    for (e, mut car, children, v) in q_car.iter_mut() {
        let is_hid_car = e == e_hid_car;
        let mut origins: Vec<Vec3> = Vec::new();
        let mut dirs: Vec<Vec3> = Vec::new();

        for &child in children.iter() {
            if let Ok((gtrf, _)) = q_near.get(child) {
                origins.push(gtrf.translation());
            }
            if let Ok((gtrf, _)) = q_far.get(child) {
                dirs.push(gtrf.translation());
            }
        }

        let mut inputs: Vec<f32> = vec![0.; SENSOR_COUNT];
        let mut hit_points: Vec<Vec3> = vec![Vec3::ZERO; SENSOR_COUNT];
        let solid = false;
        for (i, &ray_dir_pos) in dirs.iter().enumerate() {
            let ray_pos = origins[i];
            if is_hid_car {
                lines.line_colored(
                    ray_pos,
                    ray_dir_pos,
                    0.0,
                    Color::rgba(0.25, 0.88, 0.82, 0.1),
                );
            }
            let ray_dir = (ray_dir_pos - ray_pos).normalize();
            rapier_context.intersections_with_ray(
                ray_pos,
                ray_dir,
                config.max_toi,
                solid,
                sensor_filter,
                |_entity, intersection| {
                    let toi = intersection.toi;
                    hit_points[i] = intersection.point;
                    if toi > 0. {
                        inputs[i] = 1. - toi / config.max_toi;
                        if config.show_rays {
                            lines.line_colored(
                                ray_pos,
                                intersection.point,
                                0.0,
                                Color::rgba(0.98, 0.5, 0.45, 0.9),
                            );
                        }
                    } else {
                        inputs[i] = 0.;
                    }
                    false
                },
            );
        }
        if is_hid_car {
            for (i, (mut trf, _)) in ray_set.p0().iter_mut().enumerate() {
                trf.translation = origins[i];
            }
            for (i, (mut trf, _)) in ray_set.p1().iter_mut().enumerate() {
                trf.translation = dirs[i];
            }
            for (i, (mut trf, _)) in ray_set.p2().iter_mut().enumerate() {
                trf.translation = hit_points[i];
            }
            // print_float_arr("inputs", &inputs);
        }
        inputs.push(v.linvel.length());
        car.sensor_inputs = inputs.clone();
    }
}
