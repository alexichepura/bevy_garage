use bevy::{prelude::*, window::WindowResolution};
use bevy_garage::{camera::CarCameraPlugin, car_app};
use wgpu::{AddressMode, SamplerDescriptor};

fn main() {
    let mut app = App::new();
    #[cfg(not(target_arch = "wasm32"))]
    let res = WindowResolution::default();
    #[cfg(target_arch = "wasm32")]
    let res = WindowResolution::new(720., 360.);
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Bevy Garage".to_string(),
                    resolution: res,
                    canvas: Some("#bevy-garage".to_string()),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin {
                default_sampler: SamplerDescriptor {
                    address_mode_u: AddressMode::Repeat,
                    address_mode_v: AddressMode::Repeat,
                    address_mode_w: AddressMode::Repeat,
                    ..Default::default()
                },
                ..default()
            }),
    );
    app.add_plugin(CarCameraPlugin);
    car_app(&mut app).run();
}
