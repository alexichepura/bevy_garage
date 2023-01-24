use bevy::prelude::*;
use bevy_rapier_car_sim::{camera::CarCameraPlugin, car_app};

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            title: "bevy car sim".to_string(),
            width: 720.,
            height: 640.,
            // monitor: MonitorSelection::Index(1),
            position: WindowPosition::Centered,
            fit_canvas_to_parent: true,
            ..default()
        },
        ..default()
    }));
    app.add_plugin(CarCameraPlugin);

    let fps: f32 = if cfg!(target_arch = "wasm32") {
        60.
    } else {
        120.
    };
    car_app(&mut app, fps).run();
}
