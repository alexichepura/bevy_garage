mod api_client;
pub mod camera;
pub mod car;
mod config;
mod dash;
mod esp;
pub mod font;
mod gamepad;
mod input;
mod light;
// mod mesh;
mod nn;
mod progress;
mod track;
use api_client::*;
use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, pbr::DirectionalLightShadowMap, prelude::*};
use bevy_rapier3d::prelude::*;
use car::*;
use config::*;
use dash::*;
use esp::*;
use font::*;
use gamepad::*;
use input::*;
use light::*;
use nn::{dqn::dqn_system, dqn_bevy::*};
use progress::*;
use track::*;

fn rapier_config_start_system(mut c: ResMut<RapierContext>) {
    // c.integration_parameters.max_velocity_iterations = 8;
    // c.integration_parameters.max_velocity_friction_iterations = 16;
    // c.integration_parameters.max_stabilization_iterations = 16;
    c.integration_parameters.allowed_linear_error = 0.0001;
    c.integration_parameters.erp = 0.99;
    dbg!(c.integration_parameters);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum CarSimLabel {
    Input,
    Brain,
    Esp,
}

pub fn car_app(app: &mut App) -> &mut App {
    app.add_event::<StreamEvent>()
        .init_resource::<FontHandle>()
        .insert_resource(RapierConfiguration {
            timestep_mode: TimestepMode::Variable {
                max_dt: 1. / 30.,
                time_scale: 1.,
                substeps: 30,
            },
            ..default()
        })
        .insert_resource(DqnResource::default())
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(Config::default())
        .insert_resource(DirectionalLightShadowMap::default())
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .init_resource::<GamepadLobby>()
        .add_startup_system(dqn_exclusive_start_system)
        .add_startup_system(track_start_system)
        .add_startup_system(track_decorations_start_system)
        .add_startup_system(track_polyline_start_system)
        .add_startup_system(car_start_system.after(track_polyline_start_system))
        .add_startup_system(light_start_system)
        .add_startup_system(dash_speed_start_system)
        .add_startup_system(dash_fps_start_system)
        .add_startup_system(rapier_config_start_system)
        .add_system_to_stage(CoreStage::PreUpdate, gamepad_stage_preupdate_system)
        .add_system(aero_system.label(CarSimLabel::Input))
        .add_system(keyboard_input_system.label(CarSimLabel::Input))
        .add_system(gamepad_input_system.label(CarSimLabel::Input))
        .add_system(car_sensor_system.label(CarSimLabel::Input))
        .add_system(progress_system.label(CarSimLabel::Input))
        .add_system(
            dqn_system
                .label(CarSimLabel::Brain)
                .after(CarSimLabel::Input),
        )
        .add_system(esp_system.label(CarSimLabel::Esp).after(CarSimLabel::Brain))
        .add_system(dqn_dash_update_system)
        .add_system(dash_leaderboard_system)
        .add_system(dash_fps_system)
        .add_system(dash_speed_update_system)
        .add_startup_system(api_start_system)
        .add_system(api_read_stream_event_writer_system)
        .add_system(api_event_reader_system);

    #[cfg(feature = "debug_lines")]
    {
        use bevy_prototype_debug_lines::DebugLinesPlugin;
        app.add_plugin(DebugLinesPlugin::with_depth_test(true))
            .add_plugin(RapierDebugRenderPlugin {
                enabled: false,
                style: DebugRenderStyle {
                    rigid_body_axes_length: 0.5,
                    // subdivisions: 50,
                    ..default()
                },
                // | DebugRenderMode::COLLIDER_AABBS
                mode: DebugRenderMode::COLLIDER_SHAPES
                    | DebugRenderMode::RIGID_BODY_AXES
                    | DebugRenderMode::JOINTS
                    | DebugRenderMode::CONTACTS
                    | DebugRenderMode::SOLVER_CONTACTS,
                ..default()
            });
    }

    #[cfg(feature = "bevy_atmosphere")]
    {
        use bevy_atmosphere::prelude::*;
        app.insert_resource(AtmosphereModel::new(Nishita {
            sun_position: Vec3::new(0.0, 1.0, 1.0),
            ..default()
        }))
        .add_plugin(AtmospherePlugin);
    }

    app
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
