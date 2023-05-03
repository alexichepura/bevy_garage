use crate::car::*;
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
pub struct LapText;

#[derive(Component)]
pub struct TrackPositionText;

#[derive(Component)]
pub struct RideDistanceText;

#[derive(Component)]
pub struct TrainerEpsilonText;

#[derive(Component)]
pub struct TrainerGenerationText;

pub fn dash_fps_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[0].value = format!("{:.0}fps", average);
            }
        }
    }
}

pub fn dash_start_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let medium: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    let height = Val::Px(90.);
    let width = Val::Px(150.);
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), height.clone()),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            let background_color: BackgroundColor = Color::rgba(0.15, 0.15, 0.15, 0.5).into();
            parent
                .spawn(NodeBundle {
                    background_color,
                    style: Style {
                        size: Size::new(width, height.clone()),
                        padding: UiRect::all(Val::Px(4.0)),
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::End,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent
                        .spawn(TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                position: UiRect {
                                    top: Val::Px(4.),
                                    left: Val::Px(4.),
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
                                        color: Color::YELLOW_GREEN,
                                    },
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(FpsText);
                    parent
                        .spawn(TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                position: UiRect {
                                    top: Val::Px(20.),
                                    left: Val::Px(4.),
                                    ..default()
                                },
                                ..default()
                            },
                            text: Text {
                                sections: vec![TextSection {
                                    value: "".to_string(),
                                    style: TextStyle {
                                        font: medium.clone(),
                                        font_size: 18.0,
                                        color: Color::YELLOW,
                                    },
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(LapText);
                    parent
                        .spawn(TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                position: UiRect {
                                    top: Val::Px(4.),
                                    right: Val::Px(4.),
                                    ..default()
                                },
                                ..default()
                            },
                            text: Text {
                                alignment: TextAlignment::Right,
                                sections: vec![TextSection {
                                    value: "".to_string(),
                                    style: TextStyle {
                                        font: medium.clone(),
                                        font_size: 16.0,
                                        color: Color::YELLOW,
                                    },
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(TrackPositionText);
                    parent
                        .spawn(TextBundle {
                            text: Text {
                                alignment: TextAlignment::Right,
                                sections: vec![TextSection {
                                    value: "".to_string(),
                                    style: TextStyle {
                                        font: medium.clone(),
                                        font_size: 18.0,
                                        color: Color::YELLOW,
                                    },
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(RideDistanceText);
                    parent
                        .spawn(TextBundle {
                            text: Text {
                                alignment: TextAlignment::Right,
                                sections: vec![TextSection {
                                    value: "".to_string(),
                                    style: TextStyle {
                                        font: medium.clone(),
                                        font_size: 24.0,
                                        color: Color::YELLOW_GREEN,
                                    },
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(MpsText);
                    parent
                        .spawn(TextBundle {
                            text: Text {
                                sections: vec![TextSection {
                                    value: "".to_string(),
                                    style: TextStyle {
                                        font: medium.clone(),
                                        font_size: 24.0,
                                        color: Color::YELLOW,
                                    },
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(KmphText);
                    parent
                        .spawn(TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                margin: UiRect {
                                    left: Val::Px(4.),
                                    ..default()
                                },
                                position: UiRect {
                                    top: Val::Px(4.),
                                    left: Val::Percent(100.),
                                    ..default()
                                },
                                ..default()
                            },
                            text: Text {
                                alignment: TextAlignment::Right,
                                sections: vec![TextSection {
                                    value: "".to_string(),
                                    style: TextStyle {
                                        font: medium.clone(),
                                        font_size: 14.0,
                                        color: Color::BLACK,
                                    },
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(TrainerGenerationText);
                    parent
                        .spawn(TextBundle {
                            style: Style {
                                position_type: PositionType::Absolute,
                                margin: UiRect {
                                    left: Val::Px(4.),
                                    ..default()
                                },
                                position: UiRect {
                                    top: Val::Px(20.),
                                    left: Val::Percent(100.),
                                    ..default()
                                },
                                ..default()
                            },
                            text: Text {
                                alignment: TextAlignment::Right,
                                sections: vec![TextSection {
                                    value: "".to_string(),
                                    style: TextStyle {
                                        font: medium.clone(),
                                        font_size: 14.0,
                                        color: Color::DARK_GRAY,
                                    },
                                }],
                                ..default()
                            },
                            ..default()
                        })
                        .insert(TrainerEpsilonText);
                });
        });
}

pub fn dash_speed_update_system(
    mut texts: ParamSet<(
        Query<&mut Text, With<MpsText>>,
        Query<&mut Text, With<KmphText>>,
        Query<&mut Text, With<TrackPositionText>>,
        Query<&mut Text, With<RideDistanceText>>,
        Query<&mut Text, With<LapText>>,
    )>,
    mut cars: Query<(&Velocity, &Car, With<HID>)>,
) {
    for (velocity, car, _) in cars.iter_mut() {
        let mps = velocity.linvel.length();
        let kmph = mps * 3.6;
        texts.p0().single_mut().sections[0].value = format!("{:.1}m/s", mps);
        texts.p1().single_mut().sections[0].value = format!("{:.1}km/h", kmph);

        texts.p2().single_mut().sections[0].value = format!("{:.1}m", car.track_position);

        let sign: &str = if car.ride_distance.is_sign_negative() {
            "-"
        } else {
            "+"
        };
        texts.p3().single_mut().sections[0].value =
            format!("{sign}{:.1}m", car.ride_distance.abs());

        texts.p4().single_mut().sections[0].value = format!("lap {}", car.lap);
    }
}
