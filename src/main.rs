use bevy::prelude::*;
use bevy_garage::{camera::CarCameraPlugin, car_app};
use wgpu::{AddressMode, SamplerDescriptor};

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    title: "Bevy Garage".to_string(),
                    width: 720.,
                    height: 640.,
                    // monitor: MonitorSelection::Index(1),
                    position: WindowPosition::Centered,
                    fit_canvas_to_parent: true,
                    ..default()
                },
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
