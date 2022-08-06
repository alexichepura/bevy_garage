mod brain;
mod brain_dfdx;
mod camera;
mod car;
mod config;
mod dash;
mod esp;
mod gamepad;
mod input;
mod light;
mod mesh;
mod plain;
mod progress;
mod track;
mod trainer;
mod util;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier3d::prelude::*;

use brain::*;
use brain_dfdx::*;
use camera::*;
use car::*;
use config::*;
use dash::*;
use esp::*;
use gamepad::*;
use input::*;
use light::*;
use plain::*;
use progress::*;
use track::*;
use trainer::*;

fn main() {
    let config = Config::default();
    let sensor_count = config.sensor_count;
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(config)
        .insert_resource(Trainer {
            sensor_count,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        // .insert_resource(bevy_atmosphere::AtmosphereMat::default())
        // .add_plugin(bevy_atmosphere::AtmospherePlugin {
        //     dynamic: false,
        //     sky_radius: 1000.0,
        // })
        .add_startup_system(camera_start_system)
        .add_system(camera_controller_system)
        .add_system(camera_switch_system)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin {
        //     // | DebugRenderMode::COLLIDER_AABBS
        //     mode: DebugRenderMode::COLLIDER_SHAPES
        //         | DebugRenderMode::RIGID_BODY_AXES
        //         | DebugRenderMode::JOINTS
        //         | DebugRenderMode::CONTACTS
        //         | DebugRenderMode::SOLVER_CONTACTS,
        //     ..default()
        // })
        // .add_plugin(PolylinePlugin)
        .add_plugin(DebugLinesPlugin::with_depth_test(true))
        // .add_plugins(DefaultPickingPlugins)
        // .add_plugin(DebugCursorPickingPlugin)
        .init_resource::<GamepadLobby>()
        .add_startup_system(plain_start_system)
        .add_startup_system(track_start_system)
        .add_startup_system(track_decorations_start_system)
        .add_startup_system(track_polyline_start_system)
        .add_startup_system(light_start_system)
        .add_startup_system(car_start_system)
        .add_startup_system(dash_speed_start_system)
        .add_startup_system(dash_fps_start_system)
        .add_startup_system(dfdx_start_system)
        .add_system(esp_system)
        .add_system(car_brain_system)
        .add_system(trainer_system)
        .add_system(dash_fps_system)
        .add_system(dash_leaderboard_system)
        .add_system(dash_speed_update_system)
        // .add_system(gamepad_input_system)
        .add_system(arrow_input_system)
        .add_system(reset_pos_system)
        .add_system(reset_collider_system)
        .add_system(progress_system)
        .add_system(reset_spawn_key_system)
        .add_system_to_stage(CoreStage::PreUpdate, gamepad_stage_preupdate_system)
        // .add_system_to_stage(CoreStage::PostUpdate, display_events_system)
        .run();
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
