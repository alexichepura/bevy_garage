use bevy::prelude::*;
use bevy_garage::{
    car::{Car, HID},
    font::FontHandle,
};

enum BtnType {
    U,
    D,
    L,
    R,
}

#[derive(Component)]
pub struct CarButton {
    btn_type: BtnType,
}

pub fn touch_input_start_system(mut commands: Commands, font: Res<FontHandle>) {
    let margin: f32 = 30.;
    let size: f32 = 60.;
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Px(size), Val::Px(size * 2. + margin)),
                position: UiRect {
                    bottom: Val::Px(margin),
                    left: Val::Px(margin),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|commands| {
            spawn_button(
                commands,
                font.bold.clone(),
                Vec2::new(0., 0.),
                "U",
                BtnType::U,
            );
            spawn_button(
                commands,
                font.bold.clone(),
                Vec2::new(0., size),
                "D",
                BtnType::D,
            );
        });
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Px(size * 2. + margin), Val::Px(size)),
                position: UiRect {
                    bottom: Val::Px(margin),
                    right: Val::Px(margin),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|commands| {
            spawn_button(
                commands,
                font.bold.clone(),
                Vec2::new(0., 0.),
                "L",
                BtnType::L,
            );
            spawn_button(
                commands,
                font.bold.clone(),
                Vec2::new(size, 0.),
                "R",
                BtnType::R,
            );
        });
}

fn spawn_button(
    commands: &mut ChildBuilder,
    font: Handle<Font>,
    position: Vec2,
    str: &str,
    btn_type: BtnType,
) {
    let size: f32 = 60.;
    let position = UiRect {
        left: Val::Percent(position.x),
        top: Val::Percent(position.y),
        ..default()
    };
    commands
        .spawn((
            ButtonBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    size: Size::new(Val::Px(size), Val::Px(size)),
                    position,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: BTN_WHITE,
                ..default()
            },
            CarButton { btn_type },
        ))
        .with_children(|b| {
            b.spawn(
                TextBundle::from_section(
                    str,
                    TextStyle {
                        font,
                        font_size: 30.0,
                        color: Color::rgba(0., 0., 0., 0.7),
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );
        });
}

const BTN_WHITE: BackgroundColor = BackgroundColor(Color::rgba(1., 1., 1., 0.5));
const BTN_GRAY: BackgroundColor = BackgroundColor(Color::rgba(0.5, 0.5, 0.5, 0.5));
const BTN_BLUE: BackgroundColor = BackgroundColor(Color::rgba(0., 0., 1., 0.5));
pub fn touch_input_system(
    mut cars: Query<(&mut Car, &Transform, With<HID>)>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &CarButton),
        (Changed<Interaction>, With<CarButton>),
    >,
) {
    for (mut car, _transform, _car) in cars.iter_mut() {
        for (interaction, mut color, btn) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    *color = BTN_BLUE;
                    match btn.btn_type {
                        BtnType::U => {
                            car.gas = 1.;
                        }
                        BtnType::D => {
                            car.brake = 1.;
                        }
                        BtnType::L => {
                            car.steering = -1.;
                        }
                        BtnType::R => {
                            car.steering = 1.;
                        }
                    }
                }
                Interaction::Hovered => {
                    *color = BTN_GRAY;
                }
                Interaction::None => {
                    *color = BTN_WHITE;
                    match btn.btn_type {
                        BtnType::U => {
                            car.gas = 0.;
                        }
                        BtnType::D => {
                            car.brake = 0.;
                        }
                        BtnType::L => {
                            car.steering = 0.;
                        }
                        BtnType::R => {
                            car.steering = 0.;
                        }
                    }
                }
            }
        }
    }
}
