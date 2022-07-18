mod brain;
mod camera;
mod car;
mod dash;
mod gamepad;
mod input;
mod light;
mod mesh;
mod plain;
mod track;

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
use input::*;
use light::*;
use plain::*;
use track::*;

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
        .add_plugin(RapierDebugRenderPlugin {
            mode: DebugRenderMode::COLLIDER_SHAPES
                | DebugRenderMode::RIGID_BODY_AXES
                | DebugRenderMode::JOINTS
                | DebugRenderMode::CONTACTS
                | DebugRenderMode::SOLVER_CONTACTS,
            ..default()
        })
        .add_plugin(PolylinePlugin)
        .init_resource::<GamepadLobby>()
        // .add_startup_system(camera_start_system)
        .add_startup_system(unreal_camera_start_system)
        .add_startup_system(plain_start_system)
        .add_startup_system(track_start_system)
        .add_startup_system(light_start_system)
        .add_startup_system(car_start_system)
        .add_startup_system(car_brain_start_system)
        .add_startup_system(dash_speed_start_system)
        .add_startup_system(dash_fps_start_system)
        .add_system(car_change_detection_system)
        .add_system(car_brain_system)
        .add_system(dash_fps_system)
        .add_system(dash_speed_update_system)
        // .add_system(gamepad_input_system)
        .add_system(arrow_input_system)
        .add_system_to_stage(CoreStage::PreUpdate, gamepad_stage_preupdate_system)
        // .add_system_to_stage(CoreStage::Update, camera_focus_update_system)
        .add_system_to_stage(CoreStage::PostUpdate, display_events_system)
        .run();
}

fn display_events_system(
    mut collision_events: EventReader<CollisionEvent>,
    mut contact_force_events: EventReader<ContactForceEvent>,
) {
    for collision_event in collision_events.iter() {
        println!("Received collision event: {:?}", collision_event);
    }

    for contact_force_event in contact_force_events.iter() {
        println!("Received contact force event: {:?}", contact_force_event);
    }
}
