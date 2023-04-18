use bevy::prelude::*;
use virtual_joystick::*;

use crate::{
    car::{Car, HID},
    CarSimLabel,
};

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
    .insert(BackgroundColor(Color::ORANGE_RED.with_a(0.1)))
    .insert(VirtualJoystickInteractionArea);
}

fn update_joystick(
    mut virtual_joystick_events: EventReader<VirtualJoystickEvent<JoystickTypeAxis>>,
    mut cars: Query<&mut Car, With<HID>>,
) {
    for mut car in cars.iter_mut() {
        for j in virtual_joystick_events.iter() {
            let Vec2 { x, y } = j.axis();
            match j.id() {
                JoystickTypeAxis::X => {
                    car.steering = x;
                }
                JoystickTypeAxis::Y => {
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
    }
}
