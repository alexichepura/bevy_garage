use bevy::prelude::*;
use virtual_joystick::*;

use crate::{
    car::{Car, HID},
    CarSimLabel,
};

#[derive(Component, Default)]
pub struct JoystickPrevValue {
    prev: f32,
    axis: JoystickTypeAxis,
}

#[derive(Default, Reflect, Hash, Clone, PartialEq, Eq)]
enum JoystickTypeAxis {
    #[default]
    X,
    Y,
}

pub struct CarJoystickPlugin;
impl Plugin for CarJoystickPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(VirtualJoystickPlugin::<JoystickTypeAxis>::default())
            .add_startup_system(joystick_start_system)
            .add_system(update_joystick.in_set(CarSimLabel::Input));
    }
}

const MARGIN: Val = Val::Px(35.);
pub fn joystick_start_system(mut cmd: Commands, asset_server: Res<AssetServer>) {
    cmd.spawn(
        VirtualJoystickBundle::new(VirtualJoystickNode {
            border_image: asset_server.load("joystick/Horizontal_Outline_Arrows.png"),
            knob_image: asset_server.load("joystick/Outline.png"),
            knob_size: Vec2::new(80., 80.),
            dead_zone: 0.,
            id: JoystickTypeAxis::X,
            axis: VirtualJoystickAxis::Horizontal,
            behaviour: VirtualJoystickType::Fixed,
        })
        .set_color(TintColor(Color::WHITE))
        .set_style(Style {
            size: Size::all(Val::Px(150.)),
            position_type: PositionType::Absolute,
            position: UiRect {
                left: MARGIN,
                bottom: MARGIN,
                ..default()
            },
            ..default()
        }),
    )
    .insert(JoystickPrevValue {
        prev: 0.,
        axis: JoystickTypeAxis::X,
    })
    .insert(BackgroundColor(Color::ORANGE_RED.with_a(0.1)))
    .insert(VirtualJoystickInteractionArea);

    cmd.spawn(
        VirtualJoystickBundle::new(VirtualJoystickNode {
            border_image: asset_server.load("joystick/Vertical_Outline_Arrows.png"),
            knob_image: asset_server.load("joystick/Outline.png"),
            knob_size: Vec2::new(80., 80.),
            dead_zone: 0.,
            id: JoystickTypeAxis::Y,
            axis: VirtualJoystickAxis::Vertical,
            behaviour: VirtualJoystickType::Fixed,
        })
        .set_color(TintColor(Color::WHITE))
        .set_style(Style {
            size: Size::all(Val::Px(150.)),
            position_type: PositionType::Absolute,
            position: UiRect {
                right: MARGIN,
                bottom: MARGIN,
                ..default()
            },
            ..default()
        }),
    )
    .insert(JoystickPrevValue {
        prev: 0.,
        axis: JoystickTypeAxis::Y,
    })
    .insert(BackgroundColor(Color::ORANGE_RED.with_a(0.1)))
    .insert(VirtualJoystickInteractionArea);
}

fn update_joystick(
    mut virtual_joystick_events: EventReader<VirtualJoystickEvent<JoystickTypeAxis>>,
    mut cars: Query<&mut Car, With<HID>>,
    mut joysticks_prev: Query<&mut JoystickPrevValue>,
) {
    for mut car in cars.iter_mut() {
        let mut last_x = 0.;
        let mut last_y = 0.;
        for j in virtual_joystick_events.iter() {
            let Vec2 { x, y } = j.axis();
            match j.id() {
                JoystickTypeAxis::X => {
                    last_x = x;
                    car.steering = x;
                }
                JoystickTypeAxis::Y => {
                    last_y = y;
                    if y < 0. {
                        car.brake = -y / 0.75;
                        car.gas = 0.;
                    } else {
                        car.gas = y / 0.75;
                        car.brake = 0.;
                    }
                }
            }
        }
        for mut j_prev in joysticks_prev.iter_mut() {
            // workaround to reset action
            match j_prev.axis {
                JoystickTypeAxis::X => {
                    if last_x == 0. && j_prev.prev != 0. {
                        car.steering = 0.;
                    } else {
                        j_prev.prev = last_x;
                    }
                }
                JoystickTypeAxis::Y => {
                    if last_y == 0. && j_prev.prev != 0. {
                        car.gas = 0.;
                        car.brake = 0.;
                    } else {
                        j_prev.prev = last_y;
                    }
                }
            }
        }
    }
}
