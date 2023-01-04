use crate::{car::*, nn::params::*};
use bevy::prelude::*;
// use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

pub fn esp_system(
    mut query: Query<(Entity, &mut Car, &Velocity, &Transform), Changed<Car>>,
    mut wheel_set: ParamSet<(
        Query<
            (
                &Wheel,
                &mut ExternalForce,
                &Transform,
                &Velocity,
                &mut ImpulseJoint,
            ),
            With<WheelFront>,
        >,
        Query<
            (
                &Wheel,
                &mut ExternalForce,
                &Transform,
                &Velocity,
                &mut ImpulseJoint,
            ),
            With<WheelBack>,
        >,
    )>,
    // mut lines: ResMut<DebugLines>,
    // config: Res<Config>,
    time: Res<Time>,
) {
    let d_seconds = time.delta_seconds();
    let max_angle = PI / 4.;
    // let wheel_torque_ray_quat = Quat::from_axis_angle(-Vec3::Y, PI / 2.);

    for (_entity, mut car, velocity, transform) in query.iter_mut() {
        let car_vector = transform.rotation.mul_vec3(Vec3::Z);
        let delta = velocity.linvel.normalize() - car_vector.normalize();
        let car_angle_slip_rad = Vec3::new(delta.x, 0., delta.z).length();
        let moving_forward: bool = car_angle_slip_rad < PI / 2.;
        let braking = match moving_forward {
            true => car.brake > 0.,
            false => car.gas > 0.,
        };
        let car_mps = velocity.linvel.length();
        let car_kmh = car_mps / 1000. * 3600.;
        let torque_speed_x: f32 = match braking {
            true => 2.,
            _ => match car_kmh / SPEED_LIMIT_KMH {
                x if x >= 1. => 0.,
                x => 1. - x,
            },
        };
        let steering_speed_x: f32 = match car_kmh / STEERING_SPEEDLIMIT_KMH {
            x if x >= 1. => 0.,
            x => 1. - x,
        }
        .powi(2);
        let pedal = if moving_forward {
            if braking {
                -car.brake
            } else {
                car.gas
            }
        } else {
            if braking {
                car.gas
            } else {
                -car.brake
            }
        };
        let dir = pedal.signum();
        let is_same_dir = car.prev_dir == dir;
        let car_torque = pedal.abs() * car.wheel_max_torque;
        let prev_torque = if is_same_dir { car.prev_torque } else { 0. };
        let prev_steering = car.prev_steering;
        let (steering, mut torque) = (
            prev_steering + (car.steering - prev_steering) * d_seconds * 4.,
            prev_torque + (car_torque - prev_torque) * d_seconds * 4.,
        );
        car.prev_steering = steering;
        car.prev_torque = torque;
        car.prev_dir = dir;

        torque = dir * torque;

        let angle: f32 = max_angle * steering * (0.1 + 0.9 * steering_speed_x);
        let quat = -Quat::from_axis_angle(Vec3::Y, -angle);
        let torque_vec = Vec3::new(0., torque, 0.);
        let steering_torque_vec = quat.mul_vec3(torque_vec);

        for (_i, wheel_entity) in car.wheels.iter().enumerate() {
            let mut q_front_wheels = wheel_set.p0();
            let wheel_result = q_front_wheels.get_mut(*wheel_entity);
            if let Ok((wheel, mut f, transform, v, mut j)) = wheel_result {
                let radius_vel = v.angvel * wheel.radius;
                let velocity_slip = (radius_vel[0] - v.linvel[2], radius_vel[2] + v.linvel[0]);
                let slip_sq = (velocity_slip.0.powi(2) + velocity_slip.1.powi(2)).sqrt();
                let max_slip = 10.;
                let slip_sq_x: f32 = match slip_sq / max_slip {
                    x if x >= 1. => 0.,
                    x => 1. - x,
                };
                let total_torque = steering_torque_vec * slip_sq_x * torque_speed_x;
                f.torque = (transform.rotation.mul_vec3(total_torque)).into();

                // if config.show_rays {
                //     let start = transform.translation + Vec3::Y * 0.5;
                //     let end = start + wheel_torque_ray_quat.mul_vec3(f.torque) / 100.;
                //     lines.line_colored(start, end, 0.0, Color::VIOLET);
                // }
                j.data.set_local_basis1(quat);
            }

            if let Ok((wheel, mut f, transform, v, mut j)) = wheel_set.p1().get_mut(*wheel_entity) {
                let radius_vel = v.angvel * wheel.radius;
                let velocity_slip = (radius_vel[0] - v.linvel[2], radius_vel[2] + v.linvel[0]);
                let slip_sq = (velocity_slip.0.powi(2) + velocity_slip.1.powi(2)).sqrt();
                let max_slip = 10.;
                let slip_sq_x: f32 = match slip_sq / max_slip {
                    x if x >= 1. => 0.,
                    x => 1. - x,
                };
                let total_torque = torque_vec * slip_sq_x * torque_speed_x;
                f.torque = (transform.rotation.mul_vec3(total_torque)).into();

                // if config.show_rays {
                //     let start = transform.translation + Vec3::Y * 0.5;
                //     let end = start + wheel_torque_ray_quat.mul_vec3(f.torque) / 100.;
                //     lines.line_colored(start, end, 0.0, Color::VIOLET);
                // }
                let quat_back = -Quat::from_axis_angle(Vec3::Y, 0.);
                j.data.set_local_basis1(quat_back);
            }
        }
    }
}
