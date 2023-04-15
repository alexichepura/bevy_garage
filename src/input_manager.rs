use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use crate::{car::HID, font::FontHandle};

pub struct CarInputManagerPlugin;

impl Plugin for CarInputManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<CarAction>::default())
            .add_system(screen_input_spawn_system);
    }
}

#[derive(Actionlike, Component, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum CarAction {
    Left,
    Right,
    Gas,
    Brake,
}

pub fn screen_input_spawn_system(
    mut commands: Commands,
    font: Res<FontHandle>,
    cars: Query<Entity, Added<HID>>,
) {
    for car_e in &cars {
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
                    CarAction::Gas,
                    car_e,
                );
                spawn_button(
                    commands,
                    font.bold.clone(),
                    Vec2::new(0., size),
                    "D",
                    CarAction::Brake,
                    car_e,
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
                    CarAction::Left,
                    car_e,
                );
                spawn_button(
                    commands,
                    font.bold.clone(),
                    Vec2::new(size, 0.),
                    "R",
                    CarAction::Right,
                    car_e,
                );
            });
    }
}

fn spawn_button(
    commands: &mut ChildBuilder,
    font: Handle<Font>,
    position: Vec2,
    str: &str,
    action: CarAction,
    car_entity: Entity,
) {
    let size: f32 = 60.;
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
                size: Size::new(Val::Px(size), Val::Px(size)),
                position,
                position_type: PositionType::Absolute,
                ..default()
            },
            background_color: BTN_WHITE,
            ..default()
        },))
        .insert(ActionStateDriver {
            action,
            entity: car_entity,
        })
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
// const BTN_GRAY: BackgroundColor = BackgroundColor(Color::rgba(0.5, 0.5, 0.5, 0.5));
// const BTN_BLUE: BackgroundColor = BackgroundColor(Color::rgba(0., 0., 1., 0.5));
