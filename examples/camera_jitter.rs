//! Drives the car at full throttle and reports how much the camera-to-car
//! distance varies while following it.
//!
//! A camera that lags one frame makes the measured distance alternate between
//! two values, so the spread grows noticeably at speed. With correct system
//! ordering the spread stays close to zero.
//!
//! ```sh
//! cargo run --release --example camera_jitter
//! ```

use bevy::prelude::*;
use bevy_garage::car_app;
use bevy_garage_camera::CarCameraPlugin;
use bevy_garage_car::Player;

#[derive(Default)]
struct Stats {
    min: f32,
    max: f32,
    frames: u32,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, CarCameraPlugin));
    car_app(&mut app)
        .add_systems(Update, hold_throttle)
        .add_systems(Last, measure_camera_distance)
        .run();
}

fn hold_throttle(mut keys: ResMut<ButtonInput<KeyCode>>) {
    keys.press(KeyCode::KeyW);
    keys.press(KeyCode::ShiftLeft);
}

fn measure_camera_distance(
    time: Res<Time>,
    camera: Query<&GlobalTransform, With<Camera>>,
    player: Query<&GlobalTransform, With<Player>>,
    mut stats: Local<Option<Stats>>,
    mut exit: MessageWriter<AppExit>,
) {
    // Let the car get up to speed before measuring.
    if time.elapsed_secs() < 2.0 {
        return;
    }
    let (Ok(camera), Ok(player)) = (camera.single(), player.single()) else {
        return;
    };
    let distance = camera.translation().distance(player.translation());
    let stats = stats.get_or_insert_with(|| Stats {
        min: distance,
        max: distance,
        frames: 0,
    });
    stats.min = stats.min.min(distance);
    stats.max = stats.max.max(distance);
    stats.frames += 1;

    if time.elapsed_secs() > 8.0 {
        println!(
            "camera distance: min={:.6} max={:.6} spread={:.6} frames={}",
            stats.min,
            stats.max,
            stats.max - stats.min,
            stats.frames,
        );
        exit.write(AppExit::Success);
    }
}
