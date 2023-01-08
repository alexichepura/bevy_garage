#![deny(unsafe_op_in_unsafe_fn)]

use bevy::prelude::*;
use bevy_rapier_car_sim::car_app;

use crate::gyro::CMMotionManager;
mod gyro;

#[derive(Resource)]
pub struct IosRes {
    pub cm: CMMotionManager,
}

#[bevy_main]
fn main() {
    let cm = CMMotionManager::new();
    dbg!(cm.isGyroAvailable());
    dbg!(cm.isGyroActive());
    cm.startGyroUpdates();
    dbg!(cm.isGyroAvailable());
    dbg!(cm.isGyroActive());
    if cm.isGyroAvailable() {
        let gyro_data = cm.gyroData();
        dbg!(gyro_data.rotationRate);
    }

    // let ios_res = IosRes { cm };

    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        window: WindowDescriptor {
            resizable: false,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        },
        ..default()
    }));
    // app.insert_resource(ios_res);
    app.add_system(gyro_system);
    car_app(&mut app).run();
}

fn gyro_system() {}

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
