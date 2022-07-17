use bevy::prelude::*;
use bevy::{diagnostic::Diagnostics, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_rapier3d::prelude::*;

use crate::car::{Car, Wheel};

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct MetersPerSecondText;

#[derive(Component)]
pub struct KilometersPerHourText;

#[derive(Component)]
pub struct WheelsWText;

#[derive(Component)]
pub struct WheelsTorqueText;

#[derive(Component)]
pub struct MassText;

pub fn dash_fps_start_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bold: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let medium: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: bold.clone(),
                            font_size: 60.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: medium.clone(),
                            font_size: 60.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(FpsText);
}

pub fn dash_fps_update_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{:.1}", average);
            }
        }
    }
}

pub fn dash_speed_start_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(UiCameraBundle::default());
    let bold: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let medium: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(5.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: medium.clone(),
                            font_size: 40.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: "m/s".to_string(),
                        style: TextStyle {
                            font: bold.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(MetersPerSecondText);
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(40.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: medium.clone(),
                            font_size: 40.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: "km/h".to_string(),
                        style: TextStyle {
                            font: bold.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(KilometersPerHourText);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(120.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: medium.clone(),
                            font_size: 40.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: "w".to_string(),
                        style: TextStyle {
                            font: bold.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(WheelsWText);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(90.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: medium.clone(),
                            font_size: 40.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: "t".to_string(),
                        style: TextStyle {
                            font: bold.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(WheelsTorqueText);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(160.0),
                    right: Val::Px(15.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: medium.clone(),
                            font_size: 40.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: "kg".to_string(),
                        style: TextStyle {
                            font: bold.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(MassText);
}

pub fn dash_speed_update_system(
    mut texts: ParamSet<(
        Query<&mut Text, With<MetersPerSecondText>>,
        Query<&mut Text, With<KilometersPerHourText>>,
        Query<&mut Text, With<MassText>>,
        Query<&mut Text, With<WheelsWText>>,
        Query<&mut Text, With<WheelsTorqueText>>,
    )>,
    mut cars: Query<(&Velocity, &ReadMassProperties, With<Car>)>,
    mut wheels: Query<(&Velocity, &ExternalForce, With<Wheel>)>,
) {
    let (velocity, mass_props, _) = cars.single_mut();
    let mps = velocity.linvel.length();
    texts.p0().single_mut().sections[0].value = format!("{:.1}", mps);

    let kmph = mps * 3.6;
    texts.p1().single_mut().sections[0].value = format!("{:.1}", kmph);

    texts.p2().single_mut().sections[0].value = format!("{:.1}", mass_props.0.mass);

    let mut v_msg: String = "".to_string();
    let mut f_msg: String = "".to_string();
    for (v, f, _wheel) in wheels.iter_mut() {
        let v_s = format!("{:.1} ", v.angvel.length());
        v_msg = v_msg + &v_s;
        let f_s = format!("{:.1} ", f.torque.length());
        f_msg = f_msg + &f_s;
    }
    texts.p3().single_mut().sections[0].value = v_msg;
    texts.p4().single_mut().sections[0].value = f_msg;
}
