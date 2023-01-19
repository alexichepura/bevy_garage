use bevy::prelude::*;
use bevy_rapier_car_sim::{car_app, CarSimLabel};
mod touch;
use touch::*;

#[bevy_main]
fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            resizable: false,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        },
        ..default()
    }));
    app.add_startup_system(touch_input_start_system);
    app.add_system(touch_input_system.label(CarSimLabel::Input));
    car_app(&mut app, 60.).run();
}

// fn touch_camera(
//     windows: ResMut<Windows>,
//     mut touches: EventReader<TouchInput>,
//     mut camera: Query<&mut Transform, With<Camera3d>>,
//     mut last_position: Local<Option<Vec2>>,
// ) {
//     for touch in touches.iter() {
//         if touch.phase == TouchPhase::Started {
//             *last_position = None;
//         }
//         if let Some(last_position) = *last_position {
//             let window = windows.primary();
//             let mut transform = camera.single_mut();
//             *transform = Transform::from_xyz(
//                 transform.translation.x
//                     + (touch.position.x - last_position.x) / window.width() * 5.0,
//                 transform.translation.y,
//                 transform.translation.z
//                     + (touch.position.y - last_position.y) / window.height() * 5.0,
//             )
//             .looking_at(Vec3::ZERO, Vec3::Y);
//         }
//         *last_position = Some(touch.position);
//     }
// }
