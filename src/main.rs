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
use car::car_system;
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
        .add_startup_system(car_system)
        .add_startup_system(setup_dash_fps)
        .add_startup_system(setup_dash_speed)
        .add_system(dash_fps_update)
        .add_system(dash_speed_update)
        .run();
}
