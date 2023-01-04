use crate::{car::*, config::*};
use bevy::prelude::*;
// use bevy_rapier3d::render::DebugRenderContext;

pub fn touch_input_start_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
    commands
        .spawn(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                size: Size::new(Val::Percent(100.0), Val::Px(100.0)),
                position: UiRect {
                    bottom: Val::Px(100.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        })
        .with_children(|commands| {
            spawn_button(commands, font.clone(), Vec2::new(10., 0.), "U");
            spawn_button(commands, font.clone(), Vec2::new(30., 0.), "D");
            spawn_button(commands, font.clone(), Vec2::new(70., 0.), "L");
            spawn_button(commands, font.clone(), Vec2::new(90., 0.), "R");
        });
}

fn spawn_button(commands: &mut ChildBuilder, font: Handle<Font>, position: Vec2, str: &str) {
    let position = UiRect {
        left: Val::Percent(position.x),
        top: Val::Percent(position.y),
        ..default()
    };
    commands
        .spawn((ButtonBundle {
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
        },))
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

pub fn keyboard_input_system(
    input: Res<Input<KeyCode>>,
    mut config: ResMut<Config>,
    mut cars: Query<(&mut Car, &Transform, With<HID>)>,
    mut commands: Commands,
    q_car: Query<Entity, With<Car>>,
    q_wheel: Query<Entity, With<Wheel>>,
    // mut debug_ctx: ResMut<DebugRenderContext>,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
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
        for (interaction, mut color) in &mut interaction_query {
            match *interaction {
                Interaction::Clicked => {
                    *color = Color::BLUE.into();
                    car.gas = 1.;
                }
                Interaction::Hovered => {
                    *color = Color::GRAY.into();
                }
                Interaction::None => {
                    *color = Color::WHITE.into();
                    car.gas = 0.;
                }
            }
        }

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
