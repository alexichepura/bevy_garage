#![feature(slice_flatten)]
mod config;
mod dash;
pub mod font;
mod input;
#[cfg(feature = "virtual_joystick")]
pub mod joystick;
mod spawn;
use std::num::NonZeroUsize;

use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, pbr::DirectionalLightShadowMap, prelude::*};
use bevy_garage_car::{aero_system, car_start_system, esp_system, CarRes, CarSet};
use bevy_garage_light::{animate_light_direction, light_start_system};
use bevy_garage_track::{track_polyline_start_system, SpawnCarOnTrackEvent, TrackPlugin};
use bevy_rapier3d::prelude::*;
use config::*;
use dash::*;
use font::*;
use input::*;
use spawn::*;

fn rapier_config_start_system(mut c: ResMut<RapierContext>) {
    c.integration_parameters.num_solver_iterations = NonZeroUsize::new(4).unwrap();
    c.integration_parameters.num_internal_pgs_iterations = 48;
    c.integration_parameters.num_additional_friction_iterations = 4;
    c.integration_parameters.erp = 0.99;
    // c.integration_parameters.joint_erp = 0.95;
    dbg!(c.integration_parameters);
}

pub fn car_app(app: &mut App) -> &mut App {
    #[cfg(feature = "nn")]
    let esp_run_after: CarSet = CarSet::NeuralNetwork;
    #[cfg(not(feature = "nn"))]
    let esp_run_after: CarSet = CarSet::Input;

    app.init_resource::<FontHandle>()
        .insert_resource(RapierConfiguration {
            // https://github.com/dimforge/bevy_rapier/pull/474
            timestep_mode: TimestepMode::Variable {
                max_dt: 1. / 60.,
                time_scale: 1.,
                substeps: 5,
            },
            ..default()
        })
        .insert_resource(Msaa::Sample4)
        .insert_resource(Config::default())
        .insert_resource(CarRes::default())
        .insert_resource(DirectionalLightShadowMap::default())
        .add_plugins((
            FrameTimeDiagnosticsPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::default(),
            TrackPlugin,
            RapierDebugRenderPlugin {
                enabled: false,
                style: DebugRenderStyle {
                    rigid_body_axes_length: 0.5,
                    ..default()
                },
                mode: DebugRenderMode::COLLIDER_SHAPES
                    | DebugRenderMode::RIGID_BODY_AXES
                    | DebugRenderMode::JOINTS
                    | DebugRenderMode::CONTACTS
                    | DebugRenderMode::SOLVER_CONTACTS,
                ..default()
            },
        ))
        .add_event::<SpawnCarOnTrackEvent>()
        .add_systems(
            Startup,
            (
                car_start_system.after(track_polyline_start_system),
                spawn_car_start_system.after(car_start_system),
                light_start_system,
                dash_start_system,
                rapier_config_start_system,
            ),
        )
        .add_systems(
            Update,
            (
                spawn_car_system,
                aero_system.in_set(CarSet::Input),
                input_system.in_set(CarSet::Input),
                esp_system.in_set(CarSet::Esp).after(esp_run_after),
                animate_light_direction,
                dash_fps_system,
                dash_speed_update_system,
            ),
        );

    #[cfg(feature = "dsp")]
    {
        app.add_plugins(bevy_garage_dsp::EngineSoundPlugin);
    }
    #[cfg(feature = "nn")]
    {
        app.add_plugins(bevy_garage_nn::NeuralNetworkPlugin);
    }

    app
}
