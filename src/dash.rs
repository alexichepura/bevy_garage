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
pub struct Leaderboard;
#[derive(Component)]
pub struct TrainerRecordDistanceText;
#[derive(Component)]
pub struct TrainerGenerationText;

pub fn dash_leaderboard_system(
    q_cars: Query<&Car>,
    mut q_leaderboard: Query<&mut Text, With<Leaderboard>>,
) {
    let mut text_string: String = "".to_string();
    for car in q_cars.iter() {
        let distance = match car.meters {
            x => x - car.init_meters,
        };
        text_string = text_string + &distance.round().to_string() + " ";
    }
    let mut text = q_leaderboard.single_mut();
    text.sections[0].value = format!("{}m", text_string.as_str().trim_end());
}
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
    let height = Val::Px(70.);
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
                        size: Size::new(Val::Px(120.), height.clone()),
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
                                    value: "fps".to_string(),
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
                                    top: Val::Px(4.),
                                    right: Val::Px(4.),
                                    ..default()
                                },
                                ..default()
                            },
                            text: Text {
                                alignment: TextAlignment::Right,
                                sections: vec![TextSection {
                                    value: "d".to_string(),
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
                        .insert(Leaderboard);
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
                        .insert(TrainerRecordDistanceText);
                });
        });
}

pub fn dash_speed_update_system(
    mut texts: ParamSet<(
        Query<&mut Text, With<MpsText>>,
        Query<&mut Text, With<KmphText>>,
    )>,
    mut cars: Query<(&Velocity, &Car, With<HID>)>,
) {
    for (velocity, _car, _) in cars.iter_mut() {
        let mps = velocity.linvel.length();
        let kmph = mps * 3.6;
        texts.p0().single_mut().sections[0].value = format!("{:.1}m/s", mps);
        texts.p1().single_mut().sections[0].value = format!("{:.1}km/h", kmph);
    }
}
