mod brain;
mod camera;
mod car;
mod dash;
mod esp;
mod gamepad;
mod input;
mod light;
mod mesh;
mod plain;
mod track;

use std::f32::consts::PI;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
// use bevy_inspector_egui::widgets::{InspectorQuery, InspectorQuerySingle};
// use bevy_inspector_egui::{InspectorPlugin, WorldInspectorPlugin};
// use bevy_inspector_egui_rapier::InspectableRapierPlugin;
use bevy_mod_picking::*;
use bevy_obj::ObjPlugin;
use bevy_polyline::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{controllers::unreal::UnrealCameraPlugin, LookTransformPlugin};

use brain::*;
use camera::*;
use car::*;
use dash::*;
use esp::*;
use gamepad::*;
use input::*;
use light::*;
use plain::*;
use track::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins)
        // .add_plugin(WorldInspectorPlugin::new())
        // .add_plugin(InspectorPlugin::<InspectorQuerySingle<Entity, With<Car>>>::new())
        // .add_plugin(InspectorPlugin::<InspectorQuery<Entity, With<Wheel>>>::new())
        // .add_plugin(InspectableRapierPlugin)
        // CAMERA
        // .add_plugin(LookTransformPlugin)
        // .add_plugin(UnrealCameraPlugin::default())
        // .add_startup_system(unreal_camera_start_system)
        .add_startup_system(camera_start_system)
        .add_system_to_stage(CoreStage::Update, camera_focus_update_system)
        // DEBUG
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(ObjPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin {
            mode: DebugRenderMode::COLLIDER_SHAPES
                | DebugRenderMode::RIGID_BODY_AXES
                | DebugRenderMode::JOINTS
                // | DebugRenderMode::COLLIDER_AABBS
                | DebugRenderMode::CONTACTS
                | DebugRenderMode::SOLVER_CONTACTS,
            ..default()
        })
        .add_plugin(PolylinePlugin)
        .add_plugin(DebugLinesPlugin::with_depth_test(true))
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(DebugCursorPickingPlugin)
        .add_system_to_stage(CoreStage::PostUpdate, cars_pick_brain_mutate_restart)
        // APP
        .init_resource::<GamepadLobby>()
        .insert_resource(CarInit {
            translation: Vec3::new(0., 0.8, 0.),
            quat: Quat::from_rotation_y(-PI / 4.),
            hid_car: None,
        })
        .add_startup_system(plain_start_system)
        .add_startup_system(track_start_system)
        .add_startup_system(light_start_system)
        .add_startup_system(car_start_system)
        .add_startup_system(dash_speed_start_system)
        .add_startup_system(dash_fps_start_system)
        .add_system(car_change_detection_system)
        .add_system(car_brain_system)
        .add_system(dash_fps_system)
        .add_system(dash_speed_update_system)
        // .add_system(gamepad_input_system)
        .add_system(arrow_input_system)
        .add_system(reset_pos_system)
        .add_system(reset_spawn_key_system)
        .add_system_to_stage(CoreStage::PreUpdate, gamepad_stage_preupdate_system)
        .add_system_to_stage(CoreStage::PostUpdate, display_events_system)
        .run();
}

fn display_events_system(
    // mut e_collision: EventReader<CollisionEvent>,
    mut e_force: EventReader<ContactForceEvent>,
) {
    // for collision_e in e_collision.iter() {
    //     println!("collision: {:?}", collision_e);
    // }

    for force_e in e_force.iter() {
        // ContactForceEvent {
        //     collider1: todo!(),
        //     collider2: todo!(),
        //     total_force: todo!(),
        //     total_force_magnitude: todo!(),
        //     max_force_direction: todo!(),
        //     max_force_magnitude: todo!(),
        // };
        println!(
            "force: {:?} {:?}",
            force_e.total_force, force_e.total_force_magnitude
        );
    }
}
