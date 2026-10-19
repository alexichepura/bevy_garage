//! Renders the game for a few seconds and saves a screenshot of the primary window.
//!
//! Useful as a quick visual smoke test (e.g. to verify the HUD and custom track
//! materials still render after an engine upgrade):
//!
//! ```sh
//! cargo run --example screenshot
//! ```
//!
//! Note: on some platforms the window must be visible (not fully occluded) for
//! the swapchain capture to contain the rendered frame.

use bevy::{
    prelude::*,
    render::view::screenshot::{save_to_disk, Screenshot},
    window::{Window, WindowPlugin, WindowResolution},
};
use bevy_garage::car_app;
use bevy_garage_camera::CarCameraPlugin;

const SETTLE_TIME: f32 = 3.0;
const EXIT_AFTER: f32 = SETTLE_TIME + 1.0;

fn main() {
    let mut app = App::new();
    app.add_plugins((
        DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(720, 360),
                ..default()
            }),
            ..default()
        }),
        CarCameraPlugin,
    ));
    car_app(&mut app)
        .add_systems(Update, screenshot_then_exit)
        .run();
}

fn screenshot_then_exit(
    mut commands: Commands,
    time: Res<Time>,
    mut state: Local<u8>,
    mut exit: MessageWriter<AppExit>,
) {
    match *state {
        0 if time.elapsed_secs() > SETTLE_TIME => {
            *state = 1;
            commands
                .spawn(Screenshot::primary_window())
                .observe(save_to_disk("screenshot.png"));
        }
        1 if time.elapsed_secs() > EXIT_AFTER => {
            *state = 2;
            println!("wrote screenshot.png");
            exit.write(AppExit::Success);
        }
        _ => {}
    }
}
