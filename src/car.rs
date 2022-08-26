use crate::{config::*, mesh::*, nn::dqn_bevy::*, track::*};
use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;
use bevy_rapier3d::{
    parry::shape::Cylinder,
    prelude::*,
    rapier::prelude::{JointAxesMask, JointAxis},
};
use std::f32::consts::PI;

pub const SENSOR_COUNT: usize = 12;

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
pub struct WheelFrontLeft;
#[derive(Component)]
pub struct WheelFrontRight;
#[derive(Component)]
pub struct SensorFar;
#[derive(Component)]
pub struct SensorNear;
#[derive(Component)]
pub struct RayHit;
#[derive(Component)]
pub struct RayLine;
#[derive(Component)]
pub struct HID;

#[derive(Component, Debug)]
pub struct Car {
    pub sensor_inputs: Vec<f32>,
    pub gas: f32,
    pub brake: f32,
    pub steering: f32,
    pub prev_steering: f32,
    pub prev_torque: f32,
    pub prev_dir: f32,
    pub wheels: Vec<Entity>,
    pub wheel_max_torque: f32,
    pub init_transform: Transform,
    pub reset_at: Option<f64>,

    pub init_meters: f32,
    pub meters: f32,
    pub lap: usize,
    pub line_dir: Vec3,
    pub place: usize,
}

impl Car {
    pub fn new(
        wheels: &Vec<Entity>,
        wheel_max_torque: f32,
        init_transform: Transform,
        init_meters: f32,
    ) -> Self {
        Self {
            sensor_inputs: vec![0.; SENSOR_COUNT],
            gas: 0.,
            brake: 0.,
            steering: 0.,
            prev_steering: 0.,
            prev_torque: 0.,
            prev_dir: 0.,
            wheels: wheels.clone(),
            wheel_max_torque,
            init_transform,
            reset_at: None,

            init_meters,
            meters: 0.,
            place: 0,
            lap: 0,
            line_dir: Vec3::ZERO,
        }
    }
}

pub const CAR_TRAINING_GROUP: u32 = 0b001;
pub fn car_start_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut config: ResMut<Config>,
    asset_server: Res<AssetServer>,
    mut cars_dqn: NonSendMut<CarDqnResources>,
) {
    let car_gl = asset_server.load("car-race.glb#Scene0");

    let ray_point_half = 0.05;
    let ray_point_size = ray_point_half * 2.;
    let ray_point_mesh = Mesh::from(shape::Cube {
        size: ray_point_size,
    });
    for _i in 0..SENSOR_COUNT {
        commands.spawn().insert(RayHit).insert_bundle(PbrBundle {
            mesh: meshes.add(ray_point_mesh.clone()),
            material: materials.add(Color::rgba(0.95, 0.5, 0.5, 0.9).into()),
            ..default()
        });
    }

    let wheel_r: f32 = 0.4;
    let wheel_hw: f32 = 0.2;
    let car_hw: f32 = 1.;
    let car_hh: f32 = 0.35;
    let car_hl: f32 = 2.2;
    let ride_height = 0.08;

    let shift = Vec3::new(
        car_hw - wheel_hw - 0.1,
        -car_hh + wheel_r - ride_height,
        car_hl - wheel_r - 0.5,
    );
    let car_anchors: [Vec3; 4] = [
        Vec3::new(shift.x, shift.y, shift.z),
        Vec3::new(-shift.x, shift.y, shift.z),
        Vec3::new(shift.x, shift.y, -shift.z),
        Vec3::new(-shift.x, shift.y, -shift.z),
    ];

    for i in 0..config.cars_count {
        let is_hid = i == 0;
        let (car_translation, car_quat, car_init_meters) = config.get_transform_by_index(i);
        let car_transform = Transform::from_translation(car_translation).with_rotation(car_quat);
        let mut wheels: Vec<Entity> = vec![];
        let mut joints: Vec<GenericJoint> = vec![];
        for i in 0..4 {
            let joint_mask = JointAxesMask::X
                // | JointAxesMask::Y 
                // | JointAxesMask::Z
                // | JointAxesMask::ANG_X
                | JointAxesMask::ANG_Y
                | JointAxesMask::ANG_Z;

            let joint = GenericJointBuilder::new(joint_mask)
                .local_axis1(Vec3::X)
                .local_axis2(Vec3::Y)
                .local_anchor1(car_anchors[i])
                .local_anchor2(Vec3::ZERO)
                .set_motor(JointAxis::Y, 0., 0., 100000., 1.)
                .set_motor(JointAxis::Z, 0., 0., 20000., 1.)
                // .motor_velocity(JointAxis::AngX, 100., 0.)
                // .motor_velocity(JointAxis::AngY, 1., 1. / 100.)
                // .motor_velocity(JointAxis::AngZ, 1., 1. / 100.)
                .build();
            joints.push(joint);

            let wheel_border_radius = 0.05;
            let wheel_id = commands
                .spawn()
                .insert(Name::new("wheel"))
                .insert(Sleeping::disabled())
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(bevy_mesh(Cylinder::new(wheel_hw, wheel_r).to_trimesh(50))),
                    material: materials.add(Color::rgba(0.1, 0.1, 0.1, 0.7).into()),
                    ..default()
                })
                .insert_bundle(TransformBundle::from(
                    Transform::from_translation(
                        car_transform.translation + car_transform.rotation.mul_vec3(car_anchors[i]),
                    )
                    .with_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
                ))
                .insert(RigidBody::Dynamic)
                .insert(Ccd::enabled())
                .insert(Velocity::zero())
                // .insert(Collider::cylinder(wheel_hw, wheel_r - wheel_border_radius))
                .insert(Collider::round_cylinder(
                    wheel_hw - wheel_border_radius,
                    wheel_r - wheel_border_radius,
                    wheel_border_radius,
                ))
                .insert(ColliderScale::Absolute(Vec3::ONE))
                .insert(CollisionGroups::new(CAR_TRAINING_GROUP, STATIC_GROUP))
                .insert(Friction {
                    combine_rule: CoefficientCombineRule::Max,
                    coefficient: 10.0,
                    ..default()
                })
                .insert(Restitution::coefficient(0.))
                .insert(ColliderMassProperties::MassProperties(MassProperties {
                    local_center_of_mass: Vec3::ZERO,
                    mass: 15.,                          // 15.
                    principal_inertia: Vec3::ONE * 0.3, // 0.3
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

            match i {
                0 => {
                    commands
                        .entity(wheel_id)
                        .insert(WheelFrontRight)
                        .insert(WheelFront);
                }
                1 => {
                    commands
                        .entity(wheel_id)
                        .insert(WheelFrontLeft)
                        .insert(WheelFront);
                }
                _ => {
                    commands.entity(wheel_id).insert(WheelBack);
                }
            }
        }

        let car_id = commands
            .spawn()
            .insert(Name::new("car"))
            .insert(Sleeping::disabled())
            .insert(Car::new(
                &wheels,
                config.max_torque,
                car_transform,
                car_init_meters,
            ))
            .insert(RigidBody::Dynamic)
            .insert(Velocity::zero())
            .insert(ExternalForce::default())
            .insert_bundle(TransformBundle::from(car_transform))
            .insert(ReadMassProperties::default())
            .insert_bundle(SceneBundle {
                scene: car_gl.clone(),
                transform: car_transform,
                // .with_translation(Vec3::new(0., -0.75, 0.2))
                // .with_rotation(Quat::from_rotation_y(PI))
                // .with_scale(Vec3::ONE * 1.7),
                ..default()
            })
            .with_children(|children| {
                let collider_mass = ColliderMassProperties::MassProperties(MassProperties {
                    local_center_of_mass: Vec3::new(0., -car_hh, 0.),
                    mass: 1500.0,
                    // https://www.nhtsa.gov/DOT/NHTSA/NRD/Multimedia/PDFs/VRTC/ca/capubs/sae1999-01-1336.pdf
                    principal_inertia: Vec3::new(5000., 5000., 2000.),
                    ..default()
                });
                let car_bradius = 0.05;
                children
                    .spawn()
                    .insert(Name::new("car_collider"))
                    .insert(Ccd::enabled())
                    .insert(Collider::round_cuboid(
                        car_hw - car_bradius,
                        car_hh - car_bradius,
                        car_hl - car_bradius,
                        car_bradius,
                    ))
                    .insert(ColliderScale::Absolute(Vec3::ONE))
                    .insert(Friction::coefficient(1.))
                    .insert(Restitution::coefficient(0.))
                    .insert(CollisionGroups::new(CAR_TRAINING_GROUP, STATIC_GROUP))
                    .insert(CollidingEntities::default())
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(ContactForceEventThreshold(0.1))
                    .insert(collider_mass);

                let sensor_angle = 2. * PI / SENSOR_COUNT as f32;
                for a in 0..SENSOR_COUNT {
                    let far_quat = Quat::from_rotation_y(-(a as f32) * sensor_angle);
                    let dir = Vec3::Z * config.max_toi;
                    let sensor_pos_on_car = Vec3::new(0., 0.1, 0.);
                    children
                        .spawn()
                        .insert(SensorNear)
                        .insert_bundle(TransformBundle::from(Transform::from_translation(
                            sensor_pos_on_car,
                        )));
                    children
                        .spawn()
                        .insert(SensorFar)
                        .insert_bundle(TransformBundle::from(Transform::from_translation(
                            sensor_pos_on_car + far_quat.mul_vec3(dir),
                        )));
                }
            })
            .id();

        if is_hid {
            config.hid_car = Some(car_id);
            commands.entity(car_id).insert(HID);
        }
        for (i, wheel_id) in wheels.iter().enumerate() {
            commands
                .entity(*wheel_id)
                .insert(ImpulseJoint::new(car_id, joints[i]));
        }

        cars_dqn.add_car(car_id);
        println!("car log: {car_id:?} {:?}", wheels);
    }
}

pub fn car_sensor_system(
    rapier_context: Res<RapierContext>,
    config: Res<Config>,
    mut q_car: Query<(Entity, &mut Car, &Children, &Velocity), With<Car>>,
    q_near: Query<(&GlobalTransform, With<SensorNear>)>,
    q_far: Query<(&GlobalTransform, With<SensorFar>)>,
    mut ray_set: ParamSet<(Query<(&mut Transform, With<RayHit>)>,)>,
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
                                Color::rgba(0.5, 0.3, 0.3, 0.5),
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
                trf.translation = hit_points[i];
            }
        }
        inputs.push(v.linvel.length());
        car.sensor_inputs = inputs.clone();
    }
}
