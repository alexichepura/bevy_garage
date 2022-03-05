use crate::car::{BackJoint, Car, FrontJoint, FrontLeftJoint, FrontRightJoint, Wheel};
use crate::gamepad::{gamepad_lobby_system, GamepadLobby};
use crate::graphics::setup_graphics;
use crate::input::{arrow_input_system, gamepad_input_system};
use bevy::{
    app::App,
    app::CoreStage,
    asset::Assets,
    diagnostic::FrameTimeDiagnosticsPlugin,
    ecs::system::{Commands, Res, ResMut},
    math::{Quat, Vec3},
    pbr::{prelude::StandardMaterial, PbrBundle},
    prelude::{AssetServer, BuildChildren, Msaa},
    render::{color::Color, mesh::shape, mesh::Mesh},
    transform::components::Transform,
    DefaultPlugins,
};
use bevy_obj::ObjPlugin;
use bevy_rapier3d::prelude::{JointAxesMask, JointData};
use bevy_rapier3d::{
    physics::JointBuilderComponent,
    prelude::{
        ColliderBundle, ColliderMaterial, ColliderPositionSync, ColliderShape, Isometry,
        MassProperties, NoUserData, Point, RapierPhysicsPlugin, Real, RigidBodyBundle,
        RigidBodyPosition, Vector,
    },
    render::RapierRenderPlugin,
};
use dash::{dash_fps_update, dash_speed_update, setup_dash_fps, setup_dash_speed};
use graphics::focus_camera;

mod car;
mod dash;
mod gamepad;
mod graphics;
mod input;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(ObjPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierRenderPlugin)
        .add_system(gamepad_input_system)
        .add_system(arrow_input_system)
        .init_resource::<GamepadLobby>()
        .add_system_to_stage(CoreStage::PreUpdate, gamepad_lobby_system)
        .add_system_to_stage(CoreStage::Update, focus_camera)
        .add_startup_system(setup_graphics)
        .add_startup_system(setup)
        .add_startup_system(setup_dash_fps)
        .add_startup_system(setup_dash_speed)
        .add_system(dash_fps_update)
        .add_system(dash_speed_update)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let car_graphics = "hatchbackSports.obj";
    let car_hw: f32 = 0.45;
    let car_hh: f32 = 0.5;
    let car_hl: f32 = 0.8;
    let wheel_r: f32 = 0.25;
    let wheel_hw: f32 = 0.25;
    let car_pos_transform = Vec3::new(0., 2.5, 0.);
    let qvec = Vec3::new(0., 0., 0.);
    let car_quat = Quat::from_axis_angle(qvec, 0.);
    let car_isometry: Isometry<Real> = Isometry::new(car_pos_transform.into(), qvec.into());

    let car_entity = commands
        .spawn_bundle(RigidBodyBundle {
            position: RigidBodyPosition {
                position: car_isometry,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert_bundle(ColliderBundle {
            shape: ColliderShape::cuboid(car_hl, car_hh, car_hw).into(),
            mass_properties: MassProperties::new(
                Vec3::new(0.0, -0.4, 0.0).into(),
                1500.0,
                Vec3::new(100.0, 100.0, 100.0).into(),
            )
            .into(),
            material: ColliderMaterial {
                friction: 0.001,
                restitution: 0.1,
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .with_children(|parent| {
            // let mut tr: Transform = Transform::from_rotation(Quat::from_rotation_y(PI / 2.0));
            let mut tr: Transform = Transform {
                ..Default::default()
            };
            tr.translation = Vec3::new(0.0, -car_hh, 0.0);
            parent.spawn_bundle(PbrBundle {
                mesh: asset_server.load(car_graphics),
                material: materials.add(Color::rgb(0.3, 0.3, 0.8).into()),
                transform: tr,
                ..Default::default()
            });
        })
        .insert(Car)
        .insert(Transform::default())
        .insert(ColliderPositionSync::Discrete)
        .id();

    let shift = Vec3::new(car_hw + 0.1 + wheel_hw, -car_hh, car_hl);
    let car_anchors: [Vec3; 4] = [
        Vec3::new(shift.x, shift.y, shift.z),
        Vec3::new(-shift.x, shift.y, shift.z),
        Vec3::new(shift.x, shift.y, -shift.z),
        Vec3::new(-shift.x, shift.y, -shift.z),
    ];

    for i in 0..4 {
        let mask = JointAxesMask::X
            | JointAxesMask::Y
            | JointAxesMask::Z
            | JointAxesMask::ANG_Y
            | JointAxesMask::ANG_Z;

        let data = JointData::default()
            .lock_axes(mask)
            .local_axis1(Vector::x_axis())
            .local_axis2(Vector::x_axis())
            .local_anchor1(Point::from(car_anchors[i]))
            .local_anchor2(Vec3::new(0., 0., 0.).into());

        // joint.configure_motor_model(SpringModel::ForceBased);

        let wheel_transform = car_pos_transform + car_quat.mul_vec3(car_anchors[i]);
        let wheel_qvec = Vec3::new(0., 0., 0.);
        let wheel_isometry: Isometry<Real> =
            Isometry::new(wheel_transform.into(), wheel_qvec.into());

        let wheel = commands
            .spawn_bundle(PbrBundle {
                mesh: meshes.add(Mesh::from(shape::Cube {
                    size: wheel_r * 2.,
                    ..Default::default()
                })),
                // mesh: meshes.add(Mesh::from(shape::Torus {
                //     radius: wheel_r,
                //     ring_radius: wheel_r / 2.,
                //     ..Default::default()
                // })),
                material: materials.add(Color::rgb(0.1, 0.1, 0.3).into()),
                ..Default::default()
            })
            .insert_bundle(RigidBodyBundle {
                position: wheel_isometry.into(),
                ..Default::default()
            })
            .insert_bundle(ColliderBundle {
                // shape: ColliderShape::round_cylinder(wheel_r, wheel_r, 0.05).into(),
                shape: ColliderShape::ball(wheel_r).into(),
                mass_properties: MassProperties::new(
                    Vec3::new(0.0, 0.0, 0.0).into(),
                    15.0,
                    Vec3::new(1.0, 1.0, 1.0).into(),
                )
                .into(),
                material: ColliderMaterial {
                    friction: 1.0,
                    restitution: 0.1,
                    ..Default::default()
                }
                .into(),
                ..Default::default()
            })
            .insert(Wheel)
            .insert(Transform::default())
            .insert(ColliderPositionSync::Discrete)
            .id();
        if i == 0 {
            commands
                .spawn_bundle((JointBuilderComponent::new(data, car_entity, wheel),))
                .insert(FrontRightJoint)
                .insert(FrontJoint);
        } else if i == 1 {
            commands
                .spawn_bundle((JointBuilderComponent::new(data, car_entity, wheel),))
                .insert(FrontLeftJoint)
                .insert(FrontJoint);
        } else if i == 2 {
            commands
                .spawn_bundle((JointBuilderComponent::new(data, car_entity, wheel),))
                .insert(BackJoint);
        } else if i == 3 {
            commands
                .spawn_bundle((JointBuilderComponent::new(data, car_entity, wheel),))
                .insert(BackJoint);
        }
    }
}
