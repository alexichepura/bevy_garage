use bevy::openxr::XrPawn;
use bevy::prelude::*;
use bevy_garage::camera::{CameraConfig, CameraMode};
use bevy_garage::car::HID;
use core::f32::consts::PI;

// fn camera_position(mut q: Query<(&mut Transform, &mut GlobalTransform, &XrPawn)>) {
//     for (mut transform, _global, _) in q.iter_mut() {
//         transform.translation = Vec3::new(1., 0., 1.);
//     }
// }

pub fn camera_controller_system(
    config: Res<CameraConfig>,
    mut pset: ParamSet<(
        Query<(&mut Transform, &mut GlobalTransform, &XrPawn)>,
        Query<&Transform, With<HID>>,
        Query<&mut Transform, With<DirectionalLight>>,
    )>,
) {
    let follow_option: Option<Transform> = match config.mode {
        CameraMode::Free => None,
        CameraMode::Follow(_, from, at) => {
            if let Ok(car_tf) = pset.p1().get_single() {
                let mut tf = car_tf.clone();
                tf.translation += tf.rotation.mul_vec3(from);
                tf.rotate(Quat::from_rotation_y(-PI));
                // tf.look_at(car_tf.translation + at, Vec3::Y);
                Some(tf)
            } else {
                None
            }
        }
    };
    if let Some(tf) = follow_option {
        let mut p0 = pset.p0();
        let (mut camera_tf, _, _) = p0.single_mut();
        *camera_tf = tf;
    } else {
        return;
    };
}
