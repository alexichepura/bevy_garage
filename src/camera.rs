use crate::car::HID;
use crate::config::Config;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use core::f32::consts::PI;

pub fn camera_start_system(mut commands: Commands, config: Res<Config>) {
    commands
        .spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_translation(
                config.translation + Vec3::Y * 15. + config.quat.mul_vec3(-Vec3::Z * 30.),
            )
            .looking_at(Vec3::Y * 6., config.translation),
            ..default()
        })
        .insert(CameraController::default());
    println!(
        "Controls:
        WSAD   - forward/back/strafe left/right
        LShift - run
        E      - up
        Q      - down"
    );
}

#[derive(Component)]
pub struct CameraController {
    pub enabled: bool,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            sensitivity: 0.5,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::E,
            key_down: KeyCode::Q,
            key_run: KeyCode::LShift,
            walk_speed: 10.0,
            run_speed: 100.0,
            friction: 0.5,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::ZERO,
        }
    }
}

pub fn camera_switch_system(
    mut config: ResMut<Config>,
    input: Res<Input<KeyCode>>,
    query: Query<Entity, With<HID>>,
) {
    if input.just_pressed(KeyCode::Key1) {
        config.camera_follow = Some(query.single());
    }
    if input.just_pressed(KeyCode::Key0) {
        config.camera_follow = None;
    }
}

pub fn camera_controller_system(
    time: Res<Time>,
    config: Res<Config>,
    mut mouse_events: EventReader<MouseMotion>,
    key_input: Res<Input<KeyCode>>,
    mut transforms: ParamSet<(
        Query<(&mut Transform, &mut CameraController), With<Camera>>,
        Query<&Transform, With<HID>>,
    )>,
) {
    if let Some(e) = config.camera_follow {
        let p1 = transforms.p1();
        let car_transform = p1.get(e);
        if let Ok(car_transform) = car_transform {
            let mut tf = Transform::from_matrix(car_transform.compute_matrix());
            let shift_vec: Vec3 = tf.rotation.mul_vec3(Vec3::new(0., 5., -25.));
            tf.translation.x = tf.translation.x + shift_vec.x;
            tf.translation.y = tf.translation.y + shift_vec.y;
            tf.translation.z = tf.translation.z + shift_vec.z;
            tf.rotate(Quat::from_rotation_y(-PI));
            tf.look_at(car_transform.translation + Vec3::new(0., 2., 0.), Vec3::Y);
            for (mut cam_transform, _) in transforms.p0().iter_mut() {
                *cam_transform = tf;
            }
        }

        return;
    }
    let dt = time.delta_seconds();

    let mut mouse_delta = Vec2::ZERO;
    for mouse_event in mouse_events.iter() {
        mouse_delta += mouse_event.delta;
    }

    for (mut transform, mut options) in transforms.p0().iter_mut() {
        if !options.enabled {
            continue;
        }

        let mut axis_input = Vec3::ZERO;
        if key_input.pressed(options.key_forward) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(options.key_back) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(options.key_right) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(options.key_left) {
            axis_input.x -= 1.0;
        }
        if key_input.pressed(options.key_up) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(options.key_down) {
            axis_input.y -= 1.0;
        }

        if axis_input != Vec3::ZERO {
            let max_speed = if key_input.pressed(options.key_run) {
                options.run_speed
            } else {
                options.walk_speed
            };
            options.velocity = axis_input.normalize() * max_speed;
        } else {
            let friction = options.friction.clamp(0.0, 1.0);
            options.velocity *= 1.0 - friction;
            if options.velocity.length_squared() < 1e-6 {
                options.velocity = Vec3::ZERO;
            }
        }
        let forward = transform.forward();
        let right = transform.right();
        transform.translation += options.velocity.x * dt * right
            + options.velocity.y * dt * Vec3::Y
            + options.velocity.z * dt * forward;

        if mouse_delta != Vec2::ZERO {
            let (pitch, yaw) = (
                (options.pitch - mouse_delta.y * 0.5 * options.sensitivity * dt).clamp(
                    -0.99 * std::f32::consts::FRAC_PI_2,
                    0.99 * std::f32::consts::FRAC_PI_2,
                ),
                options.yaw - mouse_delta.x * options.sensitivity * dt,
            );
            transform.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, yaw, pitch);
            options.pitch = pitch;
            options.yaw = yaw;
        }
    }
}
