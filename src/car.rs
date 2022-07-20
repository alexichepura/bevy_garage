use crate::{brain::*, mesh::*, track::*};
use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;
use bevy_polyline::prelude::{Polyline, PolylineBundle, PolylineMaterial};
use bevy_rapier3d::{parry::shape::Cylinder, prelude::*};
use rapier3d::prelude::{JointAxesMask, SharedShape};
use std::{f32::consts::PI, fs::File, path::Path, sync::Arc};

pub struct CarInit {
    pub translation: Vec3,
    pub quat: Quat,
    pub hid_car: Option<Entity>,
}

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
    pub gas: f32,
    pub brake: f32,
    pub steering: f32,
    pub use_brain: bool,
    pub wheels: Vec<Entity>,
    pub wheel_max_torque: f32,
}
#[derive(Component)]
pub struct HID;

impl Car {
    pub fn new(wheels: &Vec<Entity>) -> Self {
        Self {
            gas: 0.,
            brake: 0.,
            steering: 0.,
            use_brain: true,
            wheels: wheels.clone(),
            wheel_max_torque: 300.,
        }
    }
}

pub const CAR_TRAINING_GROUP: u32 = 0b001;
// const CAR_OBJ: &str = "hatchbackSports.obj";
pub fn car_start_system(
    // asset_server: Res<AssetServer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    mut car_init: ResMut<CarInit>,
) {
    let ray_point_half = 0.05;
    let ray_point_size = ray_point_half * 2.;
    let ray_point_mesh = Mesh::from(shape::Cube {
        size: ray_point_size,
    });
    for _i in 0..5 {
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

    let saved_brain: Option<CarBrain>;
    let json_file = File::open(Path::new("brain.json"));
    if json_file.is_ok() {
        println!("brain.json found");
        saved_brain =
            CarBrain::clone_randomised(serde_json::from_reader(json_file.unwrap()).unwrap());
    } else {
        saved_brain = None;
    }

    let wheel_r: f32 = 0.5;
    let wheel_hw: f32 = 0.125;
    let car_hw: f32 = 1.;
    let car_hh: f32 = wheel_r / 2.;
    let car_hl: f32 = 2.3;

    let shift = Vec3::new(car_hw - wheel_hw - 0.05, -car_hh, car_hl - wheel_r - 0.2);
    let car_anchors: [Vec3; 4] = [
        Vec3::new(shift.x, shift.y, shift.z),
        Vec3::new(-shift.x, shift.y, shift.z),
        Vec3::new(shift.x, shift.y, -shift.z),
        Vec3::new(-shift.x, shift.y, -shift.z),
    ];

    for i in 0..100 {
        let is_hid = i == 0;
        let car_transform = Transform::from_translation(
            car_init.translation
                + Vec3::new(
                    -10. + 0.18 * i as f32,
                    0.01 * i as f32,
                    10. - 0.18 * i as f32,
                ),
        )
        .with_rotation(car_init.quat);

        let mut wheels: Vec<Entity> = vec![];
        let mut joints: Vec<GenericJoint> = vec![];
        for i in 0..4 {
            let joint_mask = JointAxesMask::X
                | JointAxesMask::Y
                | JointAxesMask::Z
                | JointAxesMask::ANG_Y
                | JointAxesMask::ANG_Z;

            let joint = GenericJointBuilder::new(joint_mask)
                .local_axis1(Vec3::X)
                .local_axis2(Vec3::Y)
                .local_anchor1(car_anchors[i])
                .local_anchor2(Vec3::ZERO)
                .build();
            joints.push(joint);

            let wheel_transform = car_init.translation + car_init.quat.mul_vec3(car_anchors[i]);
            let wheel_cylinder = Cylinder::new(wheel_hw, wheel_r);
            let wheel_shape = SharedShape(Arc::new(wheel_cylinder));
            let wheel_id = commands
                .spawn()
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(ContactForceEventThreshold(0.01))
                .insert_bundle(PbrBundle {
                    mesh: meshes.add(bevy_mesh(wheel_cylinder.to_trimesh(50))),
                    material: materials.add(Color::rgba(0.05, 0.05, 0.05, 0.2).into()),
                    ..default()
                })
                .insert(RigidBody::Dynamic)
                .insert(Ccd::enabled())
                .insert_bundle(TransformBundle::from(
                    Transform::from_translation(wheel_transform)
                        .with_rotation(Quat::from_axis_angle(Vec3::Y, PI)),
                ))
                .insert(Velocity::zero())
                .insert(Collider::from(wheel_shape))
                .insert(CollisionGroups::new(CAR_TRAINING_GROUP, STATIC_GROUP))
                .insert(Friction::coefficient(1000.))
                .insert(Restitution::coefficient(0.00001))
                .insert(ColliderMassProperties::MassProperties(MassProperties {
                    local_center_of_mass: Vec3::ZERO,
                    mass: 15.,
                    principal_inertia: Vec3::new(0.3, 0.3, 0.3),
                    ..default()
                }))
                .insert(Wheel {
                    radius: wheel_r,
                    width: wheel_hw * 2.,
                })
                .insert(ExternalForce::default())
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
            .insert(ActiveEvents::COLLISION_EVENTS)
            .insert(ContactForceEventThreshold(0.01))
            .insert(Name::new("Car"))
            .insert_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Box {
                    max_x: car_hw,
                    min_x: -car_hw,
                    max_y: car_hh,
                    min_y: -car_hh,
                    max_z: car_hl,
                    min_z: -car_hl,
                })),
                material: materials.add(Color::rgba(0.3, 0.3, 0.9, 0.2).into()),
                ..default()
            })
            .insert(Car::new(&wheels))
            .insert(RigidBody::Dynamic)
            .insert(Velocity::zero())
            .insert_bundle(TransformBundle::from(car_transform))
            .insert_bundle(PickableBundle::default())
            .insert(ReadMassProperties::default())
            .with_children(|children| {
                // children.spawn().insert_bundle(PbrBundle {
                //     mesh: asset_server.load(CAR_OBJ),
                //     material: materials.add(Color::rgb(0.3, 0.3, 0.8).into()),
                //     transform: Transform::from_translation(Vec3::new(0., -0.3, 0.)),
                //     ..default()
                // });
                children
                    .spawn()
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(ContactForceEventThreshold(0.01))
                    .insert(Ccd::enabled())
                    .insert(Collider::cuboid(car_hw, car_hh, car_hl))
                    .insert(Friction::coefficient(1.))
                    .insert(Restitution::coefficient(0.0001))
                    .insert(CollisionGroups::new(CAR_TRAINING_GROUP, STATIC_GROUP))
                    .insert(ColliderMassProperties::MassProperties(MassProperties {
                        local_center_of_mass: Vec3::new(0., -0.2, 0.),
                        mass: 1500.0,
                        principal_inertia: Vec3::new(10., 10., 10.),
                        ..default()
                    }));

                // children
                //     .spawn()
                //     .insert(Collider::cuboid(car_hw + 1., 0.5, 10.))
                //     .insert_bundle(TransformBundle::from(Transform::from_translation(
                //         Vec3::new(0., 0., car_hl + 10.),
                //     )))
                //     .insert(CollisionGroups::new(CAR_TRAINING_GROUP, STATIC_GROUP))
                //     // .insert(ActiveEvents::COLLISION_EVENTS)
                //     .insert(Sensor);

                for a in -2..3 {
                    let far_quat = Quat::from_rotation_y(-a as f32 * PI / 8.);
                    let dir = Vec3::Z * 50.;
                    children
                        .spawn_bundle(PolylineBundle {
                            polyline: polylines.add(Polyline {
                                vertices: vec![Vec3::ZERO, dir],
                                ..default()
                            }),
                            material: polyline_materials.add(PolylineMaterial {
                                width: 2.0,
                                // color: Color::RED,
                                // color: Color::rgba(0.0, 0.5, 0.5, 0.8),
                                color: Color::rgba(0.98, 0.5, 0.45, 0.8),
                                perspective: true,
                                ..default()
                            }),
                            ..default()
                        })
                        .insert_bundle(TransformBundle::from(
                            Transform::from_translation(Vec3::new(0., 0., car_hl))
                                .with_rotation(far_quat),
                        ))
                        .insert(CarSensor);

                    if is_hid {
                        let sensor_pos_on_car = Vec3::new(0., 0., car_hl);
                        children
                            .spawn()
                            .insert(SensorNear)
                            .insert_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                                material: materials.add(Color::rgba(0.9, 0.5, 0.5, 0.5).into()),
                                ..default()
                            })
                            .insert_bundle(TransformBundle::from(Transform::from_translation(
                                sensor_pos_on_car,
                            )));
                        children
                            .spawn()
                            .insert(SensorFar)
                            .insert_bundle(PbrBundle {
                                mesh: meshes.add(Mesh::from(shape::Cube { size: 0.1 })),
                                material: materials.add(Color::rgba(0.9, 0.5, 0.5, 0.5).into()),
                                ..default()
                            })
                            .insert_bundle(TransformBundle::from(Transform::from_translation(
                                sensor_pos_on_car + far_quat.mul_vec3(dir),
                            )));
                    }
                }
            })
            .id();

        if is_hid {
            // select first car for human interactions
            car_init.hid_car = Some(car);
            commands.entity(car).insert(HID);
        }

        for (i, wheel_id) in wheels.iter().enumerate() {
            commands
                .entity(*wheel_id)
                .insert(MultibodyJoint::new(car, joints[i]));
        }

        if let Some(saved_brain) = &saved_brain {
            commands.entity(car).insert(saved_brain.clone());
        } else {
            commands.entity(car).insert(CarBrain::new());
        }
    }
}
