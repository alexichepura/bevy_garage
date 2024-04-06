use bevy::{prelude::*, window::WindowResolution};
use bevy_garage::car_app;
use bevy_garage_camera::CarCameraPlugin;

fn main() {
    let mut app = App::new();
    #[cfg(not(target_arch = "wasm32"))]
    let res = WindowResolution::default();
    #[cfg(target_arch = "wasm32")]
    let res = WindowResolution::new(720., 360.);
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Bevy Garage".to_string(),
                resolution: res,
                canvas: Some("#bevy-garage".to_string()),
                ..default()
            }),
            ..default()
        }),
        CarCameraPlugin,
    ));

    car_app(&mut app).run();
}
