mod camera;
mod car;
mod config;
mod dash;
mod dqn;
mod esp;
mod gamepad;
mod gradient;
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
use dqn::*;
use esp::*;
use gamepad::*;
use gradient::*;
use input::*;
use light::*;
use nn::{dqn_bevy::*, dqn_dash_update::*};
use progress::*;
use track::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(Config::default())
        .insert_resource(CameraConfig::default())
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
        .add_startup_system(dqn_start_system.exclusive_system())
        .add_startup_system(gradient_vis_start_system)
        .add_startup_system(track_start_system)
        .add_startup_system(track_decorations_start_system)
        .add_startup_system(track_polyline_start_system)
        .add_startup_system(light_start_system)
        .add_startup_system(car_start_system)
        .add_startup_system(dash_speed_start_system)
        .add_startup_system(dash_fps_start_system)
        // .add_startup_system(rapier_config_start_system)
        .add_system(esp_system)
        .add_system(car_sensor_system)
        .add_system(dqn_system)
        .add_system(dqn_switch_system)
        .add_system(dqn_dash_update_system)
        .add_system(dash_fps_system)
        .add_system(dash_leaderboard_system)
        .add_system(dash_speed_update_system)
        // .add_system(gamepad_input_system)
        .add_system(arrow_input_system)
        .add_system(progress_system)
        .add_system(debug_system)
        .add_system_to_stage(CoreStage::PreUpdate, gamepad_stage_preupdate_system)
        // .add_system_to_stage(CoreStage::PostUpdate, display_events_system)
        .run();
}

// const ITERATIONS: usize = 1024;
// fn rapier_config_start_system(mut c: ResMut<RapierContext>) {
// c.integration_parameters.dt = 1.0 / 480.0;
// c.integration_parameters.min_ccd_dt = 1.0 / 240.0 / 100.0;
// c.integration_parameters.min_island_size = 128;
// c.integration_parameters.joint_damping_ratio = 5.;
// c.integration_parameters.damping_ratio = 5.;
// c.integration_parameters.prediction_distance = 0.000_001;
// c.integration_parameters.allowed_linear_error = 0.000_001;
// c.integration_parameters.erp = 0.8;
// c.integration_parameters.max_ccd_substeps = ITERATIONS;
// c.integration_parameters.max_velocity_iterations = ITERATIONS;
// c.integration_parameters.max_velocity_friction_iterations = ITERATIONS;
// c.integration_parameters.max_stabilization_iterations = ITERATIONS;
//     dbg!(c.integration_parameters);
// }

fn debug_system(mut debug_ctx: ResMut<DebugRenderContext>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::R) {
        debug_ctx.enabled = !debug_ctx.enabled;
    }
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
