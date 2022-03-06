use crate::car::Car;
use crate::gamepad::{gamepad_lobby_system, GamepadLobby};
use crate::graphics::graphics_system;
use crate::input::{arrow_input_system, gamepad_input_system};
use bevy::{
    app::App, app::CoreStage, diagnostic::FrameTimeDiagnosticsPlugin, prelude::Msaa, DefaultPlugins,
};
use bevy_obj::ObjPlugin;

use bevy_rapier3d::{
    prelude::{NoUserData, RapierPhysicsPlugin},
    render::RapierRenderPlugin,
};
use car::car_system;
use dash::{dash_fps_system, dash_fps_update_system, dash_speed_system, dash_speed_update_system};
use graphics::camera_focus_system;

mod car;
mod dash;
mod gamepad;
mod graphics;
mod input;
mod mesh;

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
        .add_system_to_stage(CoreStage::Update, camera_focus_system)
        .add_startup_system(graphics_system)
        .add_startup_system(car_system)
        .add_startup_system(dash_fps_system)
        .add_startup_system(dash_speed_system)
        .add_system(dash_fps_update_system)
        .add_system(dash_speed_update_system)
        .run();
}
