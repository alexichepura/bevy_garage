mod camera;
mod car;
mod config;
mod dash;
mod db;
mod db_client;
mod esp;
mod gamepad;
mod input;
mod light;
mod mesh;
mod nn;
mod progress;
mod track;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_atmosphere::prelude::*;
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier3d::prelude::*;

use camera::*;
use car::*;
use config::*;
use dash::*;
use db_client::DbClientResource;
use esp::*;
use gamepad::*;
use input::*;
use light::*;
use nn::{dqn::dqn_system, dqn_bevy::*};

use progress::*;
use track::*;

fn main() {
    App::new()
        .insert_resource(DbClientResource::default())
        .insert_resource(DqnResource::default())
        .insert_resource(WindowDescriptor {
            title: "car sim deep learning".to_string(),
            width: 1024.,
            height: 768.,
            // present_mode: PresentMode::AutoVsync,
            ..default()
        })
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(Config::default())
        .insert_resource(CameraConfig::default())
        .insert_resource(AtmosphereSettings { resolution: 1024 })
        .add_plugins(DefaultPlugins)
        .add_plugin(AtmospherePlugin)
        .add_startup_system(camera_start_system)
        .add_system(camera_controller_system)
        .add_system(camera_switch_system)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin {
            style: DebugRenderStyle {
                rigid_body_axes_length: 0.5,
                subdivisions: 50,
                ..default()
            },
            // | DebugRenderMode::COLLIDER_AABBS
            mode: DebugRenderMode::COLLIDER_SHAPES
                | DebugRenderMode::RIGID_BODY_AXES
                | DebugRenderMode::JOINTS
                | DebugRenderMode::CONTACTS
                | DebugRenderMode::SOLVER_CONTACTS,
            ..default()
        })
        .add_plugin(DebugLinesPlugin::with_depth_test(true))
        .init_resource::<GamepadLobby>()
        .add_startup_system(dqn_exclusive_start_system.exclusive_system())
        .add_startup_system(track_start_system)
        .add_startup_system(track_decorations_start_system)
        .add_startup_system(track_polyline_start_system)
        .add_startup_system(car_start_system.after(track_polyline_start_system))
        .add_startup_system(light_start_system)
        .add_startup_system(dash_speed_start_system)
        .add_startup_system(dash_fps_start_system)
        .add_startup_system(rapier_config_start_system)
        .add_system(esp_system)
        .add_system(car_sensor_system)
        .add_system(dqn_system)
        .add_system(dqn_dash_update_system)
        .add_system(dash_fps_system)
        .add_system(dash_leaderboard_system)
        .add_system(dash_speed_update_system)
        // .add_system(gamepad_input_system)
        .add_system(keyboard_input_system)
        .add_system(progress_system)
        .add_system_to_stage(CoreStage::PreUpdate, gamepad_stage_preupdate_system)
        .run();
}

fn rapier_config_start_system(mut c: ResMut<RapierContext>) {
    c.integration_parameters.max_velocity_iterations = 4 * 128;
    c.integration_parameters.max_velocity_friction_iterations = 8 * 64 * 2;
    c.integration_parameters.max_stabilization_iterations = 1 * 1024 * 2;
    dbg!(c.integration_parameters);
}

// fn display_events_system(
//     mut e_collision: EventReader<CollisionEvent>,
//     mut e_force: EventReader<ContactForceEvent>,
// ) {
//     for collision_e in e_collision.iter() {
//         println!("main collision event: {:?}", collision_e);
//     }

//     for force_e in e_force.iter() {
//         println!(
//             "glomainbal force event: {:?} {:?}",
//             force_e.total_force, force_e.total_force_magnitude
//         );
//     }
// }
