mod brain;
mod camera;
mod car;
mod dash;
mod gamepad;
mod graphics;
mod input;
mod mesh;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
// use bevy_inspector_egui::widgets::{InspectorQuery, InspectorQuerySingle};
// use bevy_inspector_egui::InspectorPlugin;
// use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_obj::ObjPlugin;
use bevy_polyline::prelude::*;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{controllers::unreal::UnrealCameraPlugin, LookTransformPlugin};

use brain::*;
use camera::*;
use car::*;
use dash::*;
use gamepad::*;
use graphics::*;
use input::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        // .add_plugin(InspectorPlugin::<InspectorQuerySingle<Entity, With<Car>>>::new())
        // .add_plugin(InspectorPlugin::<InspectorQuery<Entity, With<Wheel>>>::new())
        // .add_plugin(InspectableRapierPlugin)
        .add_plugin(LookTransformPlugin)
        .add_plugin(UnrealCameraPlugin::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(ObjPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_plugin(PolylinePlugin)
        // .add_system(gamepad_input_system)
        .add_system(arrow_input_system)
        .init_resource::<GamepadLobby>()
        .add_system_to_stage(CoreStage::PreUpdate, gamepad_lobby_system)
        // .add_system_to_stage(CoreStage::Update, camera_focus_system)
        .add_startup_system(camera_system)
        .add_startup_system(graphics_system)
        .add_startup_system(car_system)
        .add_startup_system(dash_fps_system)
        .add_startup_system(dash_speed_system)
        .add_startup_system(car_brain_start_system)
        .add_system(dash_fps_update_system)
        .add_system(car_brain_system)
        .add_system(dash_speed_update_system)
        .run();
}
