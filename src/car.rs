use crate::{config::*, track::*};
use bevy::prelude::*;
use bevy_rapier3d::{
    prelude::*,
    rapier::prelude::{JointAxesMask, JointAxis},
};
use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, FRAC_PI_8, PI};

pub type JointType = ImpulseJoint;
// https://github.com/alexichepura/bevy_garage/issues/23
// https://github.com/dimforge/bevy_rapier/issues/196
// pub type JointType = MultibodyJoint;

pub const FRAC_PI_16: f32 = FRAC_PI_8 / 2.;
pub const SENSOR_COUNT: usize = 31;

#[derive(Component)]
pub struct Wheel {
    pub radius: f32,
    pub width: f32,
}
#[derive(Component)]
pub struct WheelFront;
#[derive(Component)]
pub struct WheelBack;
#[derive(Component)]
pub struct WheelLeft;
#[derive(Component)]
pub struct WheelRight;
// #[derive(Component)]
// pub struct WheelFrontLeft;
// #[derive(Component)]
// pub struct WheelFrontRight;
#[derive(Component)]
pub struct HID;

#[derive(Debug, Clone)]
pub struct CarSize {
    pub hw: f32,
    pub hh: f32,
    pub hl: f32,
}

#[derive(Component, Debug)]
pub struct Car {
    pub size: CarSize,
    pub sensor_config: [(Vec3, Quat); SENSOR_COUNT],
    pub sensor_inputs: Vec<f32>,
    pub gas: f32,
    pub brake: f32,
    pub steering: f32,
    pub wheels: Vec<Entity>,
    pub wheel_max_torque: f32,
    pub init_transform: Transform,
    pub reset_at: Option<f64>,

    pub index: usize,
    pub init_meters: f32,
    pub meters: f32,
    pub lap: usize,
    pub line_dir: Vec3,
    pub line_pos: Vec3,
    pub place: usize,

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
            init_transform: Transform::default(),
            reset_at: None,

            index: 0,
            init_meters: 0.,
            meters: 0.,
            place: 0,
            lap: 0,
            line_dir: Vec3::ZERO,
            line_pos: Vec3::ZERO,
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

pub const CAR_TRAINING_GROUP: Group = Group::GROUP_10;
pub fn car_start_system(
    mut commands: Commands,
    mut config: ResMut<Config>,
    asset_server: Res<AssetServer>,
) {
    let wheel_gl: Handle<Scene> = asset_server.load("wheelRacing.glb#Scene0");
    config.wheel_scene = Some(wheel_gl.clone());
    let car_gl: Handle<Scene> = asset_server.load("car-race.glb#Scene0");
    config.car_scene = Some(car_gl.clone());

    for i in 0..config.cars_count {
        let is_hid = i == 0;
        let init_meters = 42.;
        // let (transform, init_meters) = config.get_transform_by_index(i);
        let (translate, quat) = config.get_transform_by_meter(init_meters);
        let transform = Transform::from_translation(translate).with_rotation(quat);
        spawn_car(
            &mut commands,
            &car_gl,
            &wheel_gl,
            is_hid,
            transform,
            i,
            init_meters,
            config.max_torque,
        );
    }
}

pub fn spawn_car(
    commands: &mut Commands,
    car_gl: &Handle<Scene>,
    wheel_gl: &Handle<Scene>,
    is_hid: bool,
    transform: Transform,
    index: usize,
    init_meters: f32,
    max_torque: f32,
) -> Entity {
    let size = CarSize {
        hw: 1.,
        hh: 0.35,
        hl: 2.2,
    };
    let wheel_front_r: f32 = 0.4;
    let wheel_back_r: f32 = 0.401;
    let wheel_front_hw: f32 = 0.2;
    let wheel_back_hw: f32 = wheel_front_hw;
    // let wheel_back_hw: f32 = 0.2;
    let ride_height = 0.08;
    let shift = Vec3::new(
        size.hw - wheel_front_hw - 0.1,
        -size.hh + wheel_front_r - ride_height,
        size.hl - wheel_front_r - 0.5,
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
        let wheel_hw = match is_front {
            true => wheel_front_hw,
            false => wheel_back_hw,
        };
        let wheel_r = match is_front {
            true => wheel_front_r,
            false => wheel_back_r,
        };
        let joint_mask: JointAxesMask = JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z
            | JointAxesMask::X
            // | JointAxesMask::Y
            | JointAxesMask::Z;
        let wheel_axis = match is_left {
            true => -Vec3::Y,
            false => Vec3::Y,
        };
        let joint = GenericJointBuilder::new(joint_mask)
            .local_axis1(Vec3::X)
            .local_axis2(wheel_axis)
            .local_anchor1(car_anchors[i])
            .local_anchor2(Vec3::ZERO)
            .set_motor(JointAxis::Y, 0., 0., 1e6, 1e3)
            .build();
        joints.push(joint);

        let wheel_border_radius = 0.05;
        let wheel_transform = Transform::from_translation(
            transform.translation + transform.rotation.mul_vec3(car_anchors[i]),
        )
        .with_rotation(Quat::from_axis_angle(Vec3::Y, PI))
        .with_scale(Vec3::new(
            wheel_front_r * 2.,
            wheel_front_hw * 2.,
            wheel_front_r * 2.,
        ));
        let wheel_id = commands
            .spawn_empty()
            .insert(Name::new("wheel"))
            .insert(SceneBundle {
                scene: wheel_gl.clone(),
                transform: wheel_transform,
                ..default()
            })
            .insert(Sleeping::disabled())
            .insert(TransformBundle::from(wheel_transform))
            .insert(RigidBody::Dynamic)
            .insert(Ccd::enabled())
            .insert(Velocity::zero())
            .insert(Collider::round_cylinder(
                wheel_hw - wheel_border_radius,
                wheel_r - wheel_border_radius,
                wheel_border_radius,
            ))
            .insert(ColliderScale::Absolute(Vec3::ONE))
            .insert(CollisionGroups::new(CAR_TRAINING_GROUP, STATIC_GROUP))
            .insert(Friction {
                combine_rule: CoefficientCombineRule::Average,
                coefficient: 5.0,
                ..default()
            })
            .insert(Restitution::coefficient(0.))
            .insert(Damping {
                linear_damping: 0.05,
                angular_damping: 0.05,
            })
            .insert(ColliderMassProperties::MassProperties(MassProperties {
                local_center_of_mass: Vec3::ZERO,
                mass: 15.,
                principal_inertia: Vec3::ONE * 0.3,
                ..default()
            }))
            .insert(Wheel {
                radius: wheel_r,
                width: wheel_hw * 2.,
            })
            .insert(ExternalForce::default())
            .insert(ExternalImpulse::default())
            .id();
        wheels.push(wheel_id);

        if is_front {
            if is_left {
                commands
                    .entity(wheel_id)
                    .insert(WheelLeft)
                    .insert(WheelFront);
            } else {
                commands
                    .entity(wheel_id)
                    .insert(WheelRight)
                    .insert(WheelFront);
            }
        } else {
            if is_left {
                commands
                    .entity(wheel_id)
                    .insert(WheelLeft)
                    .insert(WheelBack);
            } else {
                commands
                    .entity(wheel_id)
                    .insert(WheelRight)
                    .insert(WheelBack);
            }
        };
    }
    let carrr = Car {
        size: size.clone(),
        wheels: wheels.clone(),
        wheel_max_torque: max_torque,
        init_transform: transform,
        init_meters,
        index,
        ..default()
    };

    let car_id = commands
        .spawn((
            Name::new("car"),
            Sleeping::disabled(),
            carrr,
            RigidBody::Dynamic,
        ))
        .insert(Ccd::enabled())
        .insert(Damping {
            linear_damping: 0.05,
            angular_damping: 0.1,
        })
        .insert(Velocity::zero())
        .insert(ExternalForce::default())
        .insert(TransformBundle::from(transform))
        .insert(ReadMassProperties::default())
        .insert(SceneBundle {
            scene: car_gl.clone(),
            transform,
            // .with_translation(Vec3::new(0., -0.75, 0.2))
            // .with_rotation(Quat::from_rotation_y(PI))
            // .with_scale(Vec3::ONE * 1.7),
            ..default()
        })
        .with_children(|children| {
            let collider_mass = ColliderMassProperties::MassProperties(MassProperties {
                local_center_of_mass: Vec3::new(0., -size.hh, 0.),
                mass: 1000.0,
                // https://www.nhtsa.gov/DOT/NHTSA/NRD/Multimedia/PDFs/VRTC/ca/capubs/sae1999-01-1336.pdf
                principal_inertia: Vec3::new(5000., 5000., 2000.),
                ..default()
            });
            let car_bradius = 0.1;
            children
                .spawn_empty()
                .insert(Name::new("car_collider"))
                .insert(Collider::round_cuboid(
                    size.hw - car_bradius,
                    size.hh - car_bradius,
                    size.hl - car_bradius,
                    car_bradius,
                ))
                .insert(ColliderScale::Absolute(Vec3::ONE))
                .insert(Friction::coefficient(0.5))
                .insert(Restitution::coefficient(0.))
                .insert(CollisionGroups::new(CAR_TRAINING_GROUP, STATIC_GROUP))
                .insert(CollidingEntities::default())
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(ContactForceEventThreshold(0.1))
                .insert(collider_mass);
        })
        .id();
    #[cfg(feature = "brain")]
    {
        commands
            .entity(car_id)
            .insert(crate::nn::dqn_bevy::CarDqnPrev::new());
    }

    if is_hid {
        commands.entity(car_id).insert(HID);
    }
    for (i, wheel_id) in wheels.iter().enumerate() {
        commands
            .entity(*wheel_id)
            .insert(JointType::new(car_id, joints[i]));
    }
    println!("car log: {car_id:?} {:?}", wheels);
    return car_id;
}

#[cfg(feature = "brain")]
pub fn car_sensor_system(
    rapier_context: Res<RapierContext>,
    config: Res<Config>,
    mut q_car: Query<(&mut Car, &GlobalTransform, &Transform), With<Car>>,
    #[cfg(feature = "debug_lines")] mut lines: ResMut<bevy_prototype_debug_lines::DebugLines>,
) {
    let sensor_filter = QueryFilter::<'_>::exclude_dynamic().exclude_sensors();
    let dir = Vec3::Z * config.max_toi;
    for (mut car, gt, t) in q_car.iter_mut() {
        let mut origins: Vec<Vec3> = Vec::new();
        let mut dirs: Vec<Vec3> = Vec::new();
        let g_translation = gt.translation();
        #[cfg(feature = "debug_lines")]
        {
            if config.show_rays {
                let h = Vec3::Y * 0.6;
                lines.line_colored(
                    h + g_translation,
                    h + car.line_pos + Vec3::Y * g_translation.y,
                    0.0,
                    Color::rgba(0.5, 0.5, 0.5, 0.5),
                );
            }
        }
        for a in 0..SENSOR_COUNT {
            let (pos, far_quat) = car.sensor_config[a];
            let origin = g_translation + t.rotation.mul_vec3(pos);
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
