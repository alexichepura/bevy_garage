use crate::{config::*, wheel::spawn_wheel};
use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::*,
    rapier::prelude::{JointAxesMask, JointAxis},
};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_8, PI};

pub const SENSOR_COUNT: usize = 31;

pub type JointType = ImpulseJoint;
// https://github.com/alexichepura/bevy_garage/issues/23
// https://github.com/dimforge/bevy_rapier/issues/196
// pub type JointType = MultibodyJoint;

pub const FRAC_PI_16: f32 = FRAC_PI_8 / 2.;

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
    pub sensor_config: [(Vec3, Quat); SENSOR_COUNT],
    pub sensor_inputs: Vec<f32>,
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
    let size = CarSize {
        hw: 1.,
        hh: 0.35,
        hl: 2.2,
    };
    let wheel_r: f32 = 0.35;
    let wheel_hw: f32 = 0.17;
    let ride_height = 0.06;
    let shift = Vec3::new(
        size.hw - wheel_hw - 0.1,
        -size.hh + wheel_r - ride_height,
        size.hl - wheel_r - 0.5,
    );
    let car_anchors: [Vec3; 4] = [
        Vec3::new(shift.x, shift.y, shift.z),
        Vec3::new(-shift.x, shift.y, shift.z),
        Vec3::new(shift.x, shift.y, -shift.z),
        Vec3::new(-shift.x, shift.y, -shift.z),
    ];

    let mut wheels: Vec<Entity> = vec![];
    let mut joints: Vec<GenericJoint> = vec![];
    for i in 0..4 {
        let (is_front, is_left): (bool, bool) = match i {
            0 => (true, false),
            1 => (true, true),
            2 => (false, false),
            _ => (false, true),
        };
        let joint = GenericJointBuilder::new(
            JointAxesMask::ANG_Y | JointAxesMask::ANG_Z | JointAxesMask::X | JointAxesMask::Z,
        )
        .local_axis1(Vec3::X)
        .local_axis2(match is_left {
            true => -Vec3::Y,
            false => Vec3::Y,
        })
        .local_basis1(Quat::from_axis_angle(Vec3::Y, 0.)) // hackfix, prevents jumping on collider edges
        .local_anchor1(car_anchors[i])
        .local_anchor2(Vec3::ZERO)
        .set_motor(JointAxis::Y, 0., 0., 1e6, 1e3)
        .build();
        joints.push(joint);

        let wheel_transform = Transform::from_translation(
            transform.translation + transform.rotation.mul_vec3(car_anchors[i]),
        )
        .with_rotation(Quat::from_axis_angle(Vec3::Y, PI))
        .with_scale(Vec3::new(wheel_r * 2., wheel_hw * 2., wheel_r * 2.));

        let wheel_id = spawn_wheel(
            commands,
            wheel_gl,
            wheel_transform,
            wheel_r,
            wheel_hw,
            is_front,
            is_left,
        );
        wheels.push(wheel_id);
    }

    let car_border_radius = 0.1;
    let car_id = commands
        .spawn((
            Name::new("car"),
            Car {
                size: size.clone(),
                wheels: wheels.clone(),
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
                Collider::round_cuboid(
                    size.hw - car_border_radius,
                    size.hh - car_border_radius,
                    size.hl - car_border_radius,
                    car_border_radius,
                ),
                ColliderMassProperties::MassProperties(MassProperties {
                    local_center_of_mass: Vec3::new(0., -size.hh, 0.),
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
        .id();

    if is_hid {
        commands.entity(car_id).insert(HID);
    }
    for (i, wheel_id) in wheels.iter().enumerate() {
        commands
            .entity(*wheel_id)
            .insert(JointType::new(car_id, joints[i]));
    }
    println!("spawn_car: {car_id:?}");
    return car_id;
}

pub fn car_sensor_system(
    rapier_context: Res<RapierContext>,
    config: Res<CarConfig>,
    mut q_car: Query<(&mut Car, &Transform), With<Car>>,
    #[cfg(feature = "debug_lines")] mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
) {
    let sensor_filter = QueryFilter::<'_>::exclude_dynamic().exclude_sensors();
    let dir = Vec3::Z * config.max_toi;
    for (mut car, t) in q_car.iter_mut() {
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
                rapier_context.cast_ray(ray_pos, ray_dir, config.max_toi, false, sensor_filter)
            {
                hit_points[i] = ray_pos + ray_dir * toi;
                if toi > 0. {
                    inputs[i] = 1. - toi / config.max_toi;
                    #[cfg(feature = "debug_lines")]
                    if config.show_rays {
                        lines.line_colored(
                            ray_pos,
                            hit_points[i],
                            0.0,
                            Color::rgba(0.5, 0.3, 0.3, 0.5),
                        );
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
