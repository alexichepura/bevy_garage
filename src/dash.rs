use bevy::{
    color::palettes::css,
    diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin},
    prelude::*,
};
use bevy_garage_car::Player;
use bevy_garage_track::CarTrack;
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

pub fn dash_fps_system(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.0 = format!("{:.0}fps", average);
            }
        }
    }
}

pub fn dash_start_system(mut cmd: Commands, asset_server: Res<AssetServer>) {
    let medium: Handle<Font> = asset_server.load("fonts/FiraMono-Medium.ttf");
    let height = Val::Px(90.);
    let width = Val::Px(150.);

    cmd.spawn((Node {
        width: Val::Percent(100.),
        height: height.clone(),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    },))
        .with_children(|parent| {
            let background_color: BackgroundColor = Color::srgba(0.15, 0.15, 0.15, 0.5).into();
            parent
                .spawn((
                    Node {
                        width,
                        height: height.clone(),
                        padding: UiRect::all(Val::Px(4.0)),
                        justify_content: JustifyContent::End,
                        align_items: AlignItems::End,
                        flex_direction: FlexDirection::Column,
                        ..default()
                    },
                    background_color,
                ))
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("".to_string()),
                        TextFont {
                            font: medium.clone(),
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(css::YELLOW_GREEN.into()),
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(4.),
                            left: Val::Px(4.),
                            ..default()
                        },
                        FpsText,
                    ));
                    parent.spawn((
                        Text::new("".to_string()),
                        TextFont {
                            font: medium.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(css::SALMON.into()),
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(20.),
                            left: Val::Px(4.),
                            ..default()
                        },
                        MpsText,
                    ));
                    parent.spawn((
                        Text::new("".to_string()),
                        TextFont {
                            font: medium.clone(),
                            font_size: 16.0,
                            ..default()
                        },
                        TextColor(css::YELLOW.into()),
                        Node {
                            position_type: PositionType::Absolute,
                            top: Val::Px(4.),
                            ..default()
                        },
                        KmphText,
                    ));
                    parent.spawn((
                        Text::new("".to_string()),
                        TextFont {
                            font: medium.clone(),
                            font_size: 18.0,
                            ..default()
                        },
                        TextColor(css::YELLOW.into()),
                        LapText,
                    ));
                    parent.spawn((
                        Text::new("".to_string()),
                        TextFont {
                            font: medium.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(css::YELLOW_GREEN.into()),
                        TrackPositionText,
                    ));
                    parent.spawn((
                        Text::new("".to_string()),
                        TextFont {
                            font: medium.clone(),
                            font_size: 24.0,
                            ..default()
                        },
                        TextColor(css::YELLOW.into()),
                        RideDistanceText,
                    ));
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
    mut cars: Query<(&Velocity, &CarTrack), With<Player>>,
) {
    for (velocity, car_track) in cars.iter_mut() {
        let mps = velocity.linvel.length();
        let kmph = mps * 3.6;
        texts.p0().single_mut().0 = format!("{:.1}m/s", mps);
        texts.p1().single_mut().0 = format!("{:.1}km/h", kmph);

        texts.p2().single_mut().0 = format!("{:.1}m", car_track.track_position);

        let sign: &str = if car_track.ride_distance.is_sign_negative() {
            "-"
        } else {
            "+"
        };
        texts.p3().single_mut().0 = format!("{sign}{:.1}m", car_track.ride_distance.abs());

        texts.p4().single_mut().0 = format!("lap {}", car_track.lap);
    }
}
