use crate::car::HID;
use crate::config::Config;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::render::camera::Projection;

pub struct CarCameraPlugin;

impl Plugin for CarCameraPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraConfig::default())
            .add_startup_system(camera_start_system)
            .add_system(camera_controller_system)
            // .add_system_to_stage(CoreStage::PreUpdate, camera_controller_system)
            .add_system(camera_switch_system);
    }
}

pub fn camera_start_system(mut commands: Commands, config: Res<Config>) {
    commands
        .spawn((
            Camera3dBundle {
                projection: Projection::from(PerspectiveProjection {
                    far: 5000.,
                    near: 0.01,
                    ..default()
                }),
                transform: Transform::from_translation(
                    Vec3::Y * 15. + config.quat.mul_vec3(-Vec3::Z * 30.),
                )
                .looking_at(Vec3::Y * 6., Vec3::Y),
                ..default()
            },
            #[cfg(feature = "bevy_atmosphere")]
            bevy_atmosphere::prelude::AtmosphereCamera::default(),
        ))
        .insert(CameraController::default());
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

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum CameraFollowView {
    Windshield,
    FrontWheel,
    Near,
    Mid,
    Far,
}
fn follow_props_by_mode(mode: &CameraFollowView) -> (Vec3, Vec3) {
    let look_from = match mode {
        CameraFollowView::Windshield => Vec3::new(0., -0.85, 0.26),
        CameraFollowView::Near => Vec3::new(0., 2., -5.),
        CameraFollowView::Mid => Vec3::new(0., 3., -10.),
        CameraFollowView::Far => Vec3::new(0., 5., -20.),
        CameraFollowView::FrontWheel => Vec3::new(-2.5, -0.2, 2.),
    };
    let look_at = match mode {
        CameraFollowView::Windshield => Vec3::new(0., 0., 0.), // TODO
        CameraFollowView::Near => Vec3::new(0., 1.5, 0.),
        CameraFollowView::Mid => Vec3::new(0., 2., 0.),
        CameraFollowView::Far => Vec3::new(0., 3., 0.),
        CameraFollowView::FrontWheel => Vec3::new(0., -0.2, 1.),
    };
    (look_from, look_at)
}
#[derive(PartialEq, Debug)]
pub enum CameraMode {
    Follow(CameraFollowView, Vec3, Vec3),
    Free,
}

#[derive(Resource)]
pub struct CameraConfig {
    pub mode: CameraMode,
    pub prev: Transform,
}

impl CameraConfig {
    pub fn from_view(view: CameraFollowView) -> Self {
        let (from, at) = follow_props_by_mode(&view);
        Self {
            mode: CameraMode::Follow(view, from, at),
            prev: Transform::IDENTITY,
        }
    }
    pub fn free(&mut self) {
        self.mode = CameraMode::Free;
    }
    fn follow_view(&mut self, view: CameraFollowView) {
        let (from, at) = follow_props_by_mode(&view);
        self.mode = CameraMode::Follow(view, from, at);
    }
    pub fn near(&mut self) {
        self.follow_view(CameraFollowView::Near);
    }
    pub fn mid(&mut self) {
        self.follow_view(CameraFollowView::Mid);
    }
    pub fn far(&mut self) {
        self.follow_view(CameraFollowView::Far);
    }
    pub fn wheel(&mut self) {
        self.follow_view(CameraFollowView::FrontWheel);
    }
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self::from_view(CameraFollowView::Near)
    }
}
pub fn camera_switch_system(mut config: ResMut<CameraConfig>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Key1) {
        config.near();
    }
    if input.just_pressed(KeyCode::Key2) {
        config.mid();
    }
    if input.just_pressed(KeyCode::Key3) {
        config.far();
    }
    if input.just_pressed(KeyCode::Key4) {
        config.wheel();
    }
    if input.just_pressed(KeyCode::Key0) {
        config.free();
    }
}

pub fn camera_controller_system(
    time: Res<Time>,
    mut config: ResMut<CameraConfig>,
    mut mouse_events: EventReader<MouseMotion>,
    key_input: Res<Input<KeyCode>>,
    mut pset: ParamSet<(
        Query<(&mut Transform, &mut CameraController), With<Camera>>,
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
                tf.look_at(car_tf.translation + at, Vec3::Y);
                Some(tf)
            } else {
                None
            }
        }
    };
    let tf: Transform = if let Some(tf) = follow_option {
        let mut p0 = pset.p0();
        let (_, mut options) = p0.single_mut();
        let (yaw, pitch, _roll) = tf.rotation.to_euler(EulerRot::YXZ);
        options.pitch = pitch;
        options.yaw = yaw;
        tf
    } else {
        let dt = time.delta_seconds();

        let mut mouse_delta = Vec2::ZERO;
        for mouse_event in mouse_events.iter() {
            mouse_delta += mouse_event.delta;
        }

        let mut p0 = pset.p0();
        let (tf, mut options) = p0.single_mut();

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

        let mut tf = tf.clone();
        let forward = tf.forward();
        let right = tf.right();
        tf.translation += options.velocity.x * dt * right
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
            tf.rotation = Quat::from_euler(EulerRot::ZYX, 0.0, yaw, pitch);
            options.pitch = pitch;
            options.yaw = yaw;
        }
        tf
    };

    let d_seconds = time.delta_seconds();
    let d_seconds_min = if d_seconds == 0.0 {
        1. / 120.
    } else {
        d_seconds
    };
    let prev = config.prev;
    let k = d_seconds_min * 100.;
    let new_translation = prev.translation.lerp(tf.translation, k);
    let new_rotation = prev.rotation.slerp(tf.rotation, k);
    config.prev.translation = new_translation;
    config.prev.rotation = new_rotation;
    // let new_translation = tf.translation;
    // let new_rotation = tf.rotation;

    let mut p0 = pset.p0();
    let (mut camera_tf, options) = p0.single_mut();
    camera_tf.translation = new_translation;
    camera_tf.rotation = new_rotation;
    let yaw = options.yaw;

    let mut p2 = pset.p2();
    let mut dlight_tf = p2.single_mut();
    let camera_xz = Vec3::new(tf.translation.x, dlight_tf.translation.y, tf.translation.z);
    let camera_dir = -Quat::from_rotation_y(yaw).mul_vec3(Vec3::Z);
    let light_shift = 50. * camera_dir;
    dlight_tf.translation = camera_xz + light_shift;
}
