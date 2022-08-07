use crate::{config::Config, mesh::*, progress::CarProgress, track::*, trainer::*};
use bevy::prelude::*;
use bevy_rapier3d::{
    parry::shape::Cylinder,
    prelude::*,
    rapier::prelude::{JointAxesMask, JointAxis},
};
use std::f32::consts::PI;

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
pub struct RayDir;
#[derive(Component)]
pub struct RayOrig;
#[derive(Component)]
pub struct RayHit;
#[derive(Component)]
pub struct RayLine;
#[derive(Component, Debug)]
pub struct Car {
    pub sensor_inputs: Vec<f32>,
    pub gas: f32,
    pub brake: f32,
    pub steering: f32,
    pub use_brain: bool,
    pub wheels: Vec<Entity>,
    pub wheel_max_torque: f32,
    pub init_transform: Transform,
    pub reset_at: Option<f64>,
}
#[derive(Component)]
pub struct HID;

impl Car {
    pub fn new(
        wheels: &Vec<Entity>,
        use_brain: bool,
        wheel_max_torque: f32,
        init_transform: Transform,
    ) -> Self {
        Self {
            sensor_inputs: vec![0.; 7],
            gas: 0.,
            brake: 0.,
            steering: 0.,
            use_brain,
            wheels: wheels.clone(),
            wheel_max_torque,
            init_transform,
            reset_at: None,
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
    trainer: Res<Trainer>,
) {
    let car_gl = asset_server.load("car-race.glb#Scene0");

    let ray_point_half = 0.05;
    let ray_point_size = ray_point_half * 2.;
    let ray_point_mesh = Mesh::from(shape::Cube {
        size: ray_point_size,
    });
    for _i in 0..config.sensor_count {
        commands.spawn().insert(RayDir).insert_bundle(PbrBundle {
            mesh: meshes.add(ray_point_mesh.clone()),
            material: materials.add(Color::rgba(0.3, 0.9, 0.9, 0.5).into()),
            ..default()
        });
        commands.spawn().insert(RayOrig).insert_bundle(PbrBundle {
            mesh: meshes.add(ray_point_mesh.clone()),
            material: materials.add(Color::rgba(0.3, 0.9, 0.9, 0.5).into()),
            ..default()
        });
        commands.spawn().insert(RayHit).insert_bundle(PbrBundle {
            mesh: meshes.add(ray_point_mesh.clone()),
            material: materials.add(Color::rgba(0.9, 0.9, 0.9, 0.9).into()),
            ..default()
        });
    }

    let wheel_r: f32 = 0.4;
    let wheel_hw: f32 = 0.15;
    let car_hw: f32 = 1.;
    let car_hh: f32 = 0.4;
    let car_hl: f32 = 2.2;
    let ride_height = 0.25;

    let shift = Vec3::new(
        car_hw - wheel_hw - 0.01,
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
        let car_transform = Transform::from_translation(
            // config.translation + config.quat.mul_vec3(-Vec3::Z * 5. * i as f32),
            config.translation,
        )
        .with_rotation(config.quat);

        let mut wheels: Vec<Entity> = vec![];
        let mut joints: Vec<GenericJoint> = vec![];
        for i in 0..4 {
            let joint_mask =
                JointAxesMask::X | JointAxesMask::Y | JointAxesMask::ANG_Y | JointAxesMask::ANG_Z;

            let joint = GenericJointBuilder::new(joint_mask)
                .local_axis1(Vec3::X)
                .local_axis2(Vec3::Y)
                .local_anchor1(car_anchors[i])
                .local_anchor2(Vec3::ZERO)
                .set_motor(JointAxis::Z, 0., 0., 10e35 * 300., 10e35 * 10.)
                .build();
            joints.push(joint);

            let wheel_transform = config.translation + config.quat.mul_vec3(car_anchors[i]);
            let wheel_cylinder = Cylinder::new(wheel_hw, wheel_r);
            let mesh = bevy_mesh(wheel_cylinder.to_trimesh(50));
            let wheel_border_radius = 0.05;
            let collider = Collider::round_cylinder(
                wheel_hw,
                wheel_r - wheel_border_radius,
                wheel_border_radius,
            );
            let wheel_pbr = PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::rgba(0.2, 0.2, 0.2, 0.5).into()),
                ..default()
            };
            let wheel_transform = TransformBundle::from(
                Transform::from_translation(wheel_transform)
                    .with_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
            );
            let wheel_collider_mass = ColliderMassProperties::MassProperties(MassProperties {
                local_center_of_mass: Vec3::ZERO,
                mass: 15.,
                principal_inertia: Vec3::ONE * 0.5,
                ..default()
            });
            let wheel = Wheel {
                radius: wheel_r,
                width: wheel_hw * 2.,
            };
            let wheel_id = commands
                .spawn()
                .insert(Name::new("wheel"))
                .insert(Sleeping::disabled())
                .insert_bundle(wheel_pbr)
                .insert_bundle(wheel_transform)
                .insert(RigidBody::Dynamic)
                .insert(Ccd::enabled())
                .insert(Velocity::zero())
                .insert(collider)
                .insert(ColliderScale::Absolute(Vec3::ONE))
                .insert(CollisionGroups::new(CAR_TRAINING_GROUP, STATIC_GROUP))
                .insert(Friction::coefficient(10.))
                .insert(Restitution::coefficient(0.))
                .insert(wheel_collider_mass)
                .insert(wheel)
                .insert(ExternalForce::default())
                .insert(ExternalImpulse::default())
                .id();
            wheels.push(wheel_id);

            if i == 0 {
                commands
                    .entity(wheel_id)
                    .insert(WheelFrontRight)
                    .insert(WheelFront);
            } else if i == 1 {
                commands
                    .entity(wheel_id)
                    .insert(WheelFrontLeft)
                    .insert(WheelFront);
            } else if i == 2 {
                commands.entity(wheel_id).insert(WheelBack);
            } else if i == 3 {
                commands.entity(wheel_id).insert(WheelBack);
            }
        }

        let car = commands
            .spawn()
            .insert(Name::new("car"))
            .insert(Sleeping::disabled())
            .insert(Name::new("Car"))
            .insert(Car::new(
                &wheels,
                config.use_brain,
                config.max_torque,
                car_transform,
            ))
            .insert(CarProgress {
                meters: 0.,
                place: 0,
            })
            .insert(RigidBody::Dynamic)
            .insert(Velocity::zero())
            // .insert(ExternalImpulse::default())
            .insert(ExternalForce::default())
            .insert_bundle(TransformBundle::from(car_transform))
            // .insert_bundle(PickableBundle::default())
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
                    principal_inertia: Vec3::new(2000., 2000., 200.),
                    ..default()
                });
                children
                    .spawn()
                    .insert(Name::new("car_collider"))
                    .insert(Ccd::enabled())
                    .insert(Collider::cuboid(car_hw, car_hh, car_hl))
                    .insert(ColliderScale::Absolute(Vec3::ONE))
                    .insert(Friction::coefficient(0.0001))
                    .insert(Restitution::coefficient(0.))
                    .insert(CollisionGroups::new(CAR_TRAINING_GROUP, STATIC_GROUP))
                    .insert(CollidingEntities::default())
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(collider_mass);

                let half = config.sensor_count as i8 / 2;
                for a in -half..(half + 1) {
                    let far_quat = Quat::from_rotation_y(-a as f32 * PI * 0.02);
                    let dir = Vec3::Z * config.max_toi;
                    let sensor_pos_on_car = Vec3::new(0., 0.3, car_hl);
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
            config.hid_car = Some(car);
            commands.entity(car).insert(HID);
        }
        for (i, wheel_id) in wheels.iter().enumerate() {
            commands
                .entity(*wheel_id)
                .insert(MultibodyJoint::new(car, joints[i]));
        }

        if config.use_brain {
            commands
                .entity(car)
                .insert(trainer.clone_best_brain_or_get_new());
        }
    }
}
