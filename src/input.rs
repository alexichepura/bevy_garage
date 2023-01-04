use crate::{car::*, config::*, font::FontHandle};
use bevy::prelude::*;
// use bevy_rapier3d::render::DebugRenderContext;

enum BtnType {
    U,
    D,
    L,
    R,
}

#[derive(Component)]
pub struct CarButton(BtnType);

pub fn touch_input_start_system(mut commands: Commands, font: Res<FontHandle>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Px(120.0), Val::Px(120.0)),
                position: UiRect {
                    bottom: Val::Px(20.0),
                    left: Val::Px(20.0),
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
                Vec2::new(0., 60.),
                "D",
                BtnType::D,
            );
        });
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Px(120.0), Val::Px(120.0)),
                position: UiRect {
                    bottom: Val::Px(20.0),
                    right: Val::Px(20.0),
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
                Vec2::new(60., 0.),
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
                    size: Size::new(Val::Px(50.0), Val::Px(50.0)),
                    position,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..default()
            },
            CarButton(btn_type),
        ))
        .with_children(|b| {
            b.spawn(
                TextBundle::from_section(
                    str,
                    TextStyle {
                        font,
                        font_size: 30.0,
                        color: Color::BLACK,
                    },
                )
                .with_text_alignment(TextAlignment::CENTER),
            );
        });
}

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
                    *color = Color::BLUE.into();
                    match btn.0 {
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
                    *color = Color::GRAY.into();
                }
                Interaction::None => {
                    *color = Color::WHITE.into();
                    match btn.0 {
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
pub fn keyboard_input_system(
    input: Res<Input<KeyCode>>,
    mut config: ResMut<Config>,
    mut cars: Query<(&mut Car, &Transform, With<HID>)>,
    mut commands: Commands,
    q_car: Query<Entity, With<Car>>,
    q_wheel: Query<Entity, With<Wheel>>,
    // mut debug_ctx: ResMut<DebugRenderContext>,
) {
    if input.just_pressed(KeyCode::N) {
        config.use_brain = !config.use_brain;
    }
    if input.just_pressed(KeyCode::Space) {
        for e in q_wheel.iter() {
            commands.entity(e).despawn_recursive();
        }
        for e in q_car.iter() {
            commands.entity(e).despawn_recursive();
        }
    }
    // if input.just_pressed(KeyCode::R) {
    //     debug_ctx.enabled = !debug_ctx.enabled;
    // }
    for (mut car, _transform, _car) in cars.iter_mut() {
        if input.pressed(KeyCode::Up) {
            car.gas = 1.;
        }
        if input.just_released(KeyCode::Up) {
            car.gas = 0.;
        }

        if input.pressed(KeyCode::Down) {
            car.brake = 1.;
        }
        if input.just_released(KeyCode::Down) {
            car.brake = 0.;
        }

        if input.just_pressed(KeyCode::Left) {
            car.steering = -1.;
        }
        if input.just_pressed(KeyCode::Right) {
            car.steering = 1.;
        }
        if input.just_released(KeyCode::Left) {
            car.steering = 0.;
        }
        if input.just_released(KeyCode::Right) {
            car.steering = 0.;
        }
    }
}
