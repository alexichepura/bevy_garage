use bevy::text::Font;
use bevy::{
    diagnostic::Diagnostics,
    diagnostic::FrameTimeDiagnosticsPlugin,
    math::Rect,
    prelude::{AssetServer, Color, Component, QuerySet, QueryState, TextBundle},
    prelude::{Query, With},
    text::{TextSection, TextStyle},
    ui::{AlignSelf, PositionType, Style, Val},
};
use bevy::{ecs::system::Res, prelude::Commands};
use bevy::{prelude::Handle, text::Text};
use bevy_rapier3d::prelude::{
    MassProperties, RigidBodyMassPropsComponent, RigidBodyVelocityComponent,
};

use crate::car::{Car, Wheel};

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct MetersPerSecondText;

#[derive(Component)]
pub struct KilometersPerHourText;

#[derive(Component)]
pub struct RotPerSecondText;

#[derive(Component)]
pub struct MassText;

pub fn dash_fps_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let bold: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    let medium: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
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
                ..Default::default()
            },
            ..Default::default()
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

pub fn dash_speed_system(mut commands: Commands, asset_server: Res<AssetServer>) {
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
                    ..Default::default()
                },
                ..Default::default()
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
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MetersPerSecondText);
    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(80.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
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
                ..Default::default()
            },
            ..Default::default()
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
                    ..Default::default()
                },
                ..Default::default()
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
                        value: "angvel".to_string(),
                        style: TextStyle {
                            font: bold.clone(),
                            font_size: 40.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(RotPerSecondText);

    commands
        .spawn_bundle(TextBundle {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                position_type: PositionType::Absolute,
                position: Rect {
                    bottom: Val::Px(160.0),
                    right: Val::Px(15.0),
                    ..Default::default()
                },
                ..Default::default()
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
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(MassText);
}

pub fn dash_speed_update_system(
    mut texts: QuerySet<(
        QueryState<&mut Text, With<MetersPerSecondText>>,
        QueryState<&mut Text, With<KilometersPerHourText>>,
        QueryState<&mut Text, With<MassText>>,
        QueryState<&mut Text, With<RotPerSecondText>>,
    )>,
    mut cars: Query<(
        &RigidBodyVelocityComponent,
        &RigidBodyMassPropsComponent,
        With<Car>,
    )>,
    mut wheels: Query<(&RigidBodyVelocityComponent, With<Wheel>)>,
) {
    let (velocity, mass, _) = cars.single_mut();
    let mps = velocity.linvel.norm();
    texts.q0().single_mut().sections[0].value = format!("{:.1}", mps);

    let kmph = mps * 3.6;
    texts.q1().single_mut().sections[0].value = format!("{:.1}", kmph);

    let mass_p: MassProperties = mass.local_mprops;
    texts.q2().single_mut().sections[0].value = format!("{}", 1.0 / mass_p.inv_mass,);

    let mut msg: String = "".to_string();
    for (v, _wheel) in wheels.iter_mut() {
        let s = format!("{:.1} ", v.angvel.norm());
        msg = msg + &s;
    }

    texts.q3().single_mut().sections[0].value = msg;
}
