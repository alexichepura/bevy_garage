use bevy::prelude::*;
use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
use bevy_prototype_debug_lines::DebugLinesPlugin;
use bevy_rapier3d::prelude::*;
use bevy_rapier_car_sim::car_app;

const FPS: f32 = 120.;
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "car sim deep learning".to_string(),
            width: 720.,
            height: 640.,
            // monitor: MonitorSelection::Index(1),
            position: WindowPosition::Centered,
            fit_canvas_to_parent: true,
            // canvas: Some("#bevy".to_string()),
            ..default()
        },
        ..default()
    }))
    .add_plugin(FramepacePlugin)
    .insert_resource(RapierConfiguration {
        timestep_mode: TimestepMode::Fixed {
            dt: 1. / FPS,
            substeps: 5,
        },
        // timestep_mode: TimestepMode::Variable {
        //     max_dt: 1. / FPS,
        //     substeps: 5,
        //     time_scale: 1.,
        // },
        // timestep_mode: TimestepMode::Interpolated {
        //     dt: 1. / FPS,
        //     substeps: 5,
        //     time_scale: 1.,
        // },
        ..default()
    })
    .insert_resource(FramepaceSettings {
        limiter: Limiter::from_framerate(FPS as f64),
        // limiter: Limiter::Auto,
        ..default()
    });

    car_app(&mut app).run();
}
