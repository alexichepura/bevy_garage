use crate::{car::*, progress::*};
use bevy::prelude::*;
use bevy::{diagnostic::Diagnostics, diagnostic::FrameTimeDiagnosticsPlugin};
use bevy_rapier3d::prelude::*;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct MpsText;

#[derive(Component)]
pub struct KmphText;

#[derive(Component)]
pub struct WheelsWText;

#[derive(Component)]
pub struct WheelsTorqueText;

#[derive(Component)]
pub struct Leaderboard;

#[derive(Component)]
pub struct TrainerTimingText;
#[derive(Component)]
pub struct TrainerRecordDistanceText;
#[derive(Component)]
pub struct TrainerGenerationText;

pub fn dash_fps_start_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bold: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let medium: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(60.0),
                    left: Val::Px(2.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: medium.clone(),
                        font_size: 16.0,
                        color: Color::GOLD,
                    },
                }],
                ..default()
            },
            ..default()
        })
        .insert(TrainerGenerationText);
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(20.0),
                    left: Val::Px(2.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "distance record: ".to_string(),
                        style: TextStyle {
                            font: bold.clone(),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: medium.clone(),
                            font_size: 16.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(TrainerRecordDistanceText);
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(40.0),
                    left: Val::Px(2.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: TextStyle {
                        font: medium.clone(),
                        font_size: 16.0,
                        color: Color::GOLD,
                    },
                }],
                ..default()
            },
            ..default()
        })
        .insert(TrainerTimingText);
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(2.0),
                    left: Val::Px(2.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "leaderboard: ".to_string(),
                        style: TextStyle {
                            font: bold.clone(),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: medium.clone(),
                            font_size: 16.0,
                            color: Color::GOLD,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(Leaderboard);
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    top: Val::Px(2.0),
                    left: Val::Px(2.0),
                    ..default()
                },
                ..default()
            },
            text: Text {
                sections: vec![
                    TextSection {
                        value: "FPS: ".to_string(),
                        style: TextStyle {
                            font: bold.clone(),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    },
                    TextSection {
                        value: "".to_string(),
                        style: TextStyle {
                            font: medium.clone(),
                            font_size: 16.0,
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

pub fn dash_leaderboard_system(
    q_cars: Query<&CarProgress, With<CarProgress>>,
    mut q_leaderboard: Query<&mut Text, With<Leaderboard>>,
) {
    let mut text_string: String = "".to_string();
    for progress in q_cars.iter() {
        text_string = text_string + &progress.meters.round().to_string() + " ";
    }
    let mut text = q_leaderboard.single_mut();
    text.sections[1].value = text_string;
}
pub fn dash_fps_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[1].value = format!("{:.1}", average);
            }
        }
    }
}

pub fn dash_speed_start_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // commands.spawn_bundle(UiCameraBundle::default());
    let bold: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let medium: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
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
                            font_size: 16.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: "m/s".to_string(),
                        style: TextStyle {
                            font: bold.clone(),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(MpsText);
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(25.0),
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
                            font_size: 16.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: "km/h".to_string(),
                        style: TextStyle {
                            font: bold.clone(),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(KmphText);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: UiRect {
                    bottom: Val::Px(50.0),
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
                            font_size: 16.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: "w".to_string(),
                        style: TextStyle {
                            font: bold.clone(),
                            font_size: 16.0,
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
                position: UiRect {
                    bottom: Val::Px(70.0),
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
                            font_size: 16.0,
                            color: Color::GOLD,
                        },
                    },
                    TextSection {
                        value: "t".to_string(),
                        style: TextStyle {
                            font: bold.clone(),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..default()
            },
            ..default()
        })
        .insert(WheelsTorqueText);
}

pub fn dash_speed_update_system(
    mut texts: ParamSet<(
        Query<&mut Text, With<MpsText>>,
        Query<&mut Text, With<KmphText>>,
        Query<&mut Text, With<WheelsWText>>,
        Query<&mut Text, With<WheelsTorqueText>>,
    )>,
    mut cars: Query<(&Velocity, &Car, With<HID>)>,
    wheels: Query<(&Velocity, &ExternalForce), With<Wheel>>,
) {
    let (velocity, car, _) = cars.single_mut();

    let mps = velocity.linvel.length();
    let kmph = mps * 3.6;
    texts.p0().single_mut().sections[0].value = format!("{:.1}", mps);
    texts.p1().single_mut().sections[0].value = format!("{:.1}", kmph);

    let mut v_msg: String = "".to_string();
    let mut f_msg: String = "".to_string();

    for wheel_entity in car.wheels.iter() {
        if let Ok((v, f)) = wheels.get(*wheel_entity) {
            v_msg = v_msg + &format!("{:.1} ", v.angvel.length());
            f_msg = f_msg + &format!("{:.1} ", f.torque.length());
        }
    }

    texts.p2().single_mut().sections[0].value = v_msg;
    texts.p3().single_mut().sections[0].value = f_msg;
}
