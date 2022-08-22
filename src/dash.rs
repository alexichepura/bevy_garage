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

pub fn dash_fps_start_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let medium: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    let text_style = TextStyle {
        font: medium.clone(),
        font_size: 16.0,
        color: Color::BLACK,
    };
    let text_section = TextSection {
        value: "".to_string(),
        style: text_style.clone(),
    };
    let sections = vec![text_section.clone()];

    let get_style = |top: f32| -> Style {
        return Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(top),
                left: Val::Px(2.0),
                ..default()
            },
            ..default()
        };
    };

    commands
        .spawn_bundle(TextBundle {
            style: get_style(40.),
            text: Text {
                sections: sections.clone(),
                ..default()
            },
            ..default()
        })
        .insert(TrainerGenerationText);
    commands
        .spawn_bundle(TextBundle {
            style: get_style(20.),
            text: Text {
                sections: sections.clone(),
                ..default()
            },
            ..default()
        })
        .insert(TrainerRecordDistanceText);
    commands
        .spawn_bundle(TextBundle {
            style: get_style(60.),
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: text_style.clone(),
                }],
                ..default()
            },
            ..default()
        })
        .insert(Leaderboard);
    commands
        .spawn_bundle(TextBundle {
            style: get_style(2.),
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: text_style.clone(),
                }],
                ..default()
            },
            ..default()
        })
        .insert(FpsText);
}

pub fn dash_leaderboard_system(
    q_cars: Query<&Car>,
    mut q_leaderboard: Query<&mut Text, With<Leaderboard>>,
) {
    let mut text_string: String = "".to_string();
    for car in q_cars.iter() {
        text_string = text_string + &car.meters.round().to_string() + " ";
    }
    let mut text = q_leaderboard.single_mut();
    text.sections[0].value = format!("leaderboard {:?}", text_string.as_str());
}
pub fn dash_fps_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[0].value = format!("fps {:.0}", average);
            }
        }
    }
}

pub fn dash_speed_start_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let medium: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    let style = TextStyle {
        font: medium.clone(),
        font_size: 16.0,
        color: Color::BLACK,
    };
    let get_style = |top: f32| -> Style {
        return Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(top),
                left: Val::Px(2.0),
                ..default()
            },
            ..default()
        };
    };
    commands
        .spawn_bundle(TextBundle {
            style: get_style(80.),
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: style.clone(),
                }],
                ..default()
            },
            ..default()
        })
        .insert(MpsText);
    commands
        .spawn_bundle(TextBundle {
            style: get_style(100.),
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: style.clone(),
                }],
                ..default()
            },
            ..default()
        })
        .insert(KmphText);
}

pub fn dash_speed_update_system(
    mut texts: ParamSet<(
        Query<&mut Text, With<MpsText>>,
        Query<&mut Text, With<KmphText>>,
    )>,
    mut cars: Query<(&Velocity, &Car, With<HID>)>,
    wheels: Query<(&Velocity, &ExternalForce), With<Wheel>>,
) {
    let (velocity, car, _) = cars.single_mut();
    let mps = velocity.linvel.length();
    let kmph = mps * 3.6;
    texts.p0().single_mut().sections[0].value = format!("mps {:.1}", mps);
    texts.p1().single_mut().sections[0].value = format!("kmph {:.1}", kmph);
    let mut v_msg: String = "".to_string();
    let mut f_msg: String = "".to_string();
    for wheel_entity in car.wheels.iter() {
        if let Ok((v, f)) = wheels.get(*wheel_entity) {
            v_msg = v_msg + &format!("{:.1} ", v.angvel.length());
            f_msg = f_msg + &format!("{:.1} ", f.torque.length());
        }
    }
}
