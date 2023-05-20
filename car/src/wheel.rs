use crate::{CAR_TRAINING_GROUP, STATIC_GROUP};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Wheel {
    pub radius: f32,
    pub width: f32,
    pub front: bool,
    pub left: bool,
    pub border_radius: f32,
}

impl Wheel {
    pub fn new(radius: f32, width: f32, front: bool, left: bool) -> Self {
        Self {
            radius,
            width,
            front,
            left,
            border_radius: 0.05,
        }
    }
}

pub fn spawn_wheel(
    commands: &mut Commands,
    wheel_gl: &Handle<Scene>,
    wheel: Wheel,
    translation: Vec3,
) -> Entity {
    let diameter = wheel.radius * 2.;

    let transform = Transform::from_translation(translation)
        .with_rotation(Quat::from_axis_angle(Vec3::Y, PI))
        .with_scale(Vec3::new(diameter, wheel.width, diameter));

    let collider = Collider::round_cylinder(
        wheel.width / 2. - wheel.border_radius,
        wheel.radius - wheel.border_radius,
        wheel.border_radius,
    );

    let wheel_id = commands
        .spawn((
            Name::new("wheel"),
            wheel,
            SceneBundle {
                scene: wheel_gl.clone(),
                transform,
                ..default()
            },
            (
                collider,
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
