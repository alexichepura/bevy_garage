use crate::car::{CAR_TRAINING_GROUP, STATIC_GROUP};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Wheel {
    pub is_front: bool,
    pub is_left: bool,
    pub radius: f32,
    pub half_width: f32,
}

pub fn spawn_wheel(
    commands: &mut Commands,
    wheel_gl: &Handle<Scene>,
    wheel: Wheel,
    car_transform: Transform,
    anchor: Vec3,
) -> Entity {
    let wheel_border_radius = 0.05;
    let diameter = wheel.radius * 2.;
    let width = wheel.half_width * 2.;

    let transform = Transform::from_translation(
        car_transform.translation + car_transform.rotation.mul_vec3(anchor),
    )
    .with_rotation(Quat::from_axis_angle(Vec3::Y, PI))
    .with_scale(Vec3::new(diameter, width, diameter));

    let wheel_id = commands
        .spawn((
            Name::new("wheel"),
            Collider::round_cylinder(
                wheel.half_width - wheel_border_radius,
                wheel.radius - wheel_border_radius,
                wheel_border_radius,
            ),
            wheel,
            SceneBundle {
                scene: wheel_gl.clone(),
                transform,
                ..default()
            },
            (
                ColliderMassProperties::MassProperties(MassProperties {
                    local_center_of_mass: Vec3::ZERO,
                    mass: 15.,
                    principal_inertia: Vec3::ONE * 0.3,
                    ..default()
                }),
                CollisionGroups::new(CAR_TRAINING_GROUP, STATIC_GROUP),
                Damping {
                    linear_damping: 0.05,
                    angular_damping: 0.05,
                },
                Friction {
                    combine_rule: CoefficientCombineRule::Average,
                    coefficient: 5.0,
                    ..default()
                },
                Restitution::coefficient(0.),
            ),
            (
                Ccd::enabled(),
                ColliderScale::Absolute(Vec3::ONE),
                ExternalForce::default(),
                ExternalImpulse::default(),
                RigidBody::Dynamic,
                Sleeping::disabled(),
                Velocity::zero(),
            ),
        ))
        .id();

    wheel_id
}
