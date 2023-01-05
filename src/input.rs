use crate::{car::*, config::*, font::FontHandle, gamepad::GamepadLobby};
use bevy::prelude::*;

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
                .with_text_alignment(TextAlignment::CENTER),
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
pub fn keyboard_input_system(
    input: Res<Input<KeyCode>>,
    mut config: ResMut<Config>,
    mut cars: Query<(&mut Car, &Transform, With<HID>)>,
    mut commands: Commands,
    q_car: Query<Entity, With<Car>>,
    q_wheel: Query<Entity, With<Wheel>>,
    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "ios")))] mut debug_ctx: ResMut<
        bevy_rapier3d::render::DebugRenderContext,
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
    #[cfg(all(not(target_arch = "wasm32"), not(target_os = "ios")))]
    if input.just_pressed(KeyCode::R) {
        debug_ctx.enabled = !debug_ctx.enabled;
        config.show_rays = debug_ctx.enabled;
    }
    for (mut car, _transform, _hid) in cars.iter_mut() {
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

pub fn gamepad_input_system(
    buttons: Res<Input<GamepadButton>>,
    axes: Res<Axis<GamepadAxis>>,
    lobby: Res<GamepadLobby>,
    mut cars: Query<(&mut Car, &Transform, With<HID>)>,
) {
    for (mut car, _transform, _hid) in cars.iter_mut() {
        for gamepad in lobby.gamepads.iter().cloned() {
            if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
                car.gas = 1.;
            } else if buttons.just_released(GamepadButton::new(gamepad, GamepadButtonType::South)) {
                car.gas = 0.;
            }
            if buttons.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::North)) {
                car.brake = 1.;
            } else if buttons.just_released(GamepadButton::new(gamepad, GamepadButtonType::North)) {
                car.brake = 0.;
            }
            let left_stick_x = axes
                .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
                .unwrap();
            // dbg!(left_stick_x);
            car.steering = left_stick_x;
        }
    }
}
