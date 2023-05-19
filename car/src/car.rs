use crate::{config::*, joint::build_joint, wheel::spawn_wheel, Wheel, WheelJoint};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use std::f32::consts::FRAC_PI_4;

#[derive(Component)]
pub struct HID;

#[derive(Debug, Clone)]
pub struct CarSize {
    pub hw: f32,
    pub hh: f32,
    pub hl: f32,
}

const SPEED_LIMIT_KMH: f32 = 300.;
const SPEED_LIMIT_MPS: f32 = SPEED_LIMIT_KMH * 1000. / 3600.;
const STEERING_SPEEDLIMIT_KMH: f32 = 270.;
const STEERING_SPEEDLIMIT_MPS: f32 = STEERING_SPEEDLIMIT_KMH * 1000. / 3600.;

#[derive(Component, Debug)]
pub struct Car {
    pub size: CarSize,
    pub speed_limit: f32,
    pub steering_speed_limit: f32,
    pub gas: f32,
    pub brake: f32,
    pub steering: f32,
    pub wheels: Vec<Entity>,
    pub wheel_max_torque: f32,
    pub wheel_max_angle: f32,
    pub init_transform: Transform,
    pub prev_steering: f32,
    pub prev_torque: f32,
    pub prev_dir: f32,
}
impl Default for Car {
    fn default() -> Self {
        let hw = 1.;
        let hh = 0.35;
        let hl = 2.2;
        Self {
            size: CarSize { hw, hh, hl },
            speed_limit: SPEED_LIMIT_MPS,
            steering_speed_limit: STEERING_SPEEDLIMIT_MPS,
            gas: 0.,
            brake: 0.,
            steering: 0.,
            prev_steering: 0.,
            prev_torque: 0.,
            prev_dir: 0.,
            wheels: Vec::new(),
            wheel_max_torque: 1000.,
            wheel_max_angle: FRAC_PI_4,
            init_transform: Transform::default(),
        }
    }
}

impl Car {
    pub fn despawn_wheels(&mut self, commands: &mut Commands) {
        for e in self.wheels.iter() {
            commands.entity(*e).despawn_recursive();
        }
    }
}

pub const STATIC_GROUP: Group = Group::GROUP_1;
pub const CAR_TRAINING_GROUP: Group = Group::GROUP_10;
pub fn car_start_system(mut config: ResMut<CarConfig>, asset_server: Res<AssetServer>) {
    let wheel_gl: Handle<Scene> = asset_server.load("wheelRacing.glb#Scene0");
    config.wheel_scene = Some(wheel_gl.clone());
    let car_gl: Handle<Scene> = asset_server.load("car-race.glb#Scene0");
    config.car_scene = Some(car_gl.clone());
}

pub fn spawn_car(
    commands: &mut Commands,
    car_gl: &Handle<Scene>,
    wheel_gl: &Handle<Scene>,
    is_hid: bool,
    transform: Transform,
    max_torque: f32,
) -> Entity {
    let car_size = CarSize {
        hw: 1.,
        hh: 0.35,
        hl: 2.2,
    };
    let ride_height = 0.06;
    let wheel_radius: f32 = 0.35;
    let wheel_half_width: f32 = 0.17;
    let shift = Vec3::new(
        car_size.hw - wheel_half_width - 0.1,
        -car_size.hh + wheel_radius - ride_height,
        car_size.hl - wheel_radius - 0.5,
    );
    let anchors: [Vec3; 4] = [
        Vec3::new(shift.x, shift.y, shift.z),
        Vec3::new(-shift.x, shift.y, shift.z),
        Vec3::new(shift.x, shift.y, -shift.z),
        Vec3::new(-shift.x, shift.y, -shift.z),
    ];
    let wheel_front_left: [(bool, bool); 4] =
        [(true, false), (true, true), (false, false), (false, true)];

    let mut wheels: Vec<Entity> = vec![];
    let mut joints: Vec<GenericJoint> = vec![];
    for i in 0..4 {
        let (is_front, is_left) = wheel_front_left[i];
        let anchor = anchors[i];
        let wheel_id = spawn_wheel(
            commands,
            wheel_gl,
            Wheel {
                is_front,
                is_left,
                radius: wheel_radius,
                half_width: wheel_half_width,
            },
            transform.translation + transform.rotation.mul_vec3(anchor),
        );
        let joint = build_joint(anchor, is_left);
        joints.push(joint);
        wheels.push(wheel_id);
    }

    let car_id = spawn_car_body(
        commands,
        car_gl,
        car_size,
        transform,
        max_torque,
        wheels.clone(),
    );

    if is_hid {
        commands.entity(car_id).insert(HID);
    }
    for (i, wheel_id) in wheels.iter().enumerate() {
        commands
            .entity(*wheel_id)
            .insert(WheelJoint::new(car_id, joints[i]));
    }
    println!("spawn_car: {car_id:?}");
    return car_id;
}

pub fn spawn_car_body(
    commands: &mut Commands,
    car_gl: &Handle<Scene>,
    size: CarSize,
    transform: Transform,
    max_torque: f32,
    wheels: Vec<Entity>,
) -> Entity {
    let car_border_radius = 0.1;
    let local_center_of_mass = Vec3::new(0., -size.hh, 0.);
    let collider = Collider::round_cuboid(
        size.hw - car_border_radius,
        size.hh - car_border_radius,
        size.hl - car_border_radius,
        car_border_radius,
    );
    commands
        .spawn((
            Name::new("car"),
            Car {
                size,
                wheels,
                wheel_max_torque: max_torque,
                init_transform: transform,
                ..default()
            },
            SceneBundle {
                scene: car_gl.clone(),
                transform,
                ..default()
            },
            (
                collider,
                ColliderMassProperties::MassProperties(MassProperties {
                    local_center_of_mass,
                    mass: 1000.0,
                    principal_inertia: Vec3::new(5000., 5000., 2000.), // https://www.nhtsa.gov/DOT/NHTSA/NRD/Multimedia/PDFs/VRTC/ca/capubs/sae1999-01-1336.pdf
                    ..default()
                }),
                Damping {
                    linear_damping: 0.05,
                    angular_damping: 0.1,
                },
                Friction::coefficient(0.5),
                Restitution::coefficient(0.),
                CollisionGroups::new(CAR_TRAINING_GROUP, STATIC_GROUP),
                ActiveEvents::COLLISION_EVENTS,
                ContactForceEventThreshold(0.1),
            ),
            (
                Ccd::enabled(),
                CollidingEntities::default(),
                ColliderScale::Absolute(Vec3::ONE),
                ExternalForce::default(),
                ReadMassProperties::default(),
                RigidBody::Dynamic,
                Sleeping::disabled(),
                Velocity::zero(),
            ),
        ))
        .id()
}
