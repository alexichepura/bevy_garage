use std::f32::consts::PI;

use bevy::{
    app::AppExit,
    openxr::camera::XrPawn,
    prelude::*,
    utils::Duration,
    xr::{
        XrActionSet, XrHandType, XrReferenceSpaceType, XrSessionMode, XrSystem, XrTrackingSource,
        XrVibrationEvent, XrVibrationEventType,
    },
    DefaultPlugins,
};
use bevy_garage::{
    camera::{CameraConfig, CameraFollowView},
    car::{Car, HID},
    car_app,
};
use camera::camera_controller_system;
mod camera;

pub fn game_main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    std::env::set_var("RUST_LOG", "bevy_openxr=info");
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_startup_system(xr_startup);
    app.insert_resource(CameraConfig::from_view(CameraFollowView::Windshield));
    car_app(&mut app);
    app.add_system(interaction);
    app.add_system(camera_controller_system);
    app.run();
}

fn xr_startup(mut xr_system: ResMut<XrSystem>, mut app_exit_events: EventWriter<AppExit>) {
    if xr_system.is_session_mode_supported(XrSessionMode::ImmersiveVR) {
        xr_system.request_session_mode(XrSessionMode::ImmersiveVR);
    } else {
        bevy::log::error!("The XR device does not support immersive VR mode");
        app_exit_events.send(AppExit)
    }

    println!("startup done");
}

#[derive(Component, PartialEq, Eq)]
enum Hand {
    Left,
    Right,
}

fn interaction(
    mut c: Commands,
    action_set: Option<Res<XrActionSet>>,
    mut tracking_source: ResMut<XrTrackingSource>,
    mut vibration_events: EventWriter<XrVibrationEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pawn: Query<Entity, With<XrPawn>>,
    mut pset: ParamSet<(
        // Query<(&mut Transform, &mut CameraController), With<Camera>>,
        // Query<&Transform, With<HID>>,
        // Query<&mut Transform, With<DirectionalLight>>,
        Query<(&Hand, &mut Transform, &GlobalTransform)>,
        Query<(&mut Car, &Transform, With<HID>)>,
    )>,
    // mut hands: Query<(&Hand, &mut Transform, &GlobalTransform)>,
    // mut cars: Query<(&mut Car, &Transform, With<HID>)>,
) {
    if tracking_source.reference_space_type() != XrReferenceSpaceType::Stage {
        tracking_source.set_reference_space_type(XrReferenceSpaceType::Stage);
    }
    let pawn = match pawn.get_single() {
        Ok(pawn) => pawn,
        Err(_) => return,
    };

    for (hand, button, squeeze) in [
        (
            XrHandType::Left,
            "left_x".to_owned(),
            "left_trigger".to_owned(),
        ),
        (
            XrHandType::Right,
            "right_a".to_owned(),
            "right_trigger".to_owned(),
        ),
    ] {
        let action_set = match action_set {
            Some(ref s) => s,
            None => continue,
        };
        if action_set.button_just_pressed(&button) {
            println!("Short haptic click");
            vibration_events.send(XrVibrationEvent {
                hand,
                command: XrVibrationEventType::Apply {
                    duration: Duration::from_millis(2),
                    frequency: 3000_f32, // Hz
                    amplitude: 1_f32,
                },
            });
        } else {
            let squeeze_value = action_set.scalar_value(&squeeze);
            if squeeze_value > 0.0 {
                for (mut car, _transform, _hid) in pset.p1().iter_mut() {
                    if hand == XrHandType::Right {
                        car.gas = squeeze_value;
                        car.brake = 0.;
                    } else {
                        car.brake = squeeze_value;
                        car.gas = 0.;
                    }
                }
            }
        }
    }

    let [left_pose, right_pose] = tracking_source.hands_pose();
    if let Some(pose) = left_pose {
        if pset
            .p0()
            .iter()
            .find(|hand| hand.0 == &Hand::Left)
            .is_none()
        {
            let cube = c
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::rgb(0.1, 0.1, 0.8).into()),
                    transform: Transform::default().with_scale([0.1, 0.1, 0.1].into()),
                    ..Default::default()
                })
                .id();
            let hand = c
                .spawn(TransformBundle::default())
                .insert(VisibilityBundle::default())
                .add_child(cube)
                .insert(Hand::Left)
                .id();
            c.entity(pawn).add_child(hand);

            dbg!("spawned left hand");
        }
        for mut hand in pset.p0().iter_mut().filter(|hand| hand.0 == &Hand::Left) {
            *hand.1 = Transform {
                translation: pose.transform.position,
                rotation: pose.transform.orientation,
                scale: Vec3::ONE,
            };
        }
    }
    if let Some(pose) = right_pose {
        if pset
            .p0()
            .iter()
            .find(|hand| hand.0 == &Hand::Right)
            .is_none()
        {
            let cube = c
                .spawn(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
                    material: materials.add(Color::rgb(0.8, 0.1, 0.2).into()),
                    transform: Transform::default().with_scale([0.1, 0.1, 0.1].into()),
                    ..Default::default()
                })
                .id();
            let hand = c
                .spawn(TransformBundle::default())
                .insert(VisibilityBundle::default())
                .add_child(cube)
                .insert(Hand::Right)
                .id();
            c.entity(pawn).add_child(hand);

            dbg!("spawned right hand");
        }
        for mut hand in pset.p0().iter_mut().filter(|hand| hand.0 == &Hand::Right) {
            *hand.1 = Transform {
                translation: pose.transform.position,
                rotation: pose.transform.orientation,
                scale: Vec3::ONE,
            };
        }
        for (mut car, _transform, _hid) in pset.p1().iter_mut() {
            let (yaw, pitch, _roll) = pose.transform.orientation.to_euler(EulerRot::YXZ);
            car.steering = -yaw / PI;
        }
    }
}
