use crate::car::*;
use bevy::prelude::*;
// use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

pub fn esp_system(
    query: Query<(Entity, &Car, &Velocity, &Transform), Changed<Car>>,
    mut front: Query<(&mut MultibodyJoint, With<WheelFront>)>,
    mut wheel_set: ParamSet<(
        Query<(&Wheel, &mut ExternalForce, &Transform, &Velocity), With<WheelFront>>,
        Query<(&Wheel, &mut ExternalForce, &Transform, &Velocity), With<WheelBack>>,
    )>,
    // mut lines: ResMut<DebugLines>,
    // config: Res<Config>,
) {
    let max_angle = PI / 4.;
    // let wheel_torque_ray_quat = Quat::from_axis_angle(-Vec3::Y, PI / 2.);

    for (_entity, car, velocity, transform) in query.iter() {
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
            true => 3.,
            _ => match car_kmh / 70. {
                x if x >= 1. => 0.,
                x => 1. - x,
            },
        };
        let steering_speed_x: f32 = match car_kmh / 50. {
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
        let torque: f32 = pedal * car.wheel_max_torque;
        let angle: f32 = max_angle * car.steering * (0.1 + 0.9 * steering_speed_x);
        let quat = Quat::from_axis_angle(Vec3::Y, -angle);
        let torque_vec = Vec3::new(0., torque, 0.);
        let steering_torque_vec = quat.mul_vec3(torque_vec);

        for (_i, wheel_entity) in car.wheels.iter().enumerate() {
            let mut q_front_wheels = wheel_set.p0();
            let wheel_result = q_front_wheels.get_mut(*wheel_entity);
            if let Ok((wheel, mut f, transform, v)) = wheel_result {
                let radius_vel = v.angvel * wheel.radius;
                let velocity_slip = (radius_vel[0] - v.linvel[2], radius_vel[2] + v.linvel[0]);
                let slip_sq = (velocity_slip.0.powi(2) + velocity_slip.1.powi(2)).sqrt();
                let max_slip = 0.5;
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
            }

            if let Ok((wheel, mut f, transform, v)) = wheel_set.p1().get_mut(*wheel_entity) {
                let radius_vel = v.angvel * wheel.radius;
                let velocity_slip = (radius_vel[0] - v.linvel[2], radius_vel[2] + v.linvel[0]);
                let slip_sq = (velocity_slip.0.powi(2) + velocity_slip.1.powi(2)).sqrt();
                let max_slip = 0.3;
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
            }
            if let Ok((mut joint, _)) = front.get_mut(*wheel_entity) {
                let axis = quat.mul_vec3(Vec3::X);
                joint.data.set_local_axis1(axis);
            }
        }
    }
}
