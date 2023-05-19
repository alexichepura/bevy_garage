use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::car::{CAR_TRAINING_GROUP, STATIC_GROUP};

#[derive(Component)]
pub struct Wheel {
    pub radius: f32,
    pub width: f32,
    pub is_front: bool,
    pub is_left: bool,
}

pub fn spawn_wheel(
    commands: &mut Commands,
    wheel_gl: &Handle<Scene>,
    transform: Transform,
    wheel_r: f32,
    wheel_hw: f32,
    is_front: bool,
    is_left: bool,
) -> Entity {
    let wheel_border_radius = 0.05;

    let wheel_id = commands
        .spawn((
            Name::new("wheel"),
            Wheel {
                radius: wheel_r,
                width: wheel_hw * 2.,
                is_front,
                is_left,
            },
            SceneBundle {
                scene: wheel_gl.clone(),
                transform,
                ..default()
            },
            (
                Collider::round_cylinder(
                    wheel_hw - wheel_border_radius,
                    wheel_r - wheel_border_radius,
                    wheel_border_radius,
                ),
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
