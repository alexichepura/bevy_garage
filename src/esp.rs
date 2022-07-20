use crate::car::*;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

pub fn car_change_detection_system(
    query: Query<(Entity, &Car, &Velocity, &Transform), Changed<Car>>,
    mut front: Query<(&mut MultibodyJoint, With<WheelFront>)>,
    mut wheel_set: ParamSet<(
        Query<(&Wheel, &mut ExternalForce, &Transform, &Velocity), With<WheelFront>>,
        Query<(&mut ExternalForce, &Transform, &Velocity), With<WheelBack>>,
    )>,
) {
    for (_entity, car, velocity, transform) in query.iter() {
        let torque: f32;

        let car_vector = transform.rotation.mul_vec3(Vec3::Z);
        let delta = velocity.linvel.normalize() - car_vector.normalize();
        let car_angle_slip_rad = Vec3::new(delta.x, 0., delta.z).length();
        let mut forward: bool = true;
        if car_angle_slip_rad > 1. {
            forward = false;
        }

        let break_max_torque = car.wheel_max_torque * 2.;
        if forward {
            if car.brake > 0. {
                torque = -car.brake * break_max_torque;
            } else {
                torque = car.gas * car.wheel_max_torque;
            }
        } else {
            if car.brake > 0. {
                torque = -car.brake * car.wheel_max_torque;
            } else {
                torque = car.gas * break_max_torque;
            }
        }

        let steering_speed_x: f32 = match velocity.linvel.length() / 10. {
            x if x >= 1. => 0.,
            x => 1. - x,
        };

        let max_angle = PI / 4.;
        let steer_speed_sq = steering_speed_x.powi(2);
        let angle: f32 = max_angle * car.steering * (0.1 + 0.9 * steer_speed_sq);
        let quat = Quat::from_axis_angle(Vec3::Y, -angle);
        let torque_vec = Vec3::new(0., torque, 0.);
        let steering_torque_vec = quat.mul_vec3(torque_vec);
        let axis = quat.mul_vec3(Vec3::X);

        for wheel_entity in car.wheels.iter() {
            let mut q_front_wheels = wheel_set.p0();
            let wheel_result = q_front_wheels.get_mut(*wheel_entity);
            if let Ok((wheel, mut forces, transform, velocity)) = wheel_result {
                let radius_vel = velocity.angvel * wheel.radius;
                let velocity_slip = (
                    radius_vel[0] - velocity.linvel[2],
                    radius_vel[2] + velocity.linvel[0],
                );
                let slip_sq = (velocity_slip.0.powi(2) + velocity_slip.1.powi(2)).sqrt();
                let max_slip = 0.4; // TODO is too much for slip control
                let slip_sq_x: f32 = match slip_sq / max_slip {
                    x if x >= 1. => {
                        // println!("max_slip {max_slip}, slip {slip_sq}");
                        0.
                    }
                    x => 1. - x,
                };
                forces.torque =
                    (transform.rotation.mul_vec3(steering_torque_vec * slip_sq_x)).into();
                // println!("slip adjust {slip_sq_x}");
                // println!(
                //     "slip {velocity_slip:?} {:?} {:?}",
                //     (radius_vel * 10.).round() / 10.,
                //     (velocity.linvel * 10.).round() / 10.,
                // );
            }

            // let mut q_back_wheels = wheel_set.p1();
            // if let Ok((mut forces, transform, _velocity)) = q_back_wheels.get_mut(*wheel_entity) {
            //     forces.torque = (transform.rotation.mul_vec3(torque_vec)).into();
            // }
            if let Ok((mut joint, _)) = front.get_mut(*wheel_entity) {
                joint.data.set_local_axis1(axis);
            }
        }
    }
}
