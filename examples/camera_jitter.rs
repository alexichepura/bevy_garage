//! Drives the car at full throttle and reports how fast it goes and how much
//! the camera-to-car distance varies while following it.
//!
//! A camera that lags one frame makes the measured distance alternate between
//! two values, so the spread grows noticeably at speed. With correct system
//! ordering the spread stays close to zero.
//!
//! ```sh
//! cargo run --release --example camera_jitter
//! ```

use bevy::{input::InputSystems, prelude::*};
use bevy_garage::car_app;
use bevy_garage_camera::CarCameraPlugin;
use bevy_garage_car::Player;
use bevy_rapier3d::prelude::Velocity;

#[derive(Default)]
struct Stats {
    min: f32,
    max: f32,
    max_speed: f32,
    frames: u32,
}

fn main() {
    let mut app = App::new();
    app.add_plugins((DefaultPlugins, CarCameraPlugin));
    car_app(&mut app)
        // Press ArrowUp after Bevy clears keyboard input for the frame, so the
        // regular car input system sees it and drives the car autonomously.
        .add_systems(PreUpdate, hold_throttle.after(InputSystems))
        .add_systems(Last, measure_camera_distance)
        .run();
}

fn hold_throttle(mut keys: ResMut<ButtonInput<KeyCode>>) {
    keys.press(KeyCode::ArrowUp);
}

fn measure_camera_distance(
    time: Res<Time>,
    camera: Query<&GlobalTransform, With<Camera>>,
    player: Query<(&GlobalTransform, Option<&Velocity>), With<Player>>,
    mut stats: Local<Option<Stats>>,
    mut exit: MessageWriter<AppExit>,
) {
    // Let the car get up to speed before measuring.
    if time.elapsed_secs() < 2.0 {
        return;
    }
    let (Ok(camera), Ok((player, velocity))) = (camera.single(), player.single()) else {
        return;
    };
    let distance = camera.translation().distance(player.translation());
    let speed = velocity.map_or(0.0, |velocity| velocity.linear.length());

    let stats = stats.get_or_insert_with(|| Stats {
        min: distance,
        max: distance,
        max_speed: speed,
        frames: 0,
    });
    stats.min = stats.min.min(distance);
    stats.max = stats.max.max(distance);
    stats.max_speed = stats.max_speed.max(speed);
    stats.frames += 1;

    if time.elapsed_secs() > 8.0 {
        println!(
            "max speed={:.2} m/s ({:.1} km/h), camera distance: min={:.6} max={:.6} spread={:.6} frames={}",
            stats.max_speed,
            stats.max_speed * 3.6,
            stats.min,
            stats.max,
            stats.max - stats.min,
            stats.frames,
        );
        exit.write(AppExit::Success);
    }
}
